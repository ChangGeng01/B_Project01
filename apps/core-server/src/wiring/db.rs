//! 数据库一侧的装配（阶段 2 集成 B）。
//!
//! 纪律两条：
//! 一、连接预算在 configuring 阶段求和校验，违例以退出码 78 拒启，
//!    不带病运行（02 计划 §6 与基线 11.6）；
//! 二、机密解析失败或建池失败时不注入——四项 SQL 自检与
//!    `secrets-resolvable` 第一段如实报未覆盖或失败，绝不以空实现顶位
//!    （unwired-absent，archcheck 断言本目录不出现 Noop/Stub/Fake/Dummy）。
//!
//! 机密解引用路径 `<secrets.dir>/<domain>/<name>#<version>`：
//! 配置里只有 `secret://` 引用，落盘形态与 00b 基线第 9 节一致。

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::budget::BudgetViolation;
use ep_adapter_db_pg::{
    ConnectionBudget, DbMetrics, PgDataFoundationCheck, PgDegradationLedger, PgIdempotencyStore,
    PgKeyDomainStore, PgLegalEntityDirectory, PgMigrationWindowGuard, PgMigrationWindowStore,
    PgPools, PgSensitiveFieldRegistry, PgUnitOfWork, PoolBuildCfg, PoolKind, PoolSpec,
    PoolTimeouts, RetryPolicy, RoResourceLimits,
};
use ep_foundation::port::db::{IdempotencyStore, MigrationWindowGuard};
use ep_platform_obs::{DegradationLedger, MetricsRegistry};
use ep_platform_runtime::config::{DbCfg, KmsCfg, PlatformCfg, SecretRef, SecretsCfg};
use ep_platform_runtime::selfcheck::{SecretsProbe, SqlProbe};

use super::metrics::ObsDbMetrics;
use super::probes::{CoreSecretsProbe, FoundationProbeAdapter};

/// core-server 的进程名，落 `application_name` 的 `<process>` 段。
pub const PROCESS_NAME: &str = "core-server";

/// 装配产物：五池、三个工作单元、七个存取层、窗口守卫与降级台账。
/// 机密解析或建池失败时为 None，端点与自检按能力缺位处置。
/// 池与工作单元字段是装配事实的持有点（供后续阶段端点与池指标
/// 观察），并非占位实现，dead_code 容忍在此是显式决定。
#[allow(dead_code)]
pub struct DbAssembly {
    pub pools: PgPools,
    pub uow_rw: Arc<PgUnitOfWork>,
    pub uow_ro: Arc<PgUnitOfWork>,
    pub uow_ops: Arc<PgUnitOfWork>,
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
            secrets.dir.clone(),
            db.password_ref.clone(),
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

/// 五池规模表，顺序按 [`PoolKind::ALL`] 对齐。
pub fn budget_specs(db: &DbCfg) -> [PoolSpec; 5] {
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
            kind: PoolKind::Integ,
            max_connections: db.pool.integ_max,
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
        db.budget.peak_max,
        &budget_specs(db),
    )
    .validate()
}

/// 机密解引用：`secret://<domain>/<name>#<version>` 读
/// `<dir>/<domain>/<name>#<version>` 文件正文。文件缺失即失败。
pub fn resolve_secret(dir: &Path, reference: &SecretRef) -> Result<String, String> {
    let rest = reference
        .as_str()
        .strip_prefix("secret://")
        .ok_or_else(|| format!("机密引用缺前缀：{}", reference.as_str()))?;
    let path = dir.join(rest);
    std::fs::read_to_string(&path)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("机密解引用失败 {}: {e}", path.display()))
}

/// 配置段转建池取值。机密已解引用为明文口令，生命周期仅限本装配过程。
fn pool_build_cfg(db: &DbCfg, password: String) -> PoolBuildCfg {
    let to = |t: ep_platform_runtime::config::PoolTimeoutCfg| PoolTimeouts {
        statement_ms: t.statement_ms,
        lock_ms: t.lock_ms,
        idle_in_tx_ms: t.idle_in_tx_ms,
    };
    PoolBuildCfg {
        host: db.host.clone(),
        port: db.port,
        database: db.database.clone(),
        user: db.user.clone(),
        password,
        specs: budget_specs(db),
        acquire_timeout: Duration::from_millis(u64::from(db.pool.acquire_timeout_ms)),
        max_lifetime: Duration::from_secs(u64::from(db.pool.max_lifetime_s)),
        idle_timeout: Duration::from_secs(u64::from(db.pool.idle_timeout_s)),
        timeouts: [
            to(db.timeout.rw),
            to(db.timeout.ro),
            to(db.timeout.worker),
            to(db.timeout.integ),
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
    ep_adapter_db_pg::register_process_name(PROCESS_NAME);
    let password = resolve_secret(&secrets.dir, &db.password_ref)?;
    let metrics: Arc<dyn DbMetrics> = Arc::new(ObsDbMetrics::new(registry.clone()));
    let pools = PgPools::build(&pool_build_cfg(db, password), metrics.clone())
        .map_err(|e| format!("建池失败：{e}"))?;
    let policy = RetryPolicy::from_config(db.retry.max_attempts, &db.retry.backoff_ms);

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
    let uow_ops = mk(PoolKind::Ops)?;

    Ok(DbAssembly {
        pools,
        key_domains: Arc::new(PgKeyDomainStore::new(uow_rw.clone())),
        windows: Arc::new(PgMigrationWindowStore::new(uow_rw.clone())),
        window_guard: Arc::new(PgMigrationWindowGuard),
        idempotency_store: Arc::new(PgIdempotencyStore::new(platform.idempotency.retention_days)),
        sensitive_fields: Arc::new(PgSensitiveFieldRegistry::new(uow_rw.clone())),
        legal_entities: Arc::new(PgLegalEntityDirectory::new(uow_ro.clone())),
        ledger: Arc::new(PgDegradationLedger::new(uow_ops.clone(), registry)),
        foundation_check: Arc::new(PgDataFoundationCheck::new(uow_ops.clone())),
        uow_rw,
        uow_ro,
        uow_ops,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // 负样例断言的是预算这条规则本身：五池合计超过常驻上限必须拦下。
    #[test]
    fn a_budget_overflow_is_rejected_before_any_pool_is_built() {
        let mut db = DbCfg::default();
        db.budget.resident_max = 41;
        let errs = budget_check(&db).expect_err("42 超 41 必须违例");
        assert!(
            errs.iter()
                .any(|e| matches!(e, BudgetViolation::ResidentOverflow { sum: 42, limit: 41 })),
            "{errs:?}"
        );
    }

    #[test]
    fn the_standard_budget_passes() {
        assert!(budget_check(&DbCfg::default()).is_ok());
    }

    #[test]
    fn secret_reference_maps_to_a_file_under_the_secrets_dir() {
        let dir = std::env::temp_dir().join("ep-core-server-wiring-test");
        std::fs::create_dir_all(dir.join("db")).unwrap();
        std::fs::write(dir.join("db/app_rw#1"), "s3cret\n").unwrap();
        let got = resolve_secret(&dir, &SecretRef::parse("secret://db/app_rw#1").unwrap());
        assert_eq!(got.as_deref(), Ok("s3cret"));
        let missing = resolve_secret(&dir, &SecretRef::parse("secret://db/absent#9").unwrap());
        assert!(missing.is_err(), "文件缺失必须失败，不得回落空口令");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
