//! `PgTx`、`PgSnapshot` 与 `PgUnitOfWork`：端口 `Tx`/`SnapshotCtx`/`UnitOfWork`
//! 的 PostgreSQL 实现，声明位与实现位都在本 crate。
//!
//! 声明位与实现位都在本 crate。技术基线第 1.4 节配套纪律第四条原文要求
//! 「声明位在 ep-adapter-db，实现落在 ep-adapter-db-pg」，实测该写法不成立，
//! 两条独立理由：
//!
//! 一、孤儿规则。`Tx` 定义在 ep-foundation、`PgTx` 若定义在 ep-adapter-db，
//!     则 `impl Tx for PgTx` 对本 crate 而言 trait 与类型双双是外部的，
//!     rustc 报 E0117，无法绕开。
//! 二、依赖方向。要在本 crate 引用 ep-adapter-db 声明的类型就必须依赖它，
//!     而第 1.3 节禁止项第五条禁止 adapter 之间互相依赖，`xtask archcheck`
//!     的 adapter-no-peer-adapter 规则会直接判失败。
//!
//! 两条冲突互相独立，只改其中一条都不够。裁定 F-01 据此撤销 crate ep-adapter-db，
//! 端口下沉 ep_foundation::port::db；工作区内不存在名为 ep-adapter-db 的 crate。
//! 上面两条只作为成因说明保留。
//!
//! # transact 四步与重试形态
//!
//! `transact` 单次的四步：取连接 → 按 SecurityContext 写四条会话变量 →
//! 开事务执行闭包 → 提交或回滚，归还前清除会话变量。端口签名的执行体是
//! `FnOnce`，不可重复调用，因此重试不发生在 trait 方法内部。当前也没有
//! 能携带类型化幂等证明的生产调用方；保留的固有方法
//! [`PgUnitOfWork::transact_retrying`] 暂时同样只执行一次并失败关闭。
//! 重试策略判定保留给未来引入不可伪造的类型化证明后使用，不能把未置位
//! side-effect marker 当作重试授权。

use std::sync::{Arc, Mutex};
use std::time::Instant;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::port::tx::{BoxFuture, IsolationKind, SnapshotCtx, Tx, TxId, UnitOfWork};
use ep_foundation::security::SecurityContext;
use ep_foundation::Id;

use crate::budget::PoolKind;
use crate::conn::{DbConn, DbValue, PgError, SqlxConn};
use crate::metrics::{statement_kind, DbMetrics};
use crate::retry::RetryPolicy;
use crate::session::SessionContext;

/// 事务句柄。拥有连接抽象（`Box<dyn DbConn>`）而不是借用：
/// 端口 `as_any_mut` 要求句柄满足 `'static` 并可跨 crate downcast。
pub struct PgTx {
    pub(crate) tx_id: TxId,
    pub(crate) isolation: IsolationKind,
    pub(crate) legal_entity_id: Id<LegalEntity>,
    pub(crate) conn: Option<Box<dyn DbConn>>,
    pub(crate) pool_label: &'static str,
    pub(crate) metrics: Arc<dyn DbMetrics>,
    /// 遗留 side-effect marker：保留给现有执行体与未来策略判定测试；
    /// 未置位绝不构成幂等证明或重试授权。
    pub(crate) side_effect: bool,
    /// 最近一次连接层错误的副本，供 crate 内需要保存 PostgreSQL
    /// 原始诊断信息的适配器读取。
    pub(crate) last_pg_error: Option<PgError>,
}

impl PgTx {
    /// 置位遗留副作用标记。执行体仍可记录外部可见副作用，但当前公共
    /// 工厂入口无论此值如何都只执行一次。
    pub fn mark_side_effect(&mut self) {
        self.side_effect = true;
    }

    pub fn has_side_effect(&self) -> bool {
        self.side_effect
    }

