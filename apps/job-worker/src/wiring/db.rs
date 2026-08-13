//! 数据库一侧的装配（job-worker）。
//!
//! 与 core-server 同纪律：预算违例以退出码 78 拒启；机密解析或建池
//! 失败时不注入，四项 SQL 自检如实报未覆盖，绝不以空实现顶位。
//! 本进程只消费 Worker 池（任务）与 Ops 池（降级台账与自检取数），
//! 建池仍按五池全量 connect_lazy：惰性建池不产生实际连接，预算求和
//! 口径因此与库侧保持一致。

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::budget::BudgetViolation;
use ep_adapter_db_pg::{
    ConnectionBudget, DbMetrics, PgDataFoundationCheck, PgDegradationLedger, PgIdempotencyStore,
    PgLegalEntityDirectory, PgMigrationWindowGuard, PgPools, PgUnitOfWork, PoolBuildCfg, PoolKind,
    PoolSpec, PoolTimeouts, RetryPolicy, RoResourceLimits,
};
use ep_foundation::port::db::{IdempotencyStore, MigrationWindowGuard};
use ep_platform_obs::MetricsRegistry;
use ep_platform_runtime::config::{DbCfg, PlatformCfg, SecretRef, SecretsCfg};
use ep_platform_runtime::selfcheck::probe::SqlProbe;

use super::metrics::ObsDbMetrics;
use super::probes::FoundationProbeAdapter;

/// job-worker 的进程名，落 `application_name` 的 `<process>` 段。
pub const PROCESS_NAME: &str = "job-worker";

/// 装配产物：五池（惰性）、Worker 与 Ops 工作单元、窗口守卫、台账与自检取数。
#[allow(dead_code)]
pub struct WorkerDbAssembly {
    pub pools: PgPools,
    pub uow_worker: Arc<PgUnitOfWork>,
    pub uow_ops: Arc<PgUnitOfWork>,
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
) -> Result<WorkerDbAssembly, String> {
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
    let uow_worker = mk(PoolKind::Worker)?;
    let uow_ops = mk(PoolKind::Ops)?;

    Ok(WorkerDbAssembly {
        pools,
        window_guard: Arc::new(PgMigrationWindowGuard),
        idempotency_store: Arc::new(PgIdempotencyStore::new(platform.idempotency.retention_days)),
        legal_entities: Arc::new(PgLegalEntityDirectory::new(uow_worker.clone())),
        ledger: Arc::new(PgDegradationLedger::new(uow_ops.clone(), registry)),
        foundation_check: Arc::new(PgDataFoundationCheck::new(uow_ops.clone())),
        uow_worker,
        uow_ops,
    })
}

// SecretsProbe 的两段判定由 core-server 承担（密钥域只在 core-server
// 装配）；本进程按 unwired-absent 不注入该探针。法人目录自阶段 4
// 任务 #21 起由本进程装配（后台任务的逐法人枚举取数面）。

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
}
