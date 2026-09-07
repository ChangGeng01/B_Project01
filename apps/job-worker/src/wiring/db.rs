//! 数据库一侧的装配（job-worker）。
//!
//! 与 core-server 同纪律：预算违例以退出码 78 拒启；机密解析或建池
//! 失败时不注入，四项 SQL 自检如实报未覆盖，绝不以空实现顶位。
//! 本进程只持有 Worker 池；降级台账与自检也复用该池。
//! Rw/Ro 只由 core-server 持有，Ops 只由 ops-agent 持有；
//! 全机四池合计 37，不在每个进程各复制一遍。

use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::budget::BudgetViolation;
use ep_adapter_db_pg::{
    ConnectionBudget, DbMetrics, PgDataFoundationCheck, PgDegradationLedger, PgIdempotencyStore,
    PgLegalEntityDirectory, PgMigrationWindowGuard, PgPools, PgUnitOfWork, PoolBuildCfg,
    PoolCredential, PoolKind, PoolOwner, PoolSpec, PoolTimeouts, RetryPolicy, RoResourceLimits,
};
use ep_foundation::port::db::{IdempotencyStore, MigrationWindowGuard};
use ep_platform_obs::MetricsRegistry;
#[cfg(all(feature = "legacy-file", debug_assertions))]
use ep_platform_runtime::config::resolve_legacy_file_secret;
use ep_platform_runtime::config::{DbCfg, PlatformCfg, SecretRef, SecretString, SecretsCfg};
use ep_platform_runtime::selfcheck::probe::SqlProbe;

use super::metrics::ObsDbMetrics;
use super::probes::FoundationProbeAdapter;

/// job-worker 的进程名，落 `application_name` 的 `<process>` 段。
pub const PROCESS_NAME: &str = "job-worker";

/// 装配产物：job-worker 唯一持有的 Worker 池/工作单元、窗口守卫、台账与自检取数。
#[allow(dead_code)]
pub struct WorkerDbAssembly {
    pub pools: PgPools,
    pub uow_worker: Arc<PgUnitOfWork>,
    /// B-03 迁移窗口守卫（E-17 注入点）：阶段 13b 的在线 DDL 由本进程
    /// 的 DDL 执行器发起，在把控制交给 ep-platform-release 的编排之前
    /// 调用注入实例的 `assert_open(tx)`；本阶段只装配不接入执行路径。
    pub window_guard: Arc<dyn MigrationWindowGuard>,
    /// 阶段 3a 幂等键存储（表 12）：本进程的消费侧幂等与发布
    /// 执行事务内的去重回放经它执行，保留天数取自
    /// `platform.idempotency.retention_days`。
    pub idempotency_store: Arc<dyn IdempotencyStore>,
    /// 法人目录（阶段 4 任务 #21）：后台任务逐法人枚举系统上下文
    /// 的取数面（过期会话/挑战清理与应急维护）。
    pub legal_entities: Arc<PgLegalEntityDirectory>,
    pub ledger: Arc<PgDegradationLedger>,
    pub foundation_check: Arc<PgDataFoundationCheck>,
}

impl WorkerDbAssembly {
    /// 四项 SQL 自检的探针。装配成功即 Some，自检随即产生实质判定。
    pub fn sql_probe(&self) -> Option<Arc<dyn SqlProbe>> {
        Some(Arc::new(FoundationProbeAdapter::new(
            self.foundation_check.clone(),
        )))
    }

    /// 降级台账。`secrets-resolvable` 第二段缺域由 core-server 承担，
    /// 本进程只暴露台账供运行期登记。
    pub fn degradation_ledger(&self) -> Option<Arc<dyn ep_platform_obs::DegradationLedger>> {
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

/// KMS 未交付前默认失败关闭；仅显式 legacy-file debug/test 构建可走共享受控 reader。
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

fn pool_build_cfg(db: &DbCfg, password: SecretString) -> PoolBuildCfg {
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
            None,
            None,
            Some(PoolCredential {
                user: db.user.clone(),
                password,
            }),
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
) -> Result<WorkerDbAssembly, String> {
    if db.user != "ep_app_rw" {
        return Err(format!("job-worker 只允许 ep_app_rw，当前为 {}", db.user));
    }
    let password = resolve_secret(secrets, &db.password_ref)?;
    let metrics: Arc<dyn DbMetrics> = Arc::new(ObsDbMetrics::new(registry.clone()));
    let pools = PgPools::build(
        &pool_build_cfg(db, password),
        PoolOwner::JobWorker,
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
    let uow_worker = mk(PoolKind::Worker)?;

    Ok(WorkerDbAssembly {
        pools,
        window_guard: Arc::new(PgMigrationWindowGuard),
        idempotency_store: Arc::new(PgIdempotencyStore::new(platform.idempotency.retention_days)),
        legal_entities: Arc::new(PgLegalEntityDirectory::new(uow_worker.clone())),
        ledger: Arc::new(PgDegradationLedger::new(uow_worker.clone(), registry)),
        foundation_check: Arc::new(PgDataFoundationCheck::new(uow_worker.clone())),
        uow_worker,
    })
}

// SecretsProbe 的两段判定由 core-server 承担（密钥域只在 core-server
// 装配）；本进程按 unwired-absent 不注入该探针。法人目录自阶段 4
// 任务 #21 起由本进程装配（后台任务的逐法人枚举取数面）。

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
    fn kms_provider_is_not_implemented_and_never_falls_back_to_file() {
        let secrets = SecretsCfg::default();
        let err = match resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap())
        {
            Err(error) => error,
            Ok(_) => panic!("KMS 未实现必须拒启"),
        };
        assert!(err.contains("NOT_IMPLEMENTED"), "{err}");
    }

    #[cfg(all(feature = "legacy-file", debug_assertions))]
    #[test]
    fn explicit_development_legacy_file_provider_reads_a_nonempty_secret() {
        let dir = std::env::temp_dir().join("ep-job-worker-wiring-test");
        std::fs::create_dir_all(dir.join("db")).unwrap();
        std::fs::write(dir.join("db/app_rw#1"), "worker-secret\n").unwrap();
        let secrets = SecretsCfg {
            dir: dir.clone(),
            provider: ep_platform_runtime::config::SecretsProvider::File,
        };
        let got = resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap());
        let got = got.unwrap_or_else(|error| panic!("受控机密应解析成功：{error}"));
        assert_eq!(got.expose(), "worker-secret");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[cfg(all(feature = "legacy-file", not(debug_assertions)))]
    #[test]
    fn release_shape_has_no_legacy_file_reader() {
        let secrets = SecretsCfg {
            provider: ep_platform_runtime::config::SecretsProvider::File,
            ..SecretsCfg::default()
        };
        assert!(
            resolve_secret(&secrets, &SecretRef::parse("secret://db/app_rw#1").unwrap()).is_err()
        );
    }
}
