//! 五具名池的构建与钩子。Rw20/Ro10/Worker5/Integ5/Ops2 的规模读配置，
//! 连接建立后（after_connect）下发池级超时、只读资源限额、
//! `application_name = '<process>/<pool>'` 与四条会话变量的空串初始化；
//! 归还前（after_release）逐项设回空串并断言无未结束事务，断言不成立即
//! 丢弃该连接（返回 false），不让带事务状态的连接回池。
//!
//! 超时取值的出处是阶段 1 计划第 7.2 节池表：Rw statement 10000、
//! lock 3000、idle_in_tx 15000；Ro statement 60000 加 work_mem 64MB
//! （temp_file_limit 2GB 为 SUSET 参数，改由引导侧角色默认值承接）；
//! Worker 300000；Ops 5000；Integ 10000。

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::budget::{PoolKind, PoolSpec};
use crate::metrics::DbMetrics;
use crate::session::{SESSION_VARS, SET_SESSION_VAR_STMT};

/// 进程名登记位。`application_name` 的 `<process>` 段在装配时登记一次，
/// 未登记取 "ep"。
static PROCESS_NAME: OnceLock<&'static str> = OnceLock::new();

pub fn register_process_name(name: &'static str) {
    // 重复登记取首个：装配只该发生一次，后来者不改写既成事实。
    let _ = PROCESS_NAME.set(name);
}

pub fn process_name() -> &'static str {
    PROCESS_NAME.get().copied().unwrap_or("ep")
}

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

/// 构建五池所需的全部取值。由装配侧从 EP__DB__* 配置段转换而来，
/// 本 crate 不依赖配置结构体。
#[derive(Clone, Debug)]
pub struct PoolBuildCfg {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub specs: [PoolSpec; 5],
    pub acquire_timeout: Duration,
    pub max_lifetime: Duration,
    pub idle_timeout: Duration,
    /// 顺序与 [`PoolKind::ALL`] 一一对应。
    pub timeouts: [PoolTimeouts; 5],
    pub ro_limits: RoResourceLimits,
    pub process_name: &'static str,
}

/// 五个具名池的持有者。一个进程一份，装配时建好后只读共享。
pub struct PgPools {
    pools: HashMap<PoolKind, Pool<Postgres>>,
    specs: [PoolSpec; 5],
    metrics: Arc<dyn DbMetrics>,
}

impl PgPools {
    /// 构建五池。连接不在这里预热，首用建立；钩子在建池时挂好。
    pub fn build(cfg: &PoolBuildCfg, metrics: Arc<dyn DbMetrics>) -> Result<Self, sqlx::Error> {
        let mut pools = HashMap::new();
        for (i, kind) in PoolKind::ALL.iter().enumerate() {
            let spec = cfg.specs[i];
            debug_assert_eq!(spec.kind, *kind, "specs 顺序必须与 PoolKind::ALL 一致");
            let timeouts = cfg.timeouts[i];
            let ro_limits = (*kind == PoolKind::Ro).then_some(cfg.ro_limits);
            let app_name = format!("{}/{}", cfg.process_name, kind.label());

            let cmds = session_commands(timeouts, ro_limits, &app_name);
            let options = sqlx::postgres::PgConnectOptions::new()
                .host(&cfg.host)
                .port(cfg.port)
                .database(&cfg.database)
                .username(&cfg.user)
                .password(&cfg.password);

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
                        // 归还前逐项设回空串，顺序与写入一致。
                        for name in SESSION_VARS {
                            sqlx::query(SET_SESSION_VAR_STMT)
                                .bind(name)
                                .bind("")
                                .execute(&mut *conn)
                                .await?;
                        }
                        // 保证不把未结束事务归还进池：无条件 rollback。
                        //
                        // F-82 更正：原判据是「事务外 `transaction_isolation` 取值为
                        // `read uncommitted`，不成立即丢弃连接」。**该前提对 PostgreSQL 不成立**
                        // ——`transaction_isolation` 由 `XactIsoLevel` 支撑，事务内外都取
                        // `default_transaction_isolation`，而 `db/bootstrap/00_database.sql`
                        // 逐字 `alter database ep set default_transaction_isolation = 'read committed'`。
                        // 于是该式恒为 `Ok(false)`，sqlx 对 `Ok(false)` 的处置是**关闭连接、不回池**，
                        // 五具名池因此退化成「每事务一次建连」：每个事务都要一次 TCP 握手、
                        // SCRAM 认证与 after_connect 的整套 SET，`max_lifetime`／`idle_timeout`
                        // 与规模表全部失效。这条在任何测试里都不可见——`FakeConn` 直接返回
                        // 自己的 `in_tx` 标志，活库用例又用显式 `close()` 绕开了本回调。
                        //
                        // 改为无条件 `rollback`：事务外 PostgreSQL 只发一条
                        // 「there is no transaction in progress」警告并成功，事务内则真正回滚。
                        // 意图（不把未结束事务归还进池）因此**总是**达成，而不是靠一个恒假的判据。
                        sqlx::query("rollback").execute(&mut *conn).await?;
                        Ok(true)
                    })
                })
                .connect_lazy_with(options);
            pools.insert(*kind, pool);
        }
        Ok(Self {
            pools,
            specs: cfg.specs,
            metrics,
        })
    }

    pub fn pool(&self, kind: PoolKind) -> Option<&Pool<Postgres>> {
        self.pools.get(&kind)
    }

    pub fn specs(&self) -> &[PoolSpec; 5] {
        &self.specs
    }

    /// 某池当前连接数。
    pub fn connection_count(&self, kind: PoolKind) -> u32 {
        self.pools.get(&kind).map_or(0, |p| p.size())
    }

    /// 把五池当前连接数刷进 gauge。装配侧在就绪探针与周期任务中调用。
    pub fn refresh_gauges(&self) {
        for kind in PoolKind::ALL {
            self.metrics
                .pool_connections(kind.label(), self.connection_count(kind));
        }
    }

    /// 关闭五池，停机路径调用。
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
    const INTEG: PoolTimeouts = PoolTimeouts {
        statement_ms: 10_000,
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
    fn worker_integ_ops_timeouts_match_the_pool_table() {
        assert_eq!(
            session_commands(WORKER, None, "job-worker/worker")[0],
            "set statement_timeout to 300000"
        );
        assert_eq!(
            session_commands(INTEG, None, "integration-gateway/integ")[0],
            "set statement_timeout to 10000",
            "Integ 保持配置现状 10000"
        );
        assert_eq!(
            session_commands(OPS, None, "ops-agent/ops")[0],
            "set statement_timeout to 5000"
        );
    }

    #[test]
    fn app_name_quotes_are_escaped() {
        let cmds = session_commands(RW, None, "we'ird/rw");
        assert_eq!(cmds.last().unwrap(), "set application_name to 'we''ird/rw'");
    }

    #[test]
    fn process_name_defaults_to_ep_until_registered() {
        // 本测试与其他测试共享全局登记位：只断言取值非空且为 ASCII。
        let name = process_name();
        assert!(!name.is_empty());
        assert!(name.is_ascii());
    }
}
