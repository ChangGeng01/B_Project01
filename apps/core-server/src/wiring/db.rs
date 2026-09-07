//! 数据库一侧的装配（阶段 2 集成 B）。
//!
//! 纪律两条：
//! 一、连接预算在 configuring 阶段求和校验，违例以退出码 78 拒启，
//!    不带病运行（02 计划 §6 与基线 11.6）；
//! 二、机密解析失败或建池失败时不注入——四项 SQL 自检与
//!    `secrets-resolvable` 第一段如实报未覆盖或失败，绝不以空实现顶位
//!    （unwired-absent，archcheck 断言本目录不出现 Noop/Stub/Fake/Dummy）。
//!
//! KmsSecretProvider 尚未交付时绝不回退明文文件；历史路径 reader 只在
//! 显式 `legacy-file` 的 development/test debug 构建存在。

use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::budget::BudgetViolation;
use ep_adapter_db_pg::{
    ConnectionBudget, DbMetrics, PgDataFoundationCheck, PgDegradationLedger, PgIdempotencyStore,
    PgKeyDomainStore, PgLegalEntityDirectory, PgMigrationWindowGuard, PgMigrationWindowStore,
    PgPools, PgSensitiveFieldRegistry, PgUnitOfWork, PoolBuildCfg, PoolCredential, PoolKind,
    PoolOwner, PoolSpec, PoolTimeouts, RetryPolicy, RoResourceLimits,
};
use ep_foundation::port::db::{IdempotencyStore, MigrationWindowGuard};
use ep_platform_obs::{DegradationLedger, MetricsRegistry};
#[cfg(all(feature = "legacy-file", debug_assertions))]
use ep_platform_runtime::config::resolve_legacy_file_secret;
use ep_platform_runtime::config::{
    DbCfg, KmsCfg, PlatformCfg, SecretRef, SecretString, SecretsCfg,
};
use ep_platform_runtime::selfcheck::{SecretsProbe, SqlProbe};

use super::metrics::ObsDbMetrics;
use super::probes::{CoreSecretsProbe, FoundationProbeAdapter};

/// core-server 的进程名，落 `application_name` 的 `<process>` 段。
pub const PROCESS_NAME: &str = "core-server";

/// 装配产物：core-server 唯一持有的 Rw/Ro 两池、两个工作单元、
/// 七个存取层、窗口守卫与降级台账。Worker 池只由 job-worker 持有。
/// Ops 池只由 ops-agent 以 `ep_ops_ro` 持有，不借给本写进程。
/// 机密解析或建池失败时为 None，端点与自检按能力缺位处置。
/// 池与工作单元字段是装配事实的持有点（供后续阶段端点与池指标
/// 观察），并非占位实现，dead_code 容忍在此是显式决定。
#[allow(dead_code)]
pub struct DbAssembly {
    pub pools: PgPools,
    pub uow_rw: Arc<PgUnitOfWork>,
    pub uow_ro: Arc<PgUnitOfWork>,
    pub key_domains: Arc<PgKeyDomainStore>,
    pub windows: Arc<PgMigrationWindowStore>,
    /// B-03 迁移窗口守卫（E-17 注入点）：A-09 开窗后的开窗校验路径
    /// 与后续 concurrent/DDL 执行路径在同一事务内调 `assert_open` 取用，
    /// 本阶段只装配不改端点判定语义。
    pub window_guard: Arc<dyn MigrationWindowGuard>,
    /// 阶段 3a 幂等键存储（表 12）：端点的写请求在业务事务内
    /// 经 `try_begin`/`finish` 去重与回放，保留天数取自
    /// `platform.idempotency.retention_days`。
    pub idempotency_store: Arc<dyn IdempotencyStore>,
    pub sensitive_fields: Arc<PgSensitiveFieldRegistry>,
    pub legal_entities: Arc<PgLegalEntityDirectory>,
    pub ledger: Arc<PgDegradationLedger>,
    pub foundation_check: Arc<PgDataFoundationCheck>,
}

impl DbAssembly {
    /// 四项 SQL 自检的探针。装配成功即 Some，自检随即产生实质判定。
    pub fn sql_probe(&self) -> Option<Arc<dyn SqlProbe>> {
        Some(Arc::new(FoundationProbeAdapter::new(
            self.foundation_check.clone(),
        )))
    }

    /// `secrets-resolvable` 的两段探针。
    pub fn secrets_probe(
        &self,
        secrets: &SecretsCfg,
        db: &DbCfg,
        kms: &KmsCfg,
    ) -> Option<Arc<dyn SecretsProbe>> {
        Some(Arc::new(CoreSecretsProbe::new(
            secrets.clone(),
            vec![db.password_ref.clone(), db.ro_password_ref.clone()],
            kms.clone(),
            self.legal_entities.clone(),
            self.key_domains.clone(),
        )))
    }

    /// 降级台账，供 `secrets-resolvable` 第二段缺域时登记窗口。
    pub fn degradation_ledger(&self) -> Option<Arc<dyn DegradationLedger>> {
        Some(self.ledger.clone())
    }
}

