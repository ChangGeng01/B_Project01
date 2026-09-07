//! 全机四具名池的进程唯一分配、构建与钩子。
//! Rw20/Ro10 只由 core-server 持有，Worker5 只由 job-worker 持有，
//! Ops2 只由 ops-agent 持有；
//! 连接建立后（after_connect）下发池级超时、只读资源限额、
//! `application_name = '<process>/<pool>'` 与四条会话变量的空串初始化；
//! 归还前（after_release）先无条件 ROLLBACK，再逐项清空会话变量；
//! 任一步失败由 sqlx 丢弃连接，不带污染状态回池。
//!
//! 超时取值的出处是阶段 1 计划第 7.2 节池表：Rw statement 10000、
//! lock 3000、idle_in_tx 15000；Ro statement 60000 加 work_mem 64MB
//! （temp_file_limit 2GB 为 SUSET 参数，改由引导侧角色默认值承接）；
//! Worker 300000；Ops 5000。

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::budget::{PoolKind, PoolOwner, PoolSpec};
use crate::metrics::DbMetrics;
use crate::session::{SESSION_VARS, SET_SESSION_VAR_STMT};

/// 池连接数 gauge 的统一刷新周期。短于常见抓取周期，避免长期暴露陈旧值。
pub const POOL_GAUGE_REFRESH_INTERVAL: Duration = Duration::from_secs(5);

/// 一个池的三项会话超时（毫秒）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoolTimeouts {
    pub statement_ms: u32,
    pub lock_ms: u32,
    pub idle_in_tx_ms: u32,
}

/// 只读池的两项资源限额（KB）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RoResourceLimits {
    pub work_mem_kb: u32,
    pub temp_file_limit_kb: u32,
}