    /// 取底层连接。守卫类逻辑（迁移窗口）在事务内执行自己的查询时经此下钻。
    pub fn conn_mut(&mut self) -> Result<&mut (dyn DbConn + '_), AppError> {
        match self.conn.as_mut() {
            Some(c) => Ok(&mut **c),
            None => Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "事务句柄已被回收",
            )),
        }
    }

    /// 在事务内执行一条写语句，带语句计时与错误映射。
    pub async fn execute(&mut self, sql: &str, params: &[DbValue]) -> Result<u64, AppError> {
        let started = Instant::now();
        let res = match self.conn.as_mut() {
            Some(c) => c.execute(sql, params).await,
            None => {
                return Err(AppError::new(
                    PLATFORM_SYSTEM_INTERNAL_ERROR,
                    "事务句柄已被回收",
                ))
            }
        };
        self.observe(sql, started);
        match res {
            Ok(n) => Ok(n),
            Err(pg) => {
                self.last_pg_error = Some(pg.clone());
                Err(pg.into_app_error())
            }
        }
    }

    /// 在事务内执行一条查询，带语句计时与错误映射。
    pub async fn query(
        &mut self,
        sql: &str,
        params: &[DbValue],
    ) -> Result<Vec<Vec<DbValue>>, AppError> {
        let started = Instant::now();
        let res = match self.conn.as_mut() {
            Some(c) => c.query(sql, params).await,
            None => {
                return Err(AppError::new(
                    PLATFORM_SYSTEM_INTERNAL_ERROR,
                    "事务句柄已被回收",
                ))
            }
        };
        self.observe(sql, started);
        match res {
            Ok(rows) => Ok(rows),
            Err(pg) => {
                self.last_pg_error = Some(pg.clone());
                Err(pg.into_app_error())
            }
        }
    }

    fn observe(&self, sql: &str, started: Instant) {
        self.metrics.statement_observed(
            self.pool_label,
            statement_kind(sql),
            started.elapsed().as_secs_f64(),
        );
    }
}

impl Tx for PgTx {
    fn tx_id(&self) -> TxId {
        self.tx_id
    }

    fn isolation(&self) -> IsolationKind {
        self.isolation
    }

    fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.legal_entity_id
    }

    fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
        self
    }
}

/// 快照上下文。持有快照所属连接（`Arc<Mutex<…>>`）：读方经
/// [`PgSnapshot::read_with`] 借用连接执行查询，事务收尾时由
/// `snapshot_transact` 收回连接并提交清除。
pub struct PgSnapshot {
    snapshot_id: String,
    taken_at: chrono::DateTime<chrono::Utc>,
    legal_entity_id: Id<LegalEntity>,
    conn: Mutex<Option<Box<dyn DbConn>>>,
    pool_label: &'static str,
    metrics: Arc<dyn DbMetrics>,
}

impl PgSnapshot {
    /// 借用快照连接执行一次读。连接同一时刻只借给一个读方；
    /// 读方完成后连接自动放回，供下一个读方或收尾使用。
    /// 执行体以 `BoxFuture` 形态借用连接，生命周期与借用绑定。
    pub async fn read_with<R, F>(&self, f: F) -> Result<R, AppError>
    where
        R: Send,
        F: for<'c> FnOnce(&'c mut (dyn DbConn + 'c)) -> BoxFuture<'c, Result<R, AppError>>,
    {
        let mut conn = self
            .lock()
            .take()
            .ok_or_else(|| AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "快照连接已被取走"))?;
        let started = Instant::now();
        let res = f(&mut *conn).await;
        // 快照读方的执行体没有可解析的 SQL 字面量，语句种类记 other。
        self.metrics
            .statement_observed(self.pool_label, "other", started.elapsed().as_secs_f64());
        *self.lock() = Some(conn);
        res
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Option<Box<dyn DbConn>>> {
        self.conn
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl SnapshotCtx for PgSnapshot {
    fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    fn taken_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.taken_at
    }

    fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.legal_entity_id
    }

    fn as_any(&self) -> &(dyn core::any::Any + Sync) {
        self
    }
}

/// 连接来源三态：真实池、测试用固定连接、启动即失败。
enum ConnSource {
    Pool(sqlx::Pool<sqlx::Postgres>),
    Fixed(Mutex<Vec<Box<dyn DbConn>>>),
}

/// 单次尝试的失败详情。当前公共入口失败关闭，不携带可被误当作
/// 重试授权的 SQLSTATE 或布尔标记。
struct AttemptFailure {
    app: AppError,
}

impl AttemptFailure {
    fn plain(app: AppError) -> Self {
        Self { app }
    }
}

/// 一个实例在装配时绑定一个池，不带池参数。
pub struct PgUnitOfWork {
    source: ConnSource,
    pool_label: &'static str,
    policy: RetryPolicy,
    metrics: Arc<dyn DbMetrics>,
}

impl PgUnitOfWork {
    /// 装配路径：绑定一个 sqlx 池。
    pub fn with_pool(
        pool: sqlx::Pool<sqlx::Postgres>,
        kind: PoolKind,
        policy: RetryPolicy,
        metrics: Arc<dyn DbMetrics>,
    ) -> Self {
        Self {
            source: ConnSource::Pool(pool),
            pool_label: kind.label(),
            policy,
            metrics,
        }
    }