/// 四池规模表，顺序按 [`PoolKind::ALL`] 对齐。
pub fn budget_specs(db: &DbCfg) -> [PoolSpec; 4] {
    [
        PoolSpec {
            kind: PoolKind::Rw,
            max_connections: db.pool.rw_max,
        },
        PoolSpec {
            kind: PoolKind::Ro,
            max_connections: db.pool.ro_max,
        },
        PoolSpec {
            kind: PoolKind::Worker,
            max_connections: db.pool.worker_max,
        },
        PoolSpec {
            kind: PoolKind::Ops,
            max_connections: db.pool.ops_max,
        },
    ]
}

/// 启动预算求和校验（裁定 C-04）。违例逐条返回，由 main 映射为退出码 78。
pub fn budget_check(db: &DbCfg) -> Result<(), Vec<BudgetViolation>> {
    ConnectionBudget::from_specs(
        db.budget.resident_max,
        db.budget.temporary_max,
        db.budget.peak_max,
        &budget_specs(db),
    )
    .validate()
}

/// KmsSecretProvider 尚未交付时的失败关闭边界。`kms` 绝不回退文件；
/// 历史明文 reader 只存在于显式 `legacy-file` 的 debug/test 构建。
pub fn resolve_secret(secrets: &SecretsCfg, reference: &SecretRef) -> Result<SecretString, String> {
    #[cfg(not(feature = "legacy-file"))]
    {
        let _ = (&secrets.provider, reference);
        Err("NOT_IMPLEMENTED：KmsSecretProvider 尚未交付，禁止回退明文文件".into())
    }
    #[cfg(feature = "legacy-file")]
    {
        if matches!(
            secrets.provider,
            ep_platform_runtime::config::SecretsProvider::Kms
        ) {
            Err("NOT_IMPLEMENTED：KmsSecretProvider 尚未交付，禁止回退明文文件".into())
        } else {
            #[cfg(not(debug_assertions))]
            {
                let _ = reference;
                Err(
                    "生产/默认构建禁止 file provider；仅 development/test 的 legacy-file debug 构建可用"
                        .into(),
                )
            }
            #[cfg(debug_assertions)]
            {
                resolve_legacy_file_secret(&secrets.dir, reference)
                    .map_err(|error| error.to_string())
            }
        }
    }
}

/// 配置段转建池取值。机密已解引用为明文口令，生命周期仅限本装配过程。
fn pool_build_cfg(
    db: &DbCfg,
    rw_password: SecretString,
    ro_password: SecretString,
) -> PoolBuildCfg {
    let to = |t: ep_platform_runtime::config::PoolTimeoutCfg| PoolTimeouts {
        statement_ms: t.statement_ms,
        lock_ms: t.lock_ms,
        idle_in_tx_ms: t.idle_in_tx_ms,
    };
    PoolBuildCfg {
        host: db.host.clone(),
        port: db.port,
        database: db.database.clone(),
        credentials: [
            Some(PoolCredential {
                user: db.user.clone(),
                password: rw_password,
            }),
            Some(PoolCredential {
                user: db.ro_user.clone(),
                password: ro_password,
            }),
            None,
            None,
        ],
        specs: budget_specs(db),
        acquire_timeout: Duration::from_millis(u64::from(db.pool.acquire_timeout_ms)),
        max_lifetime: Duration::from_secs(u64::from(db.pool.max_lifetime_s)),
        idle_timeout: Duration::from_secs(u64::from(db.pool.idle_timeout_s)),
        timeouts: [
            to(db.timeout.rw),
            to(db.timeout.ro),
            to(db.timeout.worker),
            to(db.timeout.ops),
        ],
        ro_limits: RoResourceLimits {
            work_mem_kb: db.ro.work_mem_kb,
            temp_file_limit_kb: db.ro.temp_file_limit_kb,
        },
        process_name: PROCESS_NAME,
    }
}