/// 连接建立后要按序下发的 SET 命令（会话变量初始化不在内，它走参数化
/// 绑定）。抽成纯函数是为了让取值对应关系在无活库时可直接断言。
/// `ro_limits` 仅对只读池传入，由调用方按池种判定。
pub fn session_commands(
    timeouts: PoolTimeouts,
    ro_limits: Option<RoResourceLimits>,
    app_name: &str,
) -> Vec<String> {
    let mut cmds = vec![
        format!("set statement_timeout to {}", timeouts.statement_ms),
        format!("set lock_timeout to {}", timeouts.lock_ms),
        format!(
            "set idle_in_transaction_session_timeout to {}",
            timeouts.idle_in_tx_ms
        ),
    ];
    if let Some(ro) = ro_limits {
        cmds.push(format!("set work_mem to '{}kB'", ro.work_mem_kb));
        // temp_file_limit 不下发：它是 SUSET 参数，应用角色（非超级用户）
        // 会话级 SET 会被拒，导致 after_connect 失败、连接反复重建。
        // 该限额改由引导侧角色默认值承接（db/bootstrap/03_role_defaults.sql）。
    }
    cmds.push(format!(
        "set application_name to '{}'",
        app_name.replace('\'', "''")
    ));
    cmds
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ReleaseCleanupStep {
    Rollback,
    Clear(&'static str),
}

const RELEASE_CLEANUP_STEPS: [ReleaseCleanupStep; 5] = [
    ReleaseCleanupStep::Rollback,
    ReleaseCleanupStep::Clear(SESSION_VARS[0]),
    ReleaseCleanupStep::Clear(SESSION_VARS[1]),
    ReleaseCleanupStep::Clear(SESSION_VARS[2]),
    ReleaseCleanupStep::Clear(SESSION_VARS[3]),
];

fn release_cleanup_steps() -> &'static [ReleaseCleanupStep] {
    &RELEASE_CLEANUP_STEPS
}

/// 单个具名池的数据库凭据。每个池必须显式提供其 consumer 的凭据；
/// 是否共享数据库角色由受审角色/consumer registry 决定。
///
/// ```compile_fail
/// use ep_adapter_db_pg::PoolCredential;
/// use ep_platform_runtime::config::SecretString;
/// let credential = PoolCredential {
///     user: "ep_app_rw".into(),
///     password: SecretString::new("sensitive"),
/// };
/// let copied = credential.clone();
/// drop(copied);
/// ```
pub struct PoolCredential {
    pub user: String,
    pub password: ep_platform_runtime::config::SecretString,
}

impl std::fmt::Debug for PoolCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolCredential")
            .field("user", &self.user)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

/// 构建四池所需的全部取值。由装配侧从 EP__DB__* 配置段转换而来，
/// 本 crate 不依赖配置结构体。
#[derive(Debug)]
pub struct PoolBuildCfg {
    pub host: String,
    pub port: u16,
    pub database: String,
    /// 顺序与 [`PoolKind::ALL`] 一一对应；当前 owner 的每个池必须为 Some。
    pub credentials: [Option<PoolCredential>; 4],
    pub specs: [PoolSpec; 4],
    pub acquire_timeout: Duration,
    pub max_lifetime: Duration,
    pub idle_timeout: Duration,
    /// 顺序与 [`PoolKind::ALL`] 一一对应。
    pub timeouts: [PoolTimeouts; 4],
    pub ro_limits: RoResourceLimits,
    pub process_name: &'static str,
}

/// 某一进程被唯一分配的具名池持有者。
pub struct PgPools {
    pools: HashMap<PoolKind, Pool<Postgres>>,
    specs: [PoolSpec; 4],
    owner: PoolOwner,
    metrics: Arc<dyn DbMetrics>,
}

impl PgPools {
    /// 只构建 `owner` 持有的池。连接不在这里预热，首用建立；
    /// 另一进程的池连 lazy handle 都不创建，避免全机预算被复制。
    pub fn build(
        cfg: &PoolBuildCfg,
        owner: PoolOwner,
        metrics: Arc<dyn DbMetrics>,
    ) -> Result<Self, sqlx::Error> {
        let mut pools = HashMap::new();
        for (i, kind) in PoolKind::ALL.iter().enumerate() {
            let spec = cfg.specs[i];
            if spec.kind != *kind {
                return Err(sqlx::Error::Configuration(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "池规格位置 {i} 应为 {}，实际为 {}",
                        kind.label(),
                        spec.kind.label()
                    ),
                ))));
            }
            if kind.owner() != owner {
                continue;
            }
            let credential = cfg.credentials[i].as_ref().ok_or_else(|| {
                sqlx::Error::Configuration(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("池 {} 缺少独立凭据", kind.label()),
                )))
            })?;
            let timeouts = cfg.timeouts[i];
            let ro_limits = (*kind == PoolKind::Ro).then_some(cfg.ro_limits);
            let app_name = format!("{}/{}", cfg.process_name, kind.label());

            let cmds = session_commands(timeouts, ro_limits, &app_name);
            let options = sqlx::postgres::PgConnectOptions::new()
                .host(&cfg.host)
                .port(cfg.port)
                .database(&cfg.database)
                .username(&credential.user)
                // SQLx 的公开 builder 需要 `&str` 并在 PgConnectOptions 内持有一份
                // 第三方 String 副本。此处是当前不可消除的复制边界：上游
                // SecretString 与 PoolCredential 均不 Clone 且 drop 时 zeroize，
                // 但不能据此宣称 SQLx 内部副本也得到 zeroize。
                .password(credential.password.expose());

            let pool = PgPoolOptions::new()
                .max_connections(u32::from(spec.max_connections))
                .acquire_timeout(cfg.acquire_timeout)
                .max_lifetime(cfg.max_lifetime)
                .idle_timeout(cfg.idle_timeout)
                .after_connect(move |conn, _meta| {
                    let cmds = cmds.clone();
                    Box::pin(async move {
                        for cmd in cmds {
                            sqlx::query(&cmd).execute(&mut *conn).await?;
                        }
                        for name in SESSION_VARS {
                            sqlx::query(SET_SESSION_VAR_STMT)
                                .bind(name)
                                .bind("")
                                .execute(&mut *conn)
                                .await?;
                        }
                        Ok(())
                    })
                })
                .after_release(|conn, _meta| {
                    Box::pin(async move {
                        // 必须先结束任何遗留事务，再在 autocommit 状态清空 session GUC。
                        // 反过来会让 rollback 撤销清空并恢复旧租户上下文。
                        for step in release_cleanup_steps() {
                            match *step {
                                ReleaseCleanupStep::Rollback => {
                                    sqlx::query("rollback").execute(&mut *conn).await?;
                                }
                                ReleaseCleanupStep::Clear(name) => {
                                    sqlx::query(SET_SESSION_VAR_STMT)
                                        .bind(name)
                                        .bind("")
                                        .execute(&mut *conn)
                                        .await?;
                                }
                            }
                        }
                        Ok(true)
                    })
                })
                .connect_lazy_with(options);
            pools.insert(*kind, pool);
        }
        Ok(Self {
            pools,
            specs: cfg.specs,
            owner,
            metrics,
        })
    }

    pub const fn owner(&self) -> PoolOwner {
        self.owner
    }

    pub fn pool(&self, kind: PoolKind) -> Option<&Pool<Postgres>> {
        self.pools.get(&kind)
    }

    pub fn specs(&self) -> &[PoolSpec; 4] {
        &self.specs
    }

    /// 某池当前连接数。
    pub fn connection_count(&self, kind: PoolKind) -> u32 {
        self.pools.get(&kind).map_or(0, |p| p.size())
    }

    /// 把当前进程实际持有的池连接数刷进 gauge。
    pub fn refresh_gauges(&self) {
        for kind in PoolKind::ALL {
            if let Some(pool) = self.pools.get(&kind) {
                self.metrics.pool_connections(kind.label(), pool.size());
            }
        }
    }

    /// 立即发布一次池 gauge，随后按固定间隔刷新，直到停机 future 完成。
    /// 零间隔不启动忙循环，但仍发布初始值并等待停机。
    pub async fn refresh_gauges_until<F>(&self, interval: Duration, shutdown: F)
    where
        F: Future,
    {
        self.refresh_gauges();
        if interval.is_zero() {
            shutdown.await;
            return;
        }

        let start = tokio::time::Instant::now() + interval;
        let mut ticker = tokio::time::interval_at(start, interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        tokio::pin!(shutdown);
        loop {
            tokio::select! {
                _ = &mut shutdown => break,
                _ = ticker.tick() => self.refresh_gauges(),
            }
        }
    }

    /// 关闭四池，停机路径调用。
    pub async fn close(&self) {
        for kind in PoolKind::ALL {
            if let Some(pool) = self.pools.get(&kind) {
                pool.close().await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{NoopDbMetrics, RecordingDbMetrics};

    const RW: PoolTimeouts = PoolTimeouts {
        statement_ms: 10_000,
        lock_ms: 3_000,
        idle_in_tx_ms: 15_000,
    };
    const RO: PoolTimeouts = PoolTimeouts {
        statement_ms: 60_000,
        lock_ms: 3_000,
        idle_in_tx_ms: 15_000,
    };
    const WORKER: PoolTimeouts = PoolTimeouts {
        statement_ms: 300_000,
        lock_ms: 3_000,
        idle_in_tx_ms: 15_000,
    };
    const OPS: PoolTimeouts = PoolTimeouts {
        statement_ms: 5_000,
        lock_ms: 3_000,
        idle_in_tx_ms: 15_000,
    };
    const RO_LIMITS: RoResourceLimits = RoResourceLimits {
        work_mem_kb: 65_536,
        temp_file_limit_kb: 2_097_152,
    };

    fn lazy_build_cfg() -> PoolBuildCfg {
        let credential = |user: &str| {
            Some(PoolCredential {
                user: user.to_string(),
                password: ep_platform_runtime::config::SecretString::new("test-only"),
            })
        };
        PoolBuildCfg {
            host: "127.0.0.1".to_string(),
            port: 5432,
            database: "ep".to_string(),
            credentials: [
                credential("ep_app_rw"),
                credential("ep_analyst_ro"),
                credential("ep_app_rw"),
                credential("ep_ops_ro"),
            ],
            specs: crate::budget::STANDARD_POOL_SPECS,
            acquire_timeout: Duration::from_secs(1),
            max_lifetime: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(30),
            timeouts: [RW, RO, WORKER, OPS],
            ro_limits: RO_LIMITS,
            process_name: "test",
        }
    }

    #[test]
    fn credential_debug_is_redacted_including_parent_config() {
        let credential = PoolCredential {
            user: "ep_app_rw".into(),
            password: ep_platform_runtime::config::SecretString::new("never-log-this-secret"),
        };
        let credential_debug = format!("{credential:?}");
        assert!(credential_debug.contains("[REDACTED]"));
        assert!(!credential_debug.contains("never-log-this-secret"));

        let mut cfg = lazy_build_cfg();
        cfg.credentials[0] = Some(credential);
        assert!(!format!("{cfg:?}").contains("never-log-this-secret"));
    }

    #[test]
    fn build_rejects_reordered_specs_in_release_semantics() {
        let mut cfg = lazy_build_cfg();
        cfg.specs.swap(0, 1);
        let err = match PgPools::build(&cfg, PoolOwner::CoreServer, Arc::new(NoopDbMetrics)) {
            Ok(_) => panic!("乱序规格必须普通失败，不能依赖 debug_assert"),
            Err(err) => err,
        };
        assert!(err.to_string().contains("池规格位置 0"));
    }

    #[tokio::test]
    async fn each_process_builds_only_its_disjoint_host_pool_subset() {
        let cfg = lazy_build_cfg();
        assert_eq!(
            cfg.credentials[1].as_ref().map(|c| c.user.as_str()),
            Some("ep_analyst_ro")
        );
        let core = PgPools::build(&cfg, PoolOwner::CoreServer, Arc::new(NoopDbMetrics))
            .expect("惰性建池不连活库");
        assert!(core.pool(PoolKind::Rw).is_some());
        assert!(core.pool(PoolKind::Ro).is_some());
        assert!(core.pool(PoolKind::Worker).is_none());
        assert!(core.pool(PoolKind::Ops).is_none());

        let worker = PgPools::build(&cfg, PoolOwner::JobWorker, Arc::new(NoopDbMetrics))
            .expect("惰性建池不连活库");
        assert!(worker.pool(PoolKind::Worker).is_some());
        assert!(worker.pool(PoolKind::Rw).is_none());
        assert!(worker.pool(PoolKind::Ro).is_none());
        assert!(worker.pool(PoolKind::Ops).is_none());

        let ops = PgPools::build(&cfg, PoolOwner::OpsAgent, Arc::new(NoopDbMetrics))
            .expect("惰性建池不连活库");
        assert!(ops.pool(PoolKind::Ops).is_some());
        assert!(ops.pool(PoolKind::Rw).is_none());
        assert!(ops.pool(PoolKind::Ro).is_none());
        assert!(ops.pool(PoolKind::Worker).is_none());
    }

    #[tokio::test]
    async fn an_owned_pool_without_its_own_credential_is_rejected() {
        let mut cfg = lazy_build_cfg();
        cfg.credentials[1] = None;
        let err = PgPools::build(&cfg, PoolOwner::CoreServer, Arc::new(NoopDbMetrics))
            .err()
            .expect("缺 Ro 凭据必须拒绝建池");
        assert!(err.to_string().contains("ro"), "{err}");
    }

    #[tokio::test]
    async fn refresh_gauges_emits_only_pools_owned_by_process() {
        let metrics = Arc::new(RecordingDbMetrics::new());
        let core = PgPools::build(&lazy_build_cfg(), PoolOwner::CoreServer, metrics.clone())
            .expect("惰性建池不连活库");

        core.refresh_gauges();

        let mut gauges = metrics
            .gauges
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        gauges.sort_unstable();
        assert_eq!(gauges, vec![("ro", 0), ("rw", 0)]);
    }

    #[tokio::test]
    async fn gauge_refresher_runs_on_each_interval() {
        let metrics = Arc::new(RecordingDbMetrics::new());
        let core = Arc::new(
            PgPools::build(&lazy_build_cfg(), PoolOwner::CoreServer, metrics.clone())
                .expect("惰性建池不连活库"),
        );
        let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
        let task = tokio::spawn({
            let core = core.clone();
            async move {
                core.refresh_gauges_until(Duration::from_millis(5), async {
                    let _ = stop_rx.await;
                })
                .await;
            }
        });

        tokio::time::timeout(Duration::from_secs(1), async {
            loop {
                let count = metrics
                    .gauges
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .len();
                if count >= 4 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("core 的两个 owner 池必须至少刷新两轮");
        stop_tx.send(()).expect("刷新任务仍在运行");
        tokio::time::timeout(Duration::from_secs(1), task)
            .await
            .expect("收到停机信号后刷新任务必须退出")
            .expect("刷新任务不得 panic");
    }

    #[tokio::test]
    async fn gauge_refresher_returns_after_shutdown() {
        let metrics = Arc::new(RecordingDbMetrics::new());
        let core = PgPools::build(&lazy_build_cfg(), PoolOwner::CoreServer, metrics.clone())
            .expect("惰性建池不连活库");

        tokio::time::timeout(
            Duration::from_secs(1),
            core.refresh_gauges_until(Duration::from_secs(30), async {}),
        )
        .await
        .expect("已触发停机时不得等满刷新周期");
        assert_eq!(
            metrics
                .gauges
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .len(),
            2,
            "退出前仍须发布一次 owner 池的初始值"
        );
    }

    #[test]
    fn rw_pool_sets_its_three_timeouts_and_app_name() {
        let cmds = session_commands(RW, None, "core-server/rw");
        assert_eq!(cmds[0], "set statement_timeout to 10000");
        assert_eq!(cmds[1], "set lock_timeout to 3000");
        assert_eq!(cmds[2], "set idle_in_transaction_session_timeout to 15000");
        assert_eq!(
            cmds.last().unwrap(),
            "set application_name to 'core-server/rw'"
        );
        assert_eq!(cmds.len(), 4, "写池不下发只读资源限额");
    }

    #[test]
    fn ro_pool_adds_work_mem_but_not_temp_file_limit() {
        let cmds = session_commands(RO, Some(RO_LIMITS), "core-server/ro");
        assert_eq!(cmds[0], "set statement_timeout to 60000");
        assert!(
            cmds.contains(&"set work_mem to '65536kB'".to_string()),
            "work_mem 64MB"
        );
        assert!(
            !cmds.iter().any(|c| c.contains("temp_file_limit")),
            "SUSET 参数不得在会话级下发，由引导侧角色默认值承接"
        );
        assert_eq!(cmds.len(), 5, "只读池仅追加 work_mem 一项限额");
    }

    #[test]
    fn worker_and_ops_timeouts_match_the_pool_table() {
        assert_eq!(
            session_commands(WORKER, None, "job-worker/worker")[0],
            "set statement_timeout to 300000"
        );
        assert_eq!(
            session_commands(OPS, None, "ops-agent/ops")[0],
            "set statement_timeout to 5000"
        );
    }

    #[test]
    fn release_cleanup_rolls_back_before_clearing_session_variables() {
        let steps = release_cleanup_steps();
        assert_eq!(steps[0], ReleaseCleanupStep::Rollback);
        assert_eq!(steps.len(), 1 + SESSION_VARS.len());
        for (step, name) in steps[1..].iter().zip(SESSION_VARS) {
            assert_eq!(*step, ReleaseCleanupStep::Clear(name));
        }
    }

    #[test]
    fn app_name_quotes_are_escaped() {
        let cmds = session_commands(RW, None, "we'ird/rw");
        assert_eq!(cmds.last().unwrap(), "set application_name to 'we''ird/rw'");
    }
}