    /// 纯逻辑测试路径：以固定连接队列代替池，连接用完归还到队列。
    pub fn with_fake_conns(
        conns: Vec<Box<dyn DbConn>>,
        pool_label: &'static str,
        policy: RetryPolicy,
        metrics: Arc<dyn DbMetrics>,
    ) -> Self {
        Self {
            source: ConnSource::Fixed(Mutex::new(conns)),
            pool_label,
            policy,
            metrics,
        }
    }

    pub fn pool_name(&self) -> &'static str {
        self.pool_label
    }

    pub fn policy(&self) -> &RetryPolicy {
        &self.policy
    }

    async fn acquire(&self) -> Result<Box<dyn DbConn>, AppError> {
        match &self.source {
            ConnSource::Pool(pool) => pool
                .acquire()
                .await
                .map(|c| Box::new(SqlxConn::new(c)) as Box<dyn DbConn>)
                .map_err(|_| AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "等待数据库连接超时")),
            ConnSource::Fixed(list) => match unlock(list).pop() {
                Some(c) => Ok(c),
                None => Err(AppError::new(
                    PLATFORM_SYSTEM_INTERNAL_ERROR,
                    "没有可用的测试连接",
                )),
            },
        }
    }

    fn release(&self, conn: Box<dyn DbConn>) {
        // Pool 来源下 drop 即归还（after_release 钩子先 ROLLBACK 再清空
        // 会话变量，任一步失败即丢连接）；Fixed 来源放回队列复用。
        if let ConnSource::Fixed(list) = &self.source {
            unlock(list).push(conn);
        }
    }

    /// 唯一连接收尾出口：无条件先 rollback，再清 session GUC。
    /// 只有进入收尾前仍可复用且两步都成功时，Fixed 连接才可重排队；
    /// Pool 连接则由 drop 后的 after_release 再做同序防线。
    async fn cleanup_connection(
        &self,
        mut conn: Box<dyn DbConn>,
        reusable_if_clean: bool,
    ) -> Result<(), AppError> {
        let rollback = conn.rollback().await.map_err(PgError::into_app_error);
        let clear = SessionContext::clear(conn.as_mut()).await;
        let cleanup = match (rollback, clear) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(error), _) | (Ok(()), Err(error)) => Err(error),
        };

        if reusable_if_clean && cleanup.is_ok() {
            self.release(conn);
        } else {
            drop(conn);
        }
        cleanup
    }

    #[cfg(test)]
    fn fixed_lock(list: &Mutex<Vec<Box<dyn DbConn>>>) -> usize {
        unlock(list).len()
    }

    /// transact 四步的完整单次执行。失败时保留重试判定所需的全部细节。
    async fn run_once<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AttemptFailure>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        // 第一步：取连接。
        let mut conn = self.acquire().await.map_err(AttemptFailure::plain)?;
        // 第二步：按 SecurityContext 写四条会话变量。
        let sc = SessionContext::from_security(ctx);
        if let Err(app) = sc.apply(conn.as_mut()).await {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(AttemptFailure::plain(app));
        }
        // 第三步：开事务执行闭包。
        if let Err(pg) = conn.begin(IsolationKind::ReadCommitted, false).await {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(AttemptFailure::plain(pg.into_app_error()));
        }
        let mut pg_tx = PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: ctx.legal_entity_id,
            conn: Some(conn),
            pool_label: self.pool_label,
            metrics: self.metrics.clone(),
            side_effect: false,
            last_pg_error: None,
        };
        let outcome = body(&mut pg_tx).await;
        // 第四步：提交结果确定后进入唯一清理出口。正文失败不覆盖其业务错误；
        // commit 成功后的清理失败则不能伪装为完整成功。
        let mut conn = pg_tx.conn.take().expect("连接只在 run_once 内被取回一次");
        match outcome {
            Ok(v) => match conn.commit().await {
                Ok(()) => self
                    .cleanup_connection(conn, true)
                    .await
                    .map(|()| v)
                    .map_err(AttemptFailure::plain),
                Err(pg) => {
                    let app = pg.into_app_error();
                    let _ = self.cleanup_connection(conn, false).await;
                    Err(AttemptFailure::plain(app))
                }
            },
            Err(e) => {
                let _ = self.cleanup_connection(conn, true).await;
                Err(AttemptFailure::plain(e))
            }
        }
    }

    /// 兼容保留的事务工厂入口。目前没有类型化幂等证明，因此无论 SQLSTATE
    /// 或旧 side-effect marker 状态如何，都只取一次执行体并执行一次。
    /// 未来只有引入不可由普通调用方布尔声称的证明类型后，才可在新 API 上
    /// 接回 [`RetryPolicy`] 的退避判定。
    pub async fn transact_retrying<T, M, F>(
        &self,
        ctx: &SecurityContext,
        make_body: M,
    ) -> Result<T, AppError>
    where
        T: Send + 'static,
        M: FnOnce() -> F + Send,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        self.run_once(ctx, make_body())
            .await
            .map_err(|failure| failure.app)
    }

    /// 以既有快照号执行一次快照读：另取连接，开 REPEATABLE READ 只读
    /// 事务，`SET TRANSACTION SNAPSHOT` 对齐快照后执行查询。
    pub async fn snapshot_read(
        &self,
        ctx: &SecurityContext,
        snapshot_id: &str,
        sql: &str,
        params: &[DbValue],
    ) -> Result<Vec<Vec<DbValue>>, AppError> {
        let mut conn = self.acquire().await?;
        let sc = SessionContext::from_security(ctx);
        if let Err(app) = sc.apply(conn.as_mut()).await {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(app);
        }
        if let Err(pg) = conn
            .begin(IsolationKind::RepeatableReadSnapshot, true)
            .await
        {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(pg.into_app_error());
        }
        // 快照号来自 pg_export_snapshot，取值形如 00000003-00000001-1；
        // 该语句不支持绑定参数，按单引号字面量拼接并做转义。
        let stmt = format!(
            "set transaction snapshot '{}'",
            snapshot_id.replace('\'', "''")
        );
        if let Err(pg) = conn.execute(&stmt, &[]).await {
            let _ = self.cleanup_connection(conn, true).await;
            return Err(pg.into_app_error());
        }
        let started = Instant::now();
        let rows = conn.query(sql, params).await;
        self.metrics.statement_observed(
            self.pool_label,
            statement_kind(sql),
            started.elapsed().as_secs_f64(),
        );
        let cleanup = self.cleanup_connection(conn, true).await;
        match rows {
            Ok(rows) => cleanup.map(|()| rows),
            Err(pg) => Err(pg.into_app_error()),
        }
    }

    #[cfg(test)]
    pub(crate) fn fixed_conn_count(&self) -> usize {
        match &self.source {
            ConnSource::Fixed(list) => Self::fixed_lock(list),
            ConnSource::Pool(_) => 0,
        }
    }
}