/// 完整装配。任一步失败返回 None 与原因文本：不注入，不带病运行。
pub fn build(
    db: &DbCfg,
    secrets: &SecretsCfg,
    platform: &PlatformCfg,
    registry: Arc<MetricsRegistry>,
) -> Result<DbAssembly, String> {
    if db.user != "ep_app_rw" || db.ro_user != "ep_analyst_ro" {
        return Err(format!(
            "core-server 数据库角色必须为 rw=ep_app_rw/ro=ep_analyst_ro，当前为 rw={}/ro={}",
            db.user, db.ro_user
        ));
    }
    let rw_password = resolve_secret(secrets, &db.password_ref)?;
    let ro_password = resolve_secret(secrets, &db.ro_password_ref)?;
    let metrics: Arc<dyn DbMetrics> = Arc::new(ObsDbMetrics::new(registry.clone()));
    let pools = PgPools::build(
        &pool_build_cfg(db, rw_password, ro_password),
        PoolOwner::CoreServer,
        metrics.clone(),
    )
    .map_err(|e| format!("建池失败：{e}"))?;
    let policy = RetryPolicy::try_from_config(db.retry.max_attempts, &db.retry.backoff_ms)
        .map_err(str::to_string)?;

    let mk = |kind: PoolKind| -> Result<Arc<PgUnitOfWork>, String> {
        let pool = pools
            .pool(kind)
            .ok_or_else(|| format!("池 {} 缺失", kind.label()))?;
        Ok(Arc::new(PgUnitOfWork::with_pool(
            pool.clone(),
            kind,
            policy.clone(),
            metrics.clone(),
        )))
    };
    let uow_rw = mk(PoolKind::Rw)?;
    let uow_ro = mk(PoolKind::Ro)?;
    Ok(DbAssembly {
        pools,
        key_domains: Arc::new(PgKeyDomainStore::new(uow_rw.clone())),
        windows: Arc::new(PgMigrationWindowStore::new(uow_rw.clone())),
        window_guard: Arc::new(PgMigrationWindowGuard),
        idempotency_store: Arc::new(PgIdempotencyStore::new(platform.idempotency.retention_days)),
        sensitive_fields: Arc::new(PgSensitiveFieldRegistry::new(uow_rw.clone())),
        legal_entities: Arc::new(PgLegalEntityDirectory::new(uow_ro.clone())),
        ledger: Arc::new(PgDegradationLedger::new(uow_rw.clone(), registry)),
        foundation_check: Arc::new(PgDataFoundationCheck::new(uow_ro.clone())),
        uow_rw,
        uow_ro,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // 负样例断言的是预算这条规则本身：当前四池合计超过常驻上限必须拦下。
    #[test]
    fn a_budget_overflow_is_rejected_before_any_pool_is_built() {
        let mut db = DbCfg::default();
        db.budget.resident_max = 36;
        let errs = budget_check(&db).expect_err("37 超 36 必须违例");
        assert!(
            errs.iter()
                .any(|e| matches!(e, BudgetViolation::ResidentOverflow { sum: 37, limit: 36 })),
            "{errs:?}"
        );
    }

    #[test]
    fn the_standard_budget_passes() {
        assert!(budget_check(&DbCfg::default()).is_ok());
    }

    #[test]
    fn rw_and_ro_use_distinct_database_roles_and_secrets() {
        let cfg = pool_build_cfg(
            &DbCfg::default(),
            SecretString::new("rw-secret"),
            SecretString::new("ro-secret"),
        );
        let rw = cfg.credentials[0].as_ref().expect("Rw 凭据");
        let ro = cfg.credentials[1].as_ref().expect("Ro 凭据");
        assert_eq!(rw.user, "ep_app_rw");
        assert_eq!(ro.user, "ep_analyst_ro");
        assert_ne!(rw.password.expose(), ro.password.expose());
        assert!(cfg.credentials[2].is_none());
        assert!(cfg.credentials[3].is_none());
    }

    #[test]
    #[cfg(all(feature = "legacy-file", debug_assertions))]
    fn explicit_development_legacy_file_provider_reads_a_nonempty_secret() {
        let dir = std::env::temp_dir().join("ep-core-server-wiring-test");
        std::fs::create_dir_all(dir.join("db")).unwrap();
        std::fs::write(dir.join("db/app_rw#1"), "s3cret\n").unwrap();
        let secrets = SecretsCfg {
            dir: dir.clone(),
            provider: ep_platform_runtime::config::SecretsProvider::File,
        };
        let got = resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap());
        let got = got.unwrap_or_else(|error| panic!("受控机密应解析成功：{error}"));
        assert_eq!(got.expose(), "s3cret");
        let missing = resolve_secret(&secrets, &SecretRef::parse("secret://db/absent#9").unwrap());
        assert!(missing.is_err(), "文件缺失必须失败，不得回落空口令");
        std::fs::write(dir.join("db/empty#1"), "  \n").unwrap();
        let empty = resolve_secret(&secrets, &SecretRef::parse("secret://db/empty#1").unwrap());
        assert!(empty.is_err(), "空机密不得进入建池路径");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn kms_provider_is_not_implemented_and_never_falls_back_to_file() {
        let secrets = SecretsCfg::default();
        let err = match resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap())
        {
            Err(error) => error,
            Ok(_) => panic!("KMS 未实现必须拒启"),
        };
        assert!(err.contains("NOT_IMPLEMENTED"), "{err}");
    }

    #[cfg(all(feature = "legacy-file", not(debug_assertions)))]
    #[test]
    fn release_shape_has_no_legacy_file_reader() {
        let secrets = SecretsCfg {
            provider: ep_platform_runtime::config::SecretsProvider::File,
            ..SecretsCfg::default()
        };
        let err = match resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap())
        {
            Err(error) => error,
            Ok(_) => panic!("默认构建必须拒绝 file provider"),
        };
        assert!(err.contains("禁止 file provider"), "{err}");
    }
}
