//! 活库集成测试。需要一台运行中的 PostgreSQL，入口是环境变量
//! `EP_TEST_PG_URL`（具备建库与删库权限的连接串，形如
//! `postgres://postgres@127.0.0.1:5432/postgres`）。未设该变量时本文件
//! 全部用例即刻返回并在 stderr 留痕：本机无库，留待集成任务在有库的
//! 环境里跑。
//!
//! 建库约定与 `ep-testkit` 的 `PgTestDb`（02 计划 D-08）一致：
//! `ep_test_<唯一后缀>` 独占建库，用例结束即删库。testkit 交付前这里
//! 内联同一约定的最小实现，testkit 交付后改用其夹具。

use std::sync::Arc;

use ep_adapter_db_pg::{
    NoopDbMetrics, PgMigrationWindowGuard, PgPools, PgUnitOfWork, PoolBuildCfg, PoolKind, PoolSpec,
    RetryPolicy,
};
use ep_foundation::error::codes::{
    PLATFORM_DB_MIGRATION_WINDOW_CLOSED, PLATFORM_DB_REFERENCED_ROW_MISSING,
};
use ep_foundation::port::db::MigrationWindowGuard;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_foundation::Id;
use sqlx::postgres::PgConnectOptions;
use sqlx::{Executor, Pool, Postgres};

const ENV_URL: &str = "EP_TEST_PG_URL";

/// 从连接串拆出五池构建所需的五个分量。只支持
/// `postgres://用户[:口令]@主机:端口/库名` 这一种形态：
/// 驱动侧的 `PgConnectOptions` 不吐口令，只能从原文拆。
fn parse_pg_url(url: &str) -> (String, u16, String, String, String) {
    let rest = url
        .strip_prefix("postgres://")
        .or_else(|| url.strip_prefix("postgresql://"))
        .expect("连接串必须以 postgres:// 开头");
    let (userinfo, rest) = rest.split_once('@').expect("连接串必须含 用户@主机 段");
    let (hostport, database) = rest.split_once('/').expect("连接串必须含 /库名 段");
    let (host, port) = hostport.split_once(':').expect("主机段必须含 :端口");
    let (user, password) = match userinfo.split_once(':') {
        Some((u, p)) => (u.to_string(), p.to_string()),
        None => (userinfo.to_string(), String::new()),
    };
    (
        host.to_string(),
        port.parse().expect("端口必须是数字"),
        user,
        password,
        database.to_string(),
    )
}

