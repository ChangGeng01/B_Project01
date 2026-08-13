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
//! `FnOnce`，不可重复调用，因此重试不发生在 trait 方法内部：
//! 需要重试语义的调用方使用固有方法 [`PgUnitOfWork::transact_retrying`]，
//! 它接受一个每次尝试产出全新执行体的工厂。单次 `transact` 遇到可重试
//! 错误时按「重试已用尽」形态返回 `PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED`；
//! 执行体一旦置位 side_effect_marker，无论剩余次数一律不重试。

use std::sync::{Arc, Mutex};
use std::time::Instant;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::port::tx::{BoxFuture, IsolationKind, SnapshotCtx, Tx, TxId, UnitOfWork};
use ep_foundation::security::SecurityContext;
use ep_foundation::Id;

use crate::budget::PoolKind;
use crate::conn::{DbConn, DbErrorClass, DbValue, PgError, SqlxConn};
use crate::metrics::{statement_kind, DbMetrics};
use crate::retry::{decide_retry, RetryDecision, RetryPolicy};
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
    /// side_effect_marker：执行体在产生任何外部可见副作用后置位，
    /// 置位后本事务不再参与重试。
    pub(crate) side_effect: bool,
    /// 最近一次连接层错误的副本，供 run_once 做重试判定与指标标签。
    pub(crate) last_pg_error: Option<PgError>,
}

impl PgTx {
    /// 置位副作用标记。执行体在写出任何对其他观察者可见的内容
    /// （审计事件、外发通知等）之后必须调用。
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

/// 单次尝试的失败详情：除对外的 AppError 外，保留重试判定所需的
/// SQLSTATE 与副作用标记。
struct AttemptFailure {
    app: AppError,
    sqlstate: Option<String>,
    side_effect: bool,
    retryable: bool,
}

impl AttemptFailure {
    fn plain(app: AppError) -> Self {
        Self {
            app,
            sqlstate: None,
            side_effect: false,
            retryable: false,
        }
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
        // Pool 来源下 drop 即归还（after_release 钩子再清一遍会话变量并
        // 断言无未结束事务）；Fixed 来源放回队列供下一次尝试复用。
        if let ConnSource::Fixed(list) = &self.source {
            unlock(list).push(conn);
        }
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
            self.release(conn);
            return Err(AttemptFailure::plain(app));
        }
        // 第三步：开事务执行闭包。
        if let Err(pg) = conn.begin(IsolationKind::ReadCommitted, false).await {
            let _ = SessionContext::clear(conn.as_mut()).await;
            self.release(conn);
            return Err(AttemptFailure {
                retryable: pg.is_retryable(),
                sqlstate: pg.sqlstate.clone(),
                side_effect: false,
                app: pg.into_app_error(),
            });
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
        let side_effect = pg_tx.side_effect;
        let mut body_pg_error = pg_tx.last_pg_error.clone();
        // 第四步：提交或回滚，归还前清除。
        let mut conn = pg_tx.conn.take().expect("连接只在 run_once 内被取回一次");
        let result: Result<T, AppError> = match outcome {
            Ok(v) => match conn.commit().await {
                Ok(()) => Ok(v),
                Err(pg) => {
                    body_pg_error = Some(pg.clone());
                    Err(pg.into_app_error())
                }
            },
            Err(e) => {
                let _ = conn.rollback().await;
                Err(e)
            }
        };
        let _ = SessionContext::clear(conn.as_mut()).await;
        self.release(conn);
        match result {
            Ok(v) => Ok(v),
            Err(app) => Err(AttemptFailure {
                retryable: body_pg_error.as_ref().is_some_and(PgError::is_retryable),
                sqlstate: body_pg_error.and_then(|e| e.sqlstate),
                side_effect,
                app,
            }),
        }
    }

