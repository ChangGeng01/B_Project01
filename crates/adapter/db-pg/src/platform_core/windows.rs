//! 迁移窗口开闭的 SQL 存取（A-09，02 计划 §3.5 表六）。
//!
//! `platform_core.migration_windows` 与单例锁表都不带法人列、不建行级
//! 安全策略（第 14 号迁移登记豁免），因此读写不需要法人会话变量前置。
//! 同一时刻至多一个 OPEN 窗口的不变量由对锁行的 `SELECT ... FOR UPDATE`
//! 串行化保障：判窗、开窗、关窗都先锁同一行再读最新状态。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_DB_MIGRATION_WINDOW_CONFLICT,
    PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::security::SecurityContext;
use uuid::Uuid;

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

/// 开窗结果（A-09 响应的库侧材料）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenedWindow {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
}

const LOCK_ROW_STMT: &str =
    "select id from platform_core.migration_window_lock where id = 1 for update";

const LATEST_OPEN_STMT: &str = "select id from platform_core.migration_windows \
     where state = 'OPEN' and expires_at > now() order by opened_at desc limit 1";

const INSERT_WINDOW_STMT: &str = "insert into platform_core.migration_windows \
     (id, state, approval_ref, reason, opened_by, opened_at, expires_at) \
     values ($1, 'OPEN', $2, $3, $4, now(), now() + make_interval(mins => $5::int))";  // ::int 不可省：绑参是 i64＝int8，
                                             // 而 make_interval 的具名实参是 int4，
                                             // int8→int4 不是隐式转换，函数解析会以
                                             // 42883「函数不存在」失败（F-80）

const GET_WINDOW_STMT: &str = "select id, state from platform_core.migration_windows where id = $1";

const CLOSE_WINDOW_STMT: &str = "update platform_core.migration_windows \
     set state = 'CLOSED', closed_by = $2, closed_at = now(), close_kind = 'MANUAL', \
         row_version = row_version + 1, updated_at = now() \
     where id = $1 and state = 'OPEN'";

/// 迁移窗口的 PostgreSQL 存取层。装配时绑定 Rw 池工作单元。
pub struct PgMigrationWindowStore {
    uow: Arc<PgUnitOfWork>,
}

impl PgMigrationWindowStore {
    pub fn new(uow: Arc<PgUnitOfWork>) -> Self {
        Self { uow }
    }

    /// 开窗：锁行串行化后判重，已有未过期 OPEN 窗口即冲突。
    /// `ttl_minutes` 的上限由端点按配置键校验，这里只负责落库。
    pub async fn open(
        &self,
        ctx: &SecurityContext,
        approval_ref: String,
        reason: String,
        ttl_minutes: u32,
    ) -> Result<OpenedWindow, AppError> {
        let principal = ctx.user_id.as_uuid();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    lock_row(pg).await?;
                    let open_rows = pg.query(LATEST_OPEN_STMT, &[]).await?;
                    if !open_rows.is_empty() {
                        return Err(AppError::new(
                            PLATFORM_DB_MIGRATION_WINDOW_CONFLICT,
                            "已有处于开放状态的迁移窗口，不得重复开窗",
                        ));
                    }
                    let id = Uuid::now_v7();
                    let rows = pg
                        .execute(
                            INSERT_WINDOW_STMT,
                            &[
                                DbValue::Uuid(id),
                                DbValue::Text(approval_ref),
                                DbValue::Text(reason),
                                DbValue::Uuid(principal),
                                DbValue::Int64(i64::from(ttl_minutes)),
                            ],
                        )
                        .await?;
                    if rows != 1 {
                        return Err(AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            "开窗写入未生效",
                        ));
                    }
                    // 到期时刻与库内 `now() + ttl` 同口径在应用侧复算，
                    // 避免为 returning 再走一次查询往返。
                    let expires_at = Utc::now() + chrono::Duration::minutes(i64::from(ttl_minutes));
                    Ok(OpenedWindow { id, expires_at })
                })
            })
            .await
    }

    /// 关窗：窗口不存在返回 NOT_FOUND_OR_DENIED 语义的未找到，
    /// 已是 CLOSED 视为幂等完成。
    pub async fn close(&self, ctx: &SecurityContext, window_id: Uuid) -> Result<(), AppError> {
        let principal = ctx.user_id.as_uuid();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    lock_row(pg).await?;
                    let rows = pg
                        .query(GET_WINDOW_STMT, &[DbValue::Uuid(window_id)])
                        .await?;
                    let Some(row) = rows.first() else {
                        return Err(AppError::new(
                            PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                            "迁移窗口不存在或不可见",
                        ));
                    };
                    let state = match row.get(1) {
                        Some(DbValue::Text(s)) => s.as_str(),
                        _ => {
                            return Err(AppError::new(
                                PLATFORM_SYSTEM_INTERNAL_ERROR,
                                "迁移窗口状态列形态不符",
                            ))
                        }
                    };
                    if state == "CLOSED" {
                        return Ok(());
                    }
                    let affected = pg
                        .execute(
                            CLOSE_WINDOW_STMT,
                            &[DbValue::Uuid(window_id), DbValue::Uuid(principal)],
                        )
                        .await?;
                    if affected == 0 {
                        return Err(AppError::new(
                            PLATFORM_DB_MIGRATION_WINDOW_CONFLICT,
                            "迁移窗口在关窗前已改变形态",
                        ));
                    }
                    Ok(())
                })
            })
            .await
    }
}

async fn lock_row(pg: &mut PgTx) -> Result<(), AppError> {
    let rows = pg.query(LOCK_ROW_STMT, &[]).await?;
    if rows.is_empty() {
        return Err(AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "迁移窗口单例锁行缺失",
        ));
    }
    Ok(())
}

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "迁移窗口存取必须在 PostgreSQL 事务内执行",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;
    use crate::retry::RetryPolicy;
    use ep_foundation::id::Id;
    use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
    use ep_foundation::security::context::{RequestId, TraceId};

    fn store_with(conn: FakeConn) -> PgMigrationWindowStore {
        let uow = Arc::new(PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "rw",
            RetryPolicy::standard(),
            Arc::new(NoopDbMetrics),
        ));
        PgMigrationWindowStore::new(uow)
    }

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("window-tests").expect("固定取值合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值合法"),
        )
    }

    #[tokio::test]
    async fn open_is_rejected_when_a_window_is_already_open() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(1)]]); // 锁行
        conn.push_rows(vec![vec![DbValue::Uuid(Uuid::from_u128(5))]]); // 已有 OPEN 窗口
        let store = store_with(conn);
        let err = store
            .open(&ctx(), "审批-1".into(), "理由".into(), 60)
            .await
            .expect_err("重复开窗必须拒绝");
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CONFLICT);
    }

    #[tokio::test]
    async fn open_succeeds_without_an_open_window() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(1)]]); // 锁行
        conn.push_rows(vec![]); // 无 OPEN 窗口
        conn.execute_affected = 1;
        let store = store_with(conn);
        let opened = store
            .open(&ctx(), "审批-1".into(), "理由".into(), 60)
            .await
            .expect("开窗成功");
        assert!(opened.expires_at > Utc::now());
    }

    #[tokio::test]
    async fn close_of_an_already_closed_window_is_idempotent() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(1)]]); // 锁行
        conn.push_rows(vec![vec![
            DbValue::Uuid(Uuid::from_u128(5)),
            DbValue::Text("CLOSED".into()),
        ]]);
        let store = store_with(conn);
        store
            .close(&ctx(), Uuid::from_u128(5))
            .await
            .expect("幂等关窗");
    }
}
