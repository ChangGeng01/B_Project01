//! ops-agent 的 Ops2 只读池装配。

use std::sync::Arc;
use std::time::Duration;

use ep_adapter_db_pg::budget::BudgetViolation;
use ep_adapter_db_pg::{
    ConnectionBudget, DbMetrics, PgDataFoundationCheck, PgPools, PgUnitOfWork, PoolBuildCfg,
    PoolCredential, PoolKind, PoolOwner, PoolSpec, PoolTimeouts, RetryPolicy, RoResourceLimits,
};
use ep_platform_obs::MetricsRegistry;
#[cfg(all(feature = "legacy-file", debug_assertions))]
use ep_platform_runtime::config::resolve_legacy_file_secret;
use ep_platform_runtime::config::{DbCfg, SecretRef, SecretString, SecretsCfg};
use ep_platform_runtime::selfcheck::SqlProbe;

use super::metrics::ObsDbMetrics;
use super::probes::FoundationProbeAdapter;

pub const PROCESS_NAME: &str = "ops-agent";

#[allow(dead_code)]
pub struct OpsDbAssembly {
    pub pools: PgPools,
    pub uow_ops: Arc<PgUnitOfWork>,
    pub foundation_check: Arc<PgDataFoundationCheck>,
}

impl OpsDbAssembly {
    pub fn sql_probe(&self) -> Option<Arc<dyn SqlProbe>> {
        Some(Arc::new(FoundationProbeAdapter::new(
            self.foundation_check.clone(),
        )))
    }
}

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

pub fn budget_check(db: &DbCfg) -> Result<(), Vec<BudgetViolation>> {
    ConnectionBudget::from_specs(
        db.budget.resident_max,
        db.budget.temporary_max,
        db.budget.peak_max,
        &budget_specs(db),
    )
    .validate()
}

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
            None,
            Some(PoolCredential {
                user: db.user.clone(),
                password,
            }),
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

pub fn build(
    db: &DbCfg,
    secrets: &SecretsCfg,
    registry: Arc<MetricsRegistry>,
) -> Result<OpsDbAssembly, String> {
    if db.user != "ep_ops_ro" {
        return Err(format!("ops-agent 只允许 ep_ops_ro，当前为 {}", db.user));
    }
    let password = resolve_secret(secrets, &db.password_ref)?;
    let metrics: Arc<dyn DbMetrics> = Arc::new(ObsDbMetrics::new(registry));
    let pools = PgPools::build(
        &pool_build_cfg(db, password),
        PoolOwner::OpsAgent,
        metrics.clone(),
    )
    .map_err(|e| format!("建池失败：{e}"))?;
    let policy = RetryPolicy::try_from_config(db.retry.max_attempts, &db.retry.backoff_ms)
        .map_err(str::to_string)?;
    let pool = pools
        .pool(PoolKind::Ops)
        .ok_or_else(|| "Ops 池缺失".to_string())?;
    let uow_ops = Arc::new(PgUnitOfWork::with_pool(
        pool.clone(),
        PoolKind::Ops,
        policy,
        metrics,
    ));
    Ok(OpsDbAssembly {
        pools,
        foundation_check: Arc::new(PgDataFoundationCheck::new(uow_ops.clone())),
        uow_ops,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLATFORM_CORE_GRANTS: &str = include_str!(
        "../../../../db/migrations/platform_core/V20260901100500__platform_core_grants.sql"
    );

    #[test]
    fn standard_budget_passes_and_overflow_fails_closed() {
        let mut db = DbCfg::default();
        assert!(budget_check(&db).is_ok());
        db.budget.resident_max = 36;
        assert!(budget_check(&db).is_err());
    }

    #[test]
    #[cfg(all(feature = "legacy-file", debug_assertions))]
    fn explicit_development_legacy_file_provider_rejects_empty_secret() {
        let dir = std::env::temp_dir().join("ep-ops-agent-wiring-test");
        std::fs::create_dir_all(dir.join("db")).unwrap();
        std::fs::write(dir.join("db/nonempty#1"), "ops-secret\n").unwrap();
        std::fs::write(dir.join("db/ops_ro#1"), "  \n").unwrap();
        let nonempty_ref = SecretRef::parse("secret://db/nonempty#1").unwrap();
        let reference = SecretRef::parse("secret://db/ops_ro#1").unwrap();
        let secrets = SecretsCfg {
            dir: dir.clone(),
            provider: ep_platform_runtime::config::SecretsProvider::File,
        };
        let value = resolve_secret(&secrets, &nonempty_ref)
            .unwrap_or_else(|error| panic!("受控机密应解析成功：{error}"));
        assert_eq!(value.expose(), "ops-secret");
        assert!(resolve_secret(&secrets, &reference).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn kms_provider_is_not_implemented_and_never_falls_back_to_file() {
        let secrets = SecretsCfg::default();
        let err = match resolve_secret(&secrets, &SecretRef::parse("secret://db/ops_ro#1").unwrap())
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
        assert!(
            resolve_secret(&secrets, &SecretRef::parse("secret://db/ops_ro#1").unwrap()).is_err()
        );
    }

    #[test]
    fn ops_role_gets_only_the_schema_history_read_needed_by_its_probe() {
        let sql = PLATFORM_CORE_GRANTS.to_ascii_lowercase();
        assert!(sql.contains("grant usage on schema platform_core to ep_ops_ro"));
        assert!(sql.contains("grant select on table platform_core.schema_history to ep_ops_ro"));
        assert!(!sql.contains("grant select on all tables in schema platform_core to ep_ops_ro"));
        assert!(!sql.contains("grant create on schema platform_core to ep_ops_ro"));
    }
}
