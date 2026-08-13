//! 迁移窗口守卫。`MigrationWindowGuard`（B-03）的唯一实现
//! [`PgMigrationWindowGuard`]：在调用方事务内读取
//! `platform_core.migration_windows` 与单例锁表
//! `platform_core.migration_window_lock`，以 `SELECT … FOR UPDATE`
//! 对锁行串行化，未持 `OPEN` 且未过期的窗口一律返回
//! `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`（阶段 1 已登记，不重复登记）。
//!
//! 两表不带法人列、不建行级安全策略（迁移第 14 号登记豁免），
//! 因此查询不需要会话变量前置。

use ep_foundation::error::codes::PLATFORM_DB_MIGRATION_WINDOW_CLOSED;
use ep_foundation::error::AppError;
use ep_foundation::port::db::MigrationWindowGuard;
use ep_foundation::port::tx::Tx;

use crate::conn::{DbValue, PgError};
use crate::tx::PgTx;

/// 对单例锁行加行锁：同一时刻至多一个开窗/判窗流程在途。
/// 行数恒为 1（check (id = 1)）；锁行缺失视同窗口关闭。
const LOCK_ROW_STMT: &str =
    "select id from platform_core.migration_window_lock where id = 1 for update";

/// 取最近开窗记录的状态与到期时刻。
const LATEST_WINDOW_STMT: &str =
    "select state, expires_at from platform_core.migration_windows order by opened_at desc limit 1";

pub struct PgMigrationWindowGuard;

#[async_trait::async_trait]
impl MigrationWindowGuard for PgMigrationWindowGuard {
    async fn assert_open(&self, tx: &mut dyn Tx) -> Result<(), AppError> {
        let pg = tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
            AppError::new(
                PLATFORM_DB_MIGRATION_WINDOW_CLOSED,
                "迁移窗口断言必须在数据库事务内出示",
            )
        })?;
        let closed = AppError::new(
            PLATFORM_DB_MIGRATION_WINDOW_CLOSED,
            "当前不在允许结构变更的时间窗口内",
        );
        let conn = pg.conn_mut()?;
        let locked = conn
            .execute(LOCK_ROW_STMT, &[])
            .await
            .map_err(PgError::into_app_error)?;
        if locked == 0 {
            return Err(closed);
        }
        let rows = conn
            .query(LATEST_WINDOW_STMT, &[])
            .await
            .map_err(PgError::into_app_error)?;
        let Some(row) = rows.first() else {
            return Err(closed);
        };
        let state = match row.first() {
            Some(DbValue::Text(s)) => s.as_str(),
            _ => return Err(closed),
        };
        let expires_at = match row.get(1) {
            Some(DbValue::Timestamp(t)) => *t,
            _ => return Err(closed),
        };
        if state != "OPEN" || chrono::Utc::now() > expires_at {
            return Err(closed);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::id::marker::LegalEntity;
    use ep_foundation::id::Id;
    use ep_foundation::port::tx::{IsolationKind, TxId};

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    fn tx_over(conn: FakeConn) -> PgTx {
        PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::now_v7()),
            conn: Some(Box::new(conn)),
            pool_label: "rw",
            metrics: Arc::new(NoopDbMetrics),
            side_effect: false,
            last_pg_error: None,
        }
    }

    fn open_window_row(expires_at: chrono::DateTime<chrono::Utc>) -> Vec<DbValue> {
        vec![
            DbValue::Text("OPEN".to_string()),
            DbValue::Timestamp(expires_at),
        ]
    }

    #[tokio::test]
    async fn open_and_unexpired_window_passes() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1; // 锁行存在
        conn.push_rows(vec![open_window_row(
            chrono::Utc::now() + chrono::Duration::hours(1),
        )]);
        let mut tx = tx_over(conn);
        PgMigrationWindowGuard
            .assert_open(&mut tx)
            .await
            .expect("OPEN 且未过期应通过");
    }

    #[tokio::test]
    async fn closed_window_is_rejected() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        conn.push_rows(vec![vec![
            DbValue::Text("CLOSED".to_string()),
            DbValue::Timestamp(chrono::Utc::now() + chrono::Duration::hours(1)),
        ]]);
        let mut tx = tx_over(conn);
        let err = PgMigrationWindowGuard
            .assert_open(&mut tx)
            .await
            .expect_err("CLOSED 应拒绝");
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);
    }

    #[tokio::test]
    async fn expired_open_window_is_rejected() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        conn.push_rows(vec![open_window_row(
            chrono::Utc::now() - chrono::Duration::seconds(1),
        )]);
        let mut tx = tx_over(conn);
        let err = PgMigrationWindowGuard
            .assert_open(&mut tx)
            .await
            .expect_err("过期窗口应拒绝");
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);
    }

    #[tokio::test]
    async fn missing_lock_row_or_window_is_rejected() {
        // 锁行缺失：影响行数 0。
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        let mut tx = tx_over(conn);
        let err = PgMigrationWindowGuard
            .assert_open(&mut tx)
            .await
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);

        // 没有任何窗口记录。
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let err = PgMigrationWindowGuard
            .assert_open(&mut tx)
            .await
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);
    }

    #[tokio::test]
    async fn non_pg_tx_handle_is_rejected() {
        // 非 PgTx 句柄：用最小 Tx 实现模拟契约层误用。
        struct NotPg;
        impl Tx for NotPg {
            fn tx_id(&self) -> TxId {
                TxId(uuid::Uuid::nil())
            }
            fn isolation(&self) -> IsolationKind {
                IsolationKind::ReadCommitted
            }
            fn legal_entity_id(&self) -> Id<LegalEntity> {
                Id::<LegalEntity>::from_uuid(uuid::Uuid::nil())
            }
            fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
                self
            }
        }
        let mut handle = NotPg;
        let err = PgMigrationWindowGuard
            .assert_open(&mut handle)
            .await
            .expect_err("非数据库句柄应拒绝");
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CLOSED);
    }
}