    /// 带重试的事务执行。`make_body` 是执行体工厂：每次尝试产出一个
    /// 全新的 `FnOnce` 闭包。可重试错误（40001/40P01）按策略退避重试，
    /// 每次重试进 `ep_db_tx_retries_total`（pool + sqlstate 标签）；
    /// side_effect_marker 置位或次数用尽时返回
    /// `PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED`（错误码由单次执行映射）。
    pub async fn transact_retrying<T, M, F>(
        &self,
        ctx: &SecurityContext,
        mut make_body: M,
    ) -> Result<T, AppError>
    where
        T: Send + 'static,
        M: FnMut() -> F + Send,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        let mut failures = 0usize;
        loop {
            let failure = match self.run_once(ctx, make_body()).await {
                Ok(v) => return Ok(v),
                Err(f) => f,
            };
            failures += 1;
            let class = match failure.retryable {
                true => DbErrorClass::Retryable,
                false => DbErrorClass::Other,
            };
            let sqlstate = failure.sqlstate.as_deref();
            match decide_retry(&self.policy, class, sqlstate, failure.side_effect, failures) {
                RetryDecision::Retry(backoff) => {
                    let label = match sqlstate {
                        Some("40001") => "40001",
                        Some("40P01") => "40P01",
                        _ => "other",
                    };
                    self.metrics.tx_retry(self.pool_label, label);
                    tokio::time::sleep(backoff).await;
                }
                RetryDecision::Exhausted
                | RetryDecision::SideEffectMarked
                | RetryDecision::NotRetryable => return Err(failure.app),
            }
        }
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
        sc.apply(conn.as_mut()).await?;
        if let Err(pg) = conn
            .begin(IsolationKind::RepeatableReadSnapshot, true)
            .await
        {
            let _ = SessionContext::clear(conn.as_mut()).await;
            self.release(conn);
            return Err(pg.into_app_error());
        }
        // 快照号来自 pg_export_snapshot，取值形如 00000003-00000001-1；
        // 该语句不支持绑定参数，按单引号字面量拼接并做转义。
        let stmt = format!(
            "set transaction snapshot '{}'",
            snapshot_id.replace('\'', "''")
        );
        if let Err(pg) = conn.execute(&stmt, &[]).await {
            let _ = conn.rollback().await;
            let _ = SessionContext::clear(conn.as_mut()).await;
            self.release(conn);
            return Err(pg.into_app_error());
        }
        let started = Instant::now();
        let rows = conn.query(sql, params).await;
        self.metrics.statement_observed(
            self.pool_label,
            statement_kind(sql),
            started.elapsed().as_secs_f64(),
        );
        // 只读事务以 rollback 收尾同样干净，避免占用 idle_in_tx 预算。
        let _ = conn.rollback().await;
        let _ = SessionContext::clear(conn.as_mut()).await;
        self.release(conn);
        rows.map_err(PgError::into_app_error)
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
        sc.apply(conn.as_mut()).await?;
        if let Err(pg) = conn
            .begin(IsolationKind::RepeatableReadSnapshot, true)
            .await
        {
            let _ = SessionContext::clear(conn.as_mut()).await;
            self.release(conn);
            return Err(pg.into_app_error());
        }
        let rows = match conn.query("select pg_export_snapshot()", &[]).await {
            Ok(r) => r,
            Err(pg) => {
                let _ = conn.rollback().await;
                let _ = SessionContext::clear(conn.as_mut()).await;
                self.release(conn);
                return Err(pg.into_app_error());
            }
        };
        let snapshot_id = match rows.first().and_then(|r| r.first()) {
            Some(DbValue::Text(s)) => s.clone(),
            _ => {
                let _ = conn.rollback().await;
                let _ = SessionContext::clear(conn.as_mut()).await;
                self.release(conn);
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
        // 收尾：先取出连接（锁卫在 await 前释放），成功提交、失败回滚，
        // 随后清除会话变量并归还。
        let conn_opt = snapshot.lock().take();
        if let Some(mut conn) = conn_opt {
            match &outcome {
                Ok(_) => {
                    let _ = conn.commit().await;
                }
                Err(_) => {
                    let _ = conn.rollback().await;
                }
            }
            let _ = SessionContext::clear(conn.as_mut()).await;
            self.release(conn);
        }
        outcome
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

    /// side_effect_marker 置位后不重试：单次执行直接返
    /// SERIALIZATION_RETRY_EXHAUSTED，执行体只跑一遍。
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

    /// 40001 未置位：按策略重试直至成功，重试计数进指标。
    #[tokio::test]
    async fn retrying_succeeds_after_two_serialization_failures() {
        let mut conn = FakeConn::new();
        conn.fail_next(serialization_failure());
        conn.fail_next(serialization_failure());
        let (uow, metrics) = uow_with(conn);
        let attempt = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempt_in = attempt.clone();
        let ok = uow
            .transact_retrying(&ctx(), move || {
                let attempt = attempt_in.clone();
                move |tx: &mut dyn Tx| {
                    let attempt = attempt.clone();
                    let fut = async move {
                        attempt.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let pg = tx.as_any_mut().downcast_mut::<PgTx>().unwrap();
                        // 前两次尝试命中预置的 40001，第三次成功。
                        pg.execute("select 1", &[]).await?;
                        Ok(attempt.load(std::sync::atomic::Ordering::SeqCst))
                    };
                    Box::pin(fut) as BoxFuture<'_, Result<usize, AppError>>
                }
            })
            .await
            .expect("第三次尝试应成功");
        assert_eq!(ok, 3, "共执行三次");
        let retries = metrics.retries.lock().unwrap();
        assert_eq!(retries.len(), 2, "两次重试各记一次");
        assert!(retries
            .iter()
            .all(|(pool, s)| *pool == "rw" && *s == "40001"));
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