/// 活库会话。持有管理连接与独占库名，`cleanup` 负责删库。
struct LiveDb {
    admin: Pool<Postgres>,
    name: String,
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl LiveDb {
    /// 未设 `EP_TEST_PG_URL` 返回 None：调用方据此跳过。
    async fn new() -> Option<Self> {
        let url = std::env::var(ENV_URL).ok()?;
        let (host, port, user, password, admin_db) = parse_pg_url(&url);
        let options = PgConnectOptions::new()
            .host(&host)
            .port(port)
            .username(&user)
            .password(&password)
            .database(&admin_db);
        let admin = sqlx::pool::PoolOptions::<Postgres>::new()
            .max_connections(1)
            .connect_with(options.clone())
            .await
            .expect("管理连接必须建立成功");
        // 同一进程内多用例并行跑，纳秒时钟同刻取值会撞名（真实发生过
        // pg_database_datname_index 唯一冲突）；再拼进程内自增序号保证并行独占。
        static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let name = format!(
            "ep_test_{}{}_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("系统时钟在纪元后")
                .as_nanos(),
            std::process::id(),
            SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        admin
            .execute(format!("create database {name}").as_str())
            .await
            .expect("独占建库必须成功");
        // 用例共用的最小库内结构：迁移窗口两表加一对父子表（外键映射用）。
        let app_opts = options.clone().database(&name);
        let app = sqlx::pool::PoolOptions::<Postgres>::new()
            .max_connections(2)
            .connect_with(app_opts)
            .await
            .expect("应用连接必须建立成功");
        app.execute(
            "create schema platform_core;
             create table platform_core.migration_window_lock (
                 id int primary key check (id = 1)
             );
             insert into platform_core.migration_window_lock values (1);
             create table platform_core.migration_windows (
                 id bigint generated always as identity primary key,
                 state text not null,
                 opened_at timestamptz not null default now(),
                 expires_at timestamptz not null
             );
             create table t_parent (id bigint primary key);
             create table t_child (
                 id bigint primary key,
                 parent_id bigint not null,
                 constraint fk_child_parent foreign key (parent_id) references t_parent (id)
             );",
        )
        .await
        .expect("最小结构必须建立成功");
        Some(Self {
            admin,
            name,
            host,
            port,
            user,
            password,
        })
    }

    /// 用例收尾：先断应用侧连接，再删独占库。
    async fn cleanup(self, pools: Option<&PgPools>) {
        if let Some(pools) = pools {
            pools.close().await;
        }
        self.admin
            .execute(format!("drop database {} with (force)", self.name).as_str())
            .await
            .expect("独占库必须删除成功");
        self.admin.close().await;
    }
}

fn system_ctx() -> SecurityContext {
    SecurityContext::system(
        Id::from_uuid(SYSTEM_PRINCIPAL_ID),
        RequestId::new("0199aa11bb22cc33").expect("固定取值合法"),
        TraceId::new("0199aa11bb22cc330199aa11bb22cc33").expect("固定取值合法"),
    )
}

/// 用活库连接分量拼出五池构建取值：规模全部压到 1，钩子语义不变。
fn build_cfg(db: &LiveDb) -> PoolBuildCfg {
    use ep_adapter_db_pg::{PoolTimeouts, RoResourceLimits};
    const fn spec(kind: PoolKind) -> PoolSpec {
        PoolSpec {
            kind,
            max_connections: 1,
        }
    }
    PoolBuildCfg {
        host: db.host.clone(),
        port: db.port,
        database: db.name.clone(),
        user: db.user.clone(),
        password: db.password.clone(),
        specs: [
            spec(PoolKind::Rw),
            spec(PoolKind::Ro),
            spec(PoolKind::Worker),
            spec(PoolKind::Integ),
            spec(PoolKind::Ops),
        ],
        acquire_timeout: std::time::Duration::from_secs(5),
        max_lifetime: std::time::Duration::from_secs(1800),
        idle_timeout: std::time::Duration::from_secs(300),
        timeouts: [PoolTimeouts {
            statement_ms: 10_000,
            lock_ms: 3_000,
            idle_in_tx_ms: 15_000,
        }; 5],
        ro_limits: RoResourceLimits {
            work_mem_kb: 65_536,
            temp_file_limit_kb: 2_097_152,
        },
        process_name: "dbpg-test",
    }
}

/// after_connect 钩子：application_name、池级超时、四条会话变量空串。
#[tokio::test]
async fn live_after_connect_sets_pool_session_state() {
    let Some(db) = LiveDb::new().await else {
        eprintln!("跳过：未设 {ENV_URL}，需运行中的 PostgreSQL");
        return;
    };
    ep_adapter_db_pg::register_process_name("dbpg-test");
    let cfg = build_cfg(&db);
    let pools = PgPools::build(&cfg, Arc::new(NoopDbMetrics)).expect("五池构建应成功");
    let pool = pools.pool(PoolKind::Rw).expect("rw 池必须存在");
    let mut conn = pool.acquire().await.expect("取连接应成功");
    let row: (String, String, String) = sqlx::query_as(
        "select current_setting('application_name'),
                current_setting('statement_timeout'),
                current_setting('app.request_id')",
    )
    .fetch_one(conn.as_mut())
    .await
    .expect("会话状态可读");
    assert_eq!(row.0, "dbpg-test/rw", "application_name 形如 进程/池");
    assert_eq!(row.1, "10s", "statement_timeout 按池表下发");
    assert_eq!(row.2, "", "会话变量初始为空串");
    conn.close().await.expect("连接可正常关闭");
    db.cleanup(Some(&pools)).await;
}

/// transact 四步在真实库上：会话变量在事务体内可见，提交后可见写入。
#[tokio::test]
async fn live_transact_writes_session_vars_and_commits() {
    let Some(db) = LiveDb::new().await else {
        eprintln!("跳过：未设 {ENV_URL}，需运行中的 PostgreSQL");
        return;
    };
    let cfg = build_cfg(&db);
    let pools = PgPools::build(&cfg, Arc::new(NoopDbMetrics)).expect("五池构建应成功");
    let uow = PgUnitOfWork::with_pool(
        pools.pool(PoolKind::Rw).expect("rw 池必须存在").clone(),
        PoolKind::Rw,
        RetryPolicy::standard(),
        Arc::new(NoopDbMetrics),
    );
    let ctx = system_ctx();
    let got = uow
        .transact(&ctx, |tx| {
            Box::pin(async move {
                use ep_adapter_db_pg::{DbValue, PgTx};
                let _ = tx.tx_id();
                let pg = tx
                    .as_any_mut()
                    .downcast_mut::<PgTx>()
                    .expect("句柄必须是 PgTx");
                pg.execute("insert into t_parent values (1)", &[]).await?;
                let rows = pg
                    .query("select current_setting('app.request_id')", &[])
                    .await?;
                let seen = match &rows[0][0] {
                    DbValue::Text(s) => s.clone(),
                    other => panic!("会话变量应是文本：{other:?}"),
                };
                let ok = pg
                    .query("select id from t_parent where id = 1", &[])
                    .await?;
                Ok((seen, ok.len()))
            })
        })
        .await
        .expect("事务应提交成功");
    assert_eq!(got.0, "0199aa11bb22cc33", "事务体内可见请求级会话变量");
    assert_eq!(got.1, 1, "事务体内可见自己的写入");
    db.cleanup(Some(&pools)).await;
}

/// 23503 在真实库上映射 REFERENCED_ROW_MISSING，details 带约束与列。
#[tokio::test]
async fn live_fk_violation_maps_to_referenced_row_missing() {
    let Some(db) = LiveDb::new().await else {
        eprintln!("跳过：未设 {ENV_URL}，需运行中的 PostgreSQL");
        return;
    };
    let cfg = build_cfg(&db);
    let pools = PgPools::build(&cfg, Arc::new(NoopDbMetrics)).expect("五池构建应成功");
    let uow = PgUnitOfWork::with_pool(
        pools.pool(PoolKind::Rw).expect("rw 池必须存在").clone(),
        PoolKind::Rw,
        RetryPolicy::standard(),
        Arc::new(NoopDbMetrics),
    );
    let err = uow
        .transact(&system_ctx(), |tx| {
            Box::pin(async move {
                use ep_adapter_db_pg::PgTx;
                let pg = tx
                    .as_any_mut()
                    .downcast_mut::<PgTx>()
                    .expect("句柄必须是 PgTx");
                pg.execute("insert into t_child values (1, 999)", &[])
                    .await?;
                Ok(())
            })
        })
        .await
        .expect_err("被引用行缺失应失败");
    assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
    assert!(err.message.contains("fk_child_parent"), "details 带约束名");
    assert!(err.message.contains("parent_id"), "details 带外键列");
    db.cleanup(Some(&pools)).await;
}

/// 迁移窗口守卫的开与关两条路径（真实库上的行锁与过期判定）。
#[tokio::test]
async fn live_migration_window_guard_open_and_closed_paths() {
    let Some(db) = LiveDb::new().await else {
        eprintln!("跳过：未设 {ENV_URL}，需运行中的 PostgreSQL");
        return;
    };
    let cfg = build_cfg(&db);
    let pools = PgPools::build(&cfg, Arc::new(NoopDbMetrics)).expect("五池构建应成功");
    let uow = PgUnitOfWork::with_pool(
        pools.pool(PoolKind::Rw).expect("rw 池必须存在").clone(),
        PoolKind::Rw,
        RetryPolicy::standard(),
        Arc::new(NoopDbMetrics),
    );
    // 开一路：插入 OPEN 且未到期的窗口。
    uow.transact(&system_ctx(), |tx| {
        Box::pin(async move {
            use ep_adapter_db_pg::PgTx;
            let pg = tx
                .as_any_mut()
                .downcast_mut::<PgTx>()
                .expect("句柄必须是 PgTx");
            pg.execute(
                "insert into platform_core.migration_windows (state, expires_at)
                 values ('OPEN', now() + interval '1 hour')",
                &[],
            )
            .await?;
            Ok(())
        })
    })
    .await
    .expect("开窗写入应成功");
    uow.transact(&system_ctx(), |tx| {
        Box::pin(async move {
            PgMigrationWindowGuard
                .assert_open(tx)
                .await
                .expect("OPEN 且未过期应通过");
            Ok(())
        })
    })
    .await
    .expect("守卫通过则事务成功");
    // 关一路：把窗口改成 CLOSED。
    uow.transact(&system_ctx(), |tx| {
        Box::pin(async move {
            use ep_adapter_db_pg::PgTx;
            let pg = tx
                .as_any_mut()
                .downcast_mut::<PgTx>()
                .expect("句柄必须是 PgTx");
            pg.execute(
                "update platform_core.migration_windows set state = 'CLOSED'",
                &[],
            )
            .await?;
            Ok(())
        })
    })
    .await
    .expect("关窗写入应成功");
    let err = uow
        .transact(&system_ctx(), |tx| {
            Box::pin(async move {
                PgMigrationWindowGuard.assert_open(tx).await?;
                Ok(())
            })
        })
        .await
        .expect_err("CLOSED 窗口应拒绝");
    assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);
    db.cleanup(Some(&pools)).await;
}

/// 快照分支在真实库上：导出快照号、读方经快照对齐读到一致视图。
#[tokio::test]
async fn live_snapshot_transact_exports_a_usable_snapshot() {
    let Some(db) = LiveDb::new().await else {
        eprintln!("跳过：未设 {ENV_URL}，需运行中的 PostgreSQL");
        return;
    };
    let cfg = build_cfg(&db);
    let pools = PgPools::build(&cfg, Arc::new(NoopDbMetrics)).expect("五池构建应成功");
    let uow = PgUnitOfWork::with_pool(
        pools.pool(PoolKind::Rw).expect("rw 池必须存在").clone(),
        PoolKind::Rw,
        RetryPolicy::standard(),
        Arc::new(NoopDbMetrics),
    );
    // 先行写入一行，快照应能读到。
    uow.transact(&system_ctx(), |tx| {
        Box::pin(async move {
            use ep_adapter_db_pg::PgTx;
            let pg = tx
                .as_any_mut()
                .downcast_mut::<PgTx>()
                .expect("句柄必须是 PgTx");
            pg.execute("insert into t_parent values (7)", &[]).await?;
            Ok(())
        })
    })
    .await
    .expect("前置写入应成功");
    let got = uow
        .snapshot_transact(&system_ctx(), |snap| {
            Box::pin(async move {
                use ep_adapter_db_pg::{DbValue, PgSnapshot};
                let id = snap.snapshot_id().to_string();
                assert!(!id.is_empty(), "快照号非空");
                let any: &(dyn core::any::Any + 'static) = snap.as_any();
                let any = any.downcast_ref::<PgSnapshot>().expect("快照句柄可下钻");
                let count = any
                    .read_with(|conn| {
                        Box::pin(async move {
                            let rows = conn
                                .query("select count(*) from t_parent", &[])
                                .await
                                .expect("快照读应成功");
                            Ok(rows[0][0].clone())
                        })
                    })
                    .await?;
                match count {
                    DbValue::Int64(n) => Ok((id, n)),
                    other => panic!("count 应是整数：{other:?}"),
                }
            })
        })
        .await
        .expect("快照事务应成功");
    assert_eq!(got.1, 1, "快照视图读得到已提交的行");
    db.cleanup(Some(&pools)).await;
}