fn unlock(list: &Mutex<Vec<Box<dyn DbConn>>>) -> std::sync::MutexGuard<'_, Vec<Box<dyn DbConn>>> {
    list.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[async_trait::async_trait]
impl UnitOfWork for PgUnitOfWork {
    async fn transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        self.run_once(ctx, body).await.map_err(|f| f.app)
    }

    async fn snapshot_transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'s> FnOnce(&'s dyn SnapshotCtx) -> BoxFuture<'s, Result<T, AppError>>
            + Send
            + 'static,
    {
        let mut conn = self.acquire().await?;
        let sc = SessionContext::from_security(ctx);
        if let Err(app) = sc.apply(conn.as_mut()).await {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(app);
        }
        if let Err(pg) = conn
            .begin(IsolationKind::RepeatableReadSnapshot, true)
            .await
        {
            let _ = self.cleanup_connection(conn, false).await;
            return Err(pg.into_app_error());
        }
        let rows = match conn.query("select pg_export_snapshot()", &[]).await {
            Ok(r) => r,
            Err(pg) => {
                let _ = self.cleanup_connection(conn, true).await;
                return Err(pg.into_app_error());
            }
        };
        let snapshot_id = match rows.first().and_then(|r| r.first()) {
            Some(DbValue::Text(s)) => s.clone(),
            _ => {
                let _ = self.cleanup_connection(conn, true).await;
                return Err(AppError::new(
                    PLATFORM_SYSTEM_INTERNAL_ERROR,
                    "数据库未返回可用的快照标识",
                ));
            }
        };
        let snapshot = PgSnapshot {
            snapshot_id,
            taken_at: chrono::Utc::now(),
            legal_entity_id: ctx.legal_entity_id,
            conn: Mutex::new(Some(conn)),
            pool_label: self.pool_label,
            metrics: self.metrics.clone(),
        };
        let outcome = body(&snapshot).await;
        // 收尾：先取出连接（锁卫在 await 前释放），再使用同一个清理出口。
        let Some(mut conn) = snapshot.lock().take() else {
            return match outcome {
                Ok(_) => Err(AppError::new(
                    PLATFORM_SYSTEM_INTERNAL_ERROR,
                    "快照连接在收尾前丢失",
                )),
                Err(body_error) => Err(body_error),
            };
        };
        match outcome {
            Ok(value) => match conn.commit().await {
                Ok(()) => self.cleanup_connection(conn, true).await.map(|()| value),
                Err(commit_error) => {
                    let app = commit_error.into_app_error();
                    let _ = self.cleanup_connection(conn, false).await;
                    Err(app)
                }
            },
            Err(body_error) => {
                let _ = self.cleanup_connection(conn, true).await;
                Err(body_error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::error::codes::{
        PLATFORM_DB_REFERENCED_ROW_MISSING, PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED,
    };
    use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
    use ep_foundation::security::context::{RequestId, TraceId};

    use super::*;
    use crate::conn::{SQLSTATE_FOREIGN_KEY_VIOLATION, SQLSTATE_SERIALIZATION_FAILURE};
    use crate::fake::{FakeConn, FakeOp};
    use crate::metrics::RecordingDbMetrics;

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("0199aa11bb22cc33").expect("固定取值合法"),
            TraceId::new("0199aa11bb22cc330199aa11bb22cc33").expect("固定取值合法"),
        )
    }

    fn uow_with(conn: FakeConn) -> (PgUnitOfWork, Arc<RecordingDbMetrics>) {
        let metrics = Arc::new(RecordingDbMetrics::new());
        let uow = PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "rw",
            RetryPolicy::standard(),
            metrics.clone(),
        );
        (uow, metrics)
    }

    fn serialization_failure() -> PgError {
        PgError {
            sqlstate: Some(SQLSTATE_SERIALIZATION_FAILURE.to_string()),
            message: "could not serialize access".to_string(),
            constraint: None,
            column: None,
        }
    }

    fn connection_failure(message: &str) -> PgError {
        PgError {
            sqlstate: None,
            message: message.to_string(),
            constraint: None,
            column: None,
        }
    }

    fn observed_kinds(ops: &Mutex<Vec<FakeOp>>) -> Vec<&'static str> {
        ops.lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .map(|op| match op {
                FakeOp::Execute(sql, _) if sql == crate::session::SET_SESSION_VAR_STMT => "setvar",
                FakeOp::Query(_, _) => "query",
                FakeOp::Begin { .. } => "begin",
                FakeOp::Commit => "commit",
                FakeOp::Rollback => "rollback",
                _ => "execute",
            })
            .collect()
    }

    /// 冻结签名的可实现性与可调用性：`transact` 能在闭包里拿到 `&mut dyn Tx`。
    #[tokio::test]
    async fn transact_hands_out_tx() {
        let (uow, _) = uow_with(FakeConn::new());
        let got = uow
            .transact(&ctx(), |tx| {
                let isolation = tx.isolation();
                Box::pin(async move { Ok(isolation) })
            })
            .await;
        assert_eq!(got.expect("事务体应成功"), IsolationKind::ReadCommitted);
    }

    /// 跨 crate 取具体句柄的唯一写法在本 crate 内可用。
    #[tokio::test]
    async fn downcast_to_concrete_handle() {
        let (uow, _) = uow_with(FakeConn::new());
        let got = uow
            .transact(&ctx(), |tx| {
                let id = tx
                    .as_any_mut()
                    .downcast_mut::<PgTx>()
                    .map(|pg| pg.tx_id())
                    .expect("句柄必须是 PgTx");
                Box::pin(async move { Ok(id) })
            })
            .await;
        assert!(got.is_ok());
    }

    /// transact 四步的指标与归还行为：事务体语句经指标出口计时，
    /// 提交后连接归还。操作序列的逐步断言在
    /// `session_write_and_clear_sequence_is_exact` 与 guard 测试里做。
    #[tokio::test]
    async fn transact_runs_the_four_steps_in_order() {
        let (uow, metrics) = uow_with(FakeConn::new());
        uow.transact(&ctx(), |tx| {
            Box::pin(async move {
                let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                pg.execute("insert into t values (1)", &[]).await?;
                Ok(())
            })
        })
        .await
        .expect("事务体应成功");
        // 语句计时事件证明事务体语句真的走了指标出口。
        let obs = metrics.observations.lock().unwrap();
        assert!(obs
            .iter()
            .any(|(pool, kind, _)| *pool == "rw" && *kind == "insert"));
        drop(obs);
        assert_eq!(uow.fixed_conn_count(), 1, "连接必须归还");
    }

    /// 会话变量的写入与清除序列，用直接持有假连接的方式逐步断言。
    #[tokio::test]
    async fn session_write_and_clear_sequence_is_exact() {
        let mut conn = FakeConn::new();
        let sc = SessionContext::from_security(&ctx());
        sc.apply(&mut conn).await.unwrap();
        conn.begin(IsolationKind::ReadCommitted, false)
            .await
            .unwrap();
        conn.commit().await.unwrap();
        SessionContext::clear(&mut conn).await.unwrap();

        let kinds: Vec<&str> = conn
            .ops
            .iter()
            .map(|op| match op {
                FakeOp::Execute(sql, _) if sql == crate::session::SET_SESSION_VAR_STMT => "setvar",
                FakeOp::Begin { .. } => "begin",
                FakeOp::Commit => "commit",
                _ => "other",
            })
            .collect();
        assert_eq!(
            kinds,
            [
                "setvar", "setvar", "setvar", "setvar", "begin", "commit", "setvar", "setvar",
                "setvar", "setvar"
            ],
            "四条写入在前、四条清除在后"
        );
    }

    /// 事务体失败：回滚、清除、错误原样透传。
    #[tokio::test]
    async fn body_error_rolls_back_and_propagates() {
        let (uow, _) = uow_with(FakeConn::new());
        let err = uow
            .transact(&ctx(), |_tx| {
                Box::pin(async {
                    Err::<(), AppError>(AppError::new(
                        PLATFORM_DB_REFERENCED_ROW_MISSING,
                        "业务错误".to_string(),
                    ))
                })
            })
            .await
            .expect_err("事务体应失败");
        assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert_eq!(uow.fixed_conn_count(), 1, "失败也要归还连接");
    }

    #[tokio::test]
    async fn partial_session_application_is_rolled_back_cleared_and_discarded() {
        let observed = Arc::new(Mutex::new(Vec::new()));
        let mut conn = FakeConn::new();
        conn.observe_ops(observed.clone());
        conn.fail_session_execute(3, connection_failure("third session setter failed"));
        let (uow, _) = uow_with(conn);

        uow.transact(&ctx(), |_tx| Box::pin(async { Ok(()) }))
            .await
            .expect_err("部分会话写入失败必须失败关闭");

        assert_eq!(uow.fixed_conn_count(), 0, "部分应用过的连接不得重排队");
        assert_eq!(
            observed_kinds(&observed),
            ["setvar", "setvar", "setvar", "rollback", "setvar", "setvar", "setvar", "setvar"],
            "部分应用失败后仍须先 rollback，再尝试清除全部四个 GUC"
        );
    }

    #[tokio::test]
    async fn transact_success_reports_clear_failure_and_discards_connection() {
        let mut conn = FakeConn::new();
        conn.fail_session_execute(5, connection_failure("first clear setter failed"));
        let (uow, _) = uow_with(conn);

        let err = uow
            .transact(&ctx(), |_tx| Box::pin(async { Ok(()) }))
            .await
            .expect_err("commit 后清理失败不得伪装为完整成功");

        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
        assert_eq!(uow.fixed_conn_count(), 0, "清理失败的连接不得重排队");
    }

    #[tokio::test]
    async fn transact_body_error_survives_rollback_failure_and_discards_connection() {
        let mut conn = FakeConn::new();
        conn.fail_rollback(connection_failure("rollback failed"));
        let (uow, _) = uow_with(conn);

        let err = uow
            .transact(&ctx(), |_tx| {
                Box::pin(async {
                    Err::<(), AppError>(AppError::new(
                        PLATFORM_DB_REFERENCED_ROW_MISSING,
                        "body failed",
                    ))
                })
            })
            .await
            .expect_err("正文错误应保留");

        assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert_eq!(err.message, "body failed");
        assert_eq!(uow.fixed_conn_count(), 0, "rollback 失败的连接不得重排队");
    }

    /// 23503 统一映射 REFERENCED_ROW_MISSING 且 details 带约束与列。
    #[tokio::test]
    async fn foreign_key_violation_maps_to_referenced_row_missing() {
        let mut conn = FakeConn::new();
        conn.fail_next(PgError {
            sqlstate: Some(SQLSTATE_FOREIGN_KEY_VIOLATION.to_string()),
            message: "violates fk".to_string(),
            constraint: Some("fk_t_ref_id".to_string()),
            column: Some("ref_id".to_string()),
        });
        let (uow, _) = uow_with(conn);
        let err = uow
            .transact(&ctx(), |tx| {
                Box::pin(async move {
                    let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                    pg.execute("insert into t values (1)", &[]).await?;
                    Ok(())
                })
            })
            .await
            .expect_err("应因外键违约失败");
        assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert!(err.message.contains("fk_t_ref_id"));
        assert!(err.message.contains("ref_id"));
    }

    /// 遗留 marker 不改变失败关闭保障：执行体仍只跑一遍。
    #[tokio::test]
    async fn side_effect_marker_disables_retry() {
        let mut conn = FakeConn::new();
        conn.fail_next(serialization_failure());
        let (uow, _) = uow_with(conn);
        let runs = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let runs_in = runs.clone();
        let err = uow
            .transact_retrying(&ctx(), move || {
                let runs = runs_in.clone();
                move |tx: &mut dyn Tx| {
                    let runs = runs.clone();
                    let fut = async move {
                        runs.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                        pg.mark_side_effect();
                        pg.execute("select 1", &[]).await?;
                        Ok(())
                    };
                    Box::pin(fut) as BoxFuture<'_, Result<(), AppError>>
                }
            })
            .await
            .expect_err("应返重试耗尽");
        assert_eq!(err.code, PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED);
        assert_eq!(
            runs.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "置位后不重试"
        );
    }

    /// 未携带类型化幂等证明的执行体属于未知工作；即使调用方没有置位旧的
    /// side-effect marker，公开入口也不得自动再次调用工厂。
    #[tokio::test]
    async fn unmarked_unknown_work_is_single_attempt_fail_closed() {
        let mut conn = FakeConn::new();
        conn.fail_next(serialization_failure());
        let (uow, metrics) = uow_with(conn);
        let external_effects = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let external_effects_in = external_effects.clone();

        let err = uow
            .transact_retrying(&ctx(), move || {
                let external_effects = external_effects_in.clone();
                move |tx: &mut dyn Tx| {
                    let external_effects = external_effects.clone();
                    Box::pin(async move {
                        external_effects.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                        pg.execute("select 1", &[]).await?;
                        Ok(())
                    }) as BoxFuture<'_, Result<(), AppError>>
                }
            })
            .await
            .expect_err("未知工作第一次遇到 40001 后必须直接失败关闭");

        assert_eq!(err.code, PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED);
        assert_eq!(
            external_effects.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "未证明幂等的外部工作不得自动执行第二次"
        );
        assert!(
            metrics.retries.lock().unwrap().is_empty(),
            "没有获准重试时不得记录实际重试"
        );
    }

    /// 非重试 SQLSTATE（23503）在重试外壳下也直接返回，不重试。
    #[tokio::test]
    async fn non_retryable_error_returns_immediately_under_retrying() {
        let mut conn = FakeConn::new();
        conn.fail_next(PgError {
            sqlstate: Some(SQLSTATE_FOREIGN_KEY_VIOLATION.to_string()),
            message: "fk".to_string(),
            constraint: None,
            column: None,
        });
        let (uow, metrics) = uow_with(conn);
        let runs = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let runs_in = runs.clone();
        let err = uow
            .transact_retrying(&ctx(), move || {
                let runs = runs_in.clone();
                move |tx: &mut dyn Tx| {
                    let runs = runs.clone();
                    let fut = async move {
                        runs.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                        pg.execute("insert into t values (1)", &[]).await?;
                        Ok(())
                    };
                    Box::pin(fut) as BoxFuture<'_, Result<(), AppError>>
                }
            })
            .await
            .expect_err("应因外键违约失败");
        assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert_eq!(runs.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert!(
            metrics.retries.lock().unwrap().is_empty(),
            "非重试错误不记重试"
        );
    }

    /// 快照分支：REPEATABLE READ 只读事务 + pg_export_snapshot 取号。
    #[tokio::test]
    async fn snapshot_transact_exports_a_snapshot_id() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("00000003-00000001-1".to_string())]]);
        let (uow, _) = uow_with(conn);
        let got = uow
            .snapshot_transact(&ctx(), |snap| {
                let id = snap.snapshot_id().to_string();
                Box::pin(async move { Ok(id) })
            })
            .await;
        assert_eq!(got.expect("快照体应成功"), "00000003-00000001-1");
        assert_eq!(uow.fixed_conn_count(), 1, "快照收尾后连接归还");
    }

    /// 快照读方的连接借用：read_with 用完放回，收尾仍能提交。
    #[tokio::test]
    async fn snapshot_read_with_borrows_the_connection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("snap-1".to_string())]]);
        conn.push_rows(vec![vec![DbValue::Int64(7)]]);
        let (uow, _) = uow_with(conn);
        let got = uow
            .snapshot_transact(&ctx(), |snap| {
                Box::pin(async move {
                    let any: &(dyn core::any::Any + 'static) = snap.as_any();
                    let any = any.downcast_ref::<PgSnapshot>().unwrap();
                    let n = any
                        .read_with(|conn| {
                            Box::pin(async move {
                                let rows = conn.query("select 7", &[]).await.unwrap();
                                Ok(rows[0][0].clone())
                            })
                        })
                        .await?;
                    Ok(n)
                })
            })
            .await
            .expect("快照体应成功");
        assert_eq!(got, DbValue::Int64(7));
    }

    #[tokio::test]
    async fn snapshot_read_reports_clear_failure_and_discards_connection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(7)]]);
        conn.fail_session_execute(5, connection_failure("snapshot read clear failed"));
        let (uow, _) = uow_with(conn);

        let err = uow
            .snapshot_read(&ctx(), "snap-1", "select 7", &[])
            .await
            .expect_err("快照读取清理失败不得返回成功");

        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
        assert_eq!(uow.fixed_conn_count(), 0, "清理失败的快照读连接不得重排队");
    }

    #[tokio::test]
    async fn snapshot_transact_reports_clear_failure_and_discards_connection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("snap-clear-fail".to_string())]]);
        conn.fail_session_execute(5, connection_failure("snapshot owner clear failed"));
        let (uow, _) = uow_with(conn);

        let err = uow
            .snapshot_transact(&ctx(), |_snap| Box::pin(async { Ok(()) }))
            .await
            .expect_err("快照拥有者清理失败不得返回成功");

        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
        assert_eq!(uow.fixed_conn_count(), 0, "清理失败的快照连接不得重排队");
    }

    #[tokio::test]
    async fn snapshot_body_success_propagates_commit_failure_and_discards_the_connection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("snap-commit-fail".to_string())]]);
        conn.fail_commit(PgError {
            sqlstate: None,
            message: "connection lost while committing snapshot".to_string(),
            constraint: None,
            column: None,
        });
        let (uow, _) = uow_with(conn);

        let err = uow
            .snapshot_transact(&ctx(), |_snap| Box::pin(async { Ok("body-ok") }))
            .await
            .expect_err("快照正文成功不得掩盖 commit 失败");

        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
        assert_eq!(
            uow.fixed_conn_count(),
            0,
            "commit 失败后事务状态不确定，连接不得回到可复用队列"
        );
    }

    #[tokio::test]
    async fn snapshot_body_error_survives_rollback_failure_and_discards_the_connection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("snap-rollback-fail".to_string())]]);
        conn.fail_rollback(PgError {
            sqlstate: None,
            message: "connection lost while rolling back snapshot".to_string(),
            constraint: None,
            column: None,
        });
        let (uow, _) = uow_with(conn);

        let err = uow
            .snapshot_transact(&ctx(), |_snap| {
                Box::pin(async {
                    Err::<(), AppError>(AppError::new(
                        PLATFORM_DB_REFERENCED_ROW_MISSING,
                        "snapshot body business error",
                    ))
                })
            })
            .await
            .expect_err("正文失败应原样返回");

        assert_eq!(err.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert_eq!(err.message, "snapshot body business error");
        assert_eq!(
            uow.fixed_conn_count(),
            0,
            "rollback 失败后事务状态不确定，连接不得回到可复用队列"
        );
    }

    /// 空的 `Arc<[RoleCode]>` 与系统上下文的固定填充。
    #[test]
    fn system_context_uses_frozen_constants() {
        let c = ctx();
        assert_eq!(c.user_id.as_uuid(), SYSTEM_PRINCIPAL_ID);
        assert_eq!(
            c.device_id.as_str(),
            ep_foundation::principal::SYSTEM_DEVICE_ID
        );
        assert!(c.roles.is_empty());
        let _: &Arc<[_]> = &c.roles;
    }
}
