//! `IdempotencyStore` 的 SQL 实现体（阶段 3a，03 计划 §3.4.4）。
//!
//! 端口在 ep-foundation，三态判定纯逻辑在 ep-platform-outbox：
//! 本文件只做事务内的插入、读行与定稿写回，不另立判等
//! （裁定 C-07，全仓唯一生产实现）。
//!
//! 并发语义：`try_begin` 以 `INSERT ... ON CONFLICT DO NOTHING`
//! 抢占键位，影响行 1 即首call；0 时读已有行交 [`judge`] 三态判定。
//! 两个事务同时抢占时唯一约束保证至多一个成功；在途行的读方
//! 以 `PLATFORM.IDEMPOTENCY.IN_PROGRESS` 拒绝。事务回滚时
//! `IN_PROGRESS` 行随事务一并消失，不产生僵尸键。

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::port::db::{IdempotencyOutcome, IdempotencyScope, IdempotencyStore};
use ep_foundation::port::tx::Tx;
use ep_platform_outbox::{hash_hex, judge, ExistingKeyRow, KeyState};

use crate::conn::DbValue;
use crate::tx::PgTx;

/// 抢占键位：四元组冲突时静默不插，由影响行数区分首call与已有键。
/// `expires_at` 取建库时刻加保留天数，保留期清理据此扫描。
const INSERT_KEY_STMT: &str = "insert into platform_msg.idempotency_keys \
     (id, key, legal_entity_id, user_id, endpoint, request_hash, state, expires_at) \
     values ($1, $2, $3, $4, $5, $6, 'IN_PROGRESS', now() + $7::int4 * interval '1 day') \
     on conflict (legal_entity_id, user_id, endpoint, key) do nothing";

/// 读已有键行。响应体以文本形态取出（jsonb 无 DbValue 变体），
/// 响应状态提为 INT8 以对齐解码分支。
const SELECT_KEY_STMT: &str = "select state, request_hash, \
     response_status::int8, response_body::text \
     from platform_msg.idempotency_keys \
     where legal_entity_id = $1 and user_id = $2 and endpoint = $3 and key = $4";

/// 定稿：同事务置 COMPLETED 并写回响应。事务回滚时随事务消失。
const FINISH_KEY_STMT: &str = "update platform_msg.idempotency_keys \
     set state = 'COMPLETED', response_status = $5::smallint, response_body = $6::jsonb \
     where legal_entity_id = $1 and user_id = $2 and endpoint = $3 and key = $4 \
     and state = 'IN_PROGRESS'";

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "幂等键读写必须在 PostgreSQL 事务内执行",
        )
    })
}

/// `platform_msg.idempotency_keys` 的存取器。装配时注入保留天数
/// （`platform.idempotency.retention_days`，默认 7）。
pub struct PgIdempotencyStore {
    retention_days: u32,
}

impl PgIdempotencyStore {
    pub fn new(retention_days: u32) -> Self {
        Self { retention_days }
    }

    fn scope_params(scope: &IdempotencyScope) -> [DbValue; 4] {
        [
            DbValue::Uuid(scope.legal_entity_id.as_uuid()),
            DbValue::Uuid(scope.user_id.as_uuid()),
            DbValue::Text(scope.endpoint.clone()),
            DbValue::Uuid(scope.key),
        ]
    }
}

#[async_trait::async_trait]
impl IdempotencyStore for PgIdempotencyStore {
    async fn try_begin(
        &self,
        tx: &mut dyn Tx,
        scope: IdempotencyScope,
        request_hash: [u8; 32],
    ) -> Result<IdempotencyOutcome, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                INSERT_KEY_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    DbValue::Uuid(scope.key),
                    DbValue::Uuid(scope.legal_entity_id.as_uuid()),
                    DbValue::Uuid(scope.user_id.as_uuid()),
                    DbValue::Text(scope.endpoint.clone()),
                    DbValue::Text(hash_hex(&request_hash)),
                    DbValue::Int64(self.retention_days as i64),
                ],
            )
            .await?;
        if affected == 1 {
            return Ok(IdempotencyOutcome::FirstCall);
        }
        let rows = pg
            .query(SELECT_KEY_STMT, &Self::scope_params(&scope))
            .await?;
        let row = rows.first().ok_or_else(|| {
            AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "幂等键冲突后读不到既有行，疑似会话法人上下文漂移",
            )
        })?;
        judge(Some(decode_key_row(row)?), request_hash)
    }

    async fn finish(
        &self,
        tx: &mut dyn Tx,
        scope: IdempotencyScope,
        response_status: u16,
        response_body: &[u8],
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let body = core::str::from_utf8(response_body).map_err(|_| {
            AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "响应体不是合法 UTF-8 文本，无法落 jsonb 列",
            )
        })?;
        let mut params: Vec<DbValue> = Self::scope_params(&scope).to_vec();
        params.push(DbValue::Int64(response_status as i64));
        params.push(DbValue::Text(body.to_string()));
        let affected = pg.execute(FINISH_KEY_STMT, &params).await?;
        if affected != 1 {
            return Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "定稿未命中在途幂等键，事务不应继续提交",
            ));
        }
        Ok(())
    }
}

/// 四列投影解码为判定输入。形态不符即数据损坏，以内部错误返回。
fn decode_key_row(row: &[DbValue]) -> Result<ExistingKeyRow, AppError> {
    let bad = || {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "幂等键行的列形态与四列投影不符",
        )
    };
    let state = match row.first() {
        Some(DbValue::Text(s)) if s == "IN_PROGRESS" => KeyState::InProgress,
        Some(DbValue::Text(s)) if s == "COMPLETED" => KeyState::Completed,
        _ => return Err(bad()),
    };
    let request_hash_hex = match row.get(1) {
        Some(DbValue::Text(s)) => s.clone(),
        _ => return Err(bad()),
    };
    let status = match row.get(2) {
        Some(DbValue::Int64(n)) => Some(*n as u16),
        Some(DbValue::Null) | None => None,
        _ => return Err(bad()),
    };
    let body = match row.get(3) {
        Some(DbValue::Text(s)) => Some(s.as_bytes().to_vec()),
        Some(DbValue::Null) | None => None,
        _ => return Err(bad()),
    };
    Ok(ExistingKeyRow {
        state,
        request_hash_hex,
        response: match (status, body) {
            (Some(s), Some(b)) => Some((s, b)),
            _ => None,
        },
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::error::codes::PLATFORM_IDEMPOTENCY_IN_PROGRESS;
    use ep_foundation::id::marker::{LegalEntity, UserAccount};
    use ep_foundation::id::Id;
    use ep_foundation::port::tx::{IsolationKind, TxId};

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    fn tx_over(conn: FakeConn) -> PgTx {
        PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
            conn: Some(Box::new(conn)),
            pool_label: "rw",
            metrics: Arc::new(NoopDbMetrics),
            side_effect: false,
            last_pg_error: None,
        }
    }

    fn scope() -> IdempotencyScope {
        IdempotencyScope {
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
            user_id: Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(2)),
            endpoint: "POST /api/v1/platform/x".to_string(),
            key: uuid::Uuid::from_u128(9),
        }
    }

    const HASH: [u8; 32] = [7; 32];

    /// 首call：影响行 1 即取得执行权，插入语句必须带四元组冲突静默。
    #[tokio::test]
    async fn first_call_inserts_with_on_conflict_do_nothing() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let store = PgIdempotencyStore::new(7);
        let got = store
            .try_begin(&mut tx, scope(), HASH)
            .await
            .expect("抢占可完成");
        assert_eq!(got, IdempotencyOutcome::FirstCall);
        assert!(tx.has_side_effect(), "写入必须置副作用标志");
    }

    /// SQL 构造断言：抢占语句的四元组冲突静默与唯一索引同列序，
    /// 读行与定稿语句都以四元组定位，定稿只命中在途行。
    #[test]
    fn statement_shapes_follow_the_table_contract() {
        assert!(
            INSERT_KEY_STMT
                .contains("on conflict (legal_entity_id, user_id, endpoint, key) do nothing"),
            "冲突静默的四元组必须与唯一索引同列序：{INSERT_KEY_STMT}"
        );
        assert!(
            INSERT_KEY_STMT.contains("'IN_PROGRESS'"),
            "抢占行一律先在途"
        );
        for stmt in [SELECT_KEY_STMT, FINISH_KEY_STMT] {
            assert!(
                stmt.contains(
                    "legal_entity_id = $1 and user_id = $2 and endpoint = $3 and key = $4"
                ),
                "四元组定位必须齐备：{stmt}"
            );
        }
        assert!(
            FINISH_KEY_STMT.contains("and state = 'IN_PROGRESS'"),
            "定稿只允许覆盖在途行"
        );
        assert!(
            FINISH_KEY_STMT.contains("response_body = $6::jsonb"),
            "响应体以文本绑定显式转 jsonb"
        );
    }

    /// 已有键 COMPLETED 同哈希：回放定稿响应。
    #[tokio::test]
    async fn completed_same_hash_replays_the_finalized_response() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        conn.push_rows(vec![vec![
            DbValue::Text("COMPLETED".to_string()),
            DbValue::Text(hash_hex(&HASH)),
            DbValue::Int64(201),
            DbValue::Text("{\"id\":1}".to_string()),
        ]]);
        let mut tx = tx_over(conn);
        let got = PgIdempotencyStore::new(7)
            .try_begin(&mut tx, scope(), HASH)
            .await
            .expect("读行可完成");
        assert_eq!(
            got,
            IdempotencyOutcome::Replay {
                response_status: 201,
                response_body: b"{\"id\":1}".to_vec()
            }
        );
    }

    /// 已有键 COMPLETED 异哈希：载荷不符。
    #[tokio::test]
    async fn completed_other_hash_is_payload_mismatch() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        conn.push_rows(vec![vec![
            DbValue::Text("COMPLETED".to_string()),
            DbValue::Text(hash_hex(&[9; 32])),
            DbValue::Int64(200),
            DbValue::Text("{}".to_string()),
        ]]);
        let mut tx = tx_over(conn);
        let got = PgIdempotencyStore::new(7)
            .try_begin(&mut tx, scope(), HASH)
            .await
            .expect("读行可完成");
        assert_eq!(got, IdempotencyOutcome::PayloadMismatch);
    }

    /// 已有键 IN_PROGRESS：以冲突错误返回，不占 Outcome 变体。
    #[tokio::test]
    async fn in_progress_row_is_rejected_with_the_registered_code() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        conn.push_rows(vec![vec![
            DbValue::Text("IN_PROGRESS".to_string()),
            DbValue::Text(hash_hex(&HASH)),
            DbValue::Null,
            DbValue::Null,
        ]]);
        let mut tx = tx_over(conn);
        let err = PgIdempotencyStore::new(7)
            .try_begin(&mut tx, scope(), HASH)
            .await
            .expect_err("在途必须返错");
        assert_eq!(err.code, PLATFORM_IDEMPOTENCY_IN_PROGRESS);
    }

    /// 冲突后读不到行（会话法人上下文漂移）即内部错误，不得假判首call。
    #[tokio::test]
    async fn conflict_without_a_readable_row_is_internal_error() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        conn.push_rows(vec![]);
        let mut tx = tx_over(conn);
        let err = PgIdempotencyStore::new(7)
            .try_begin(&mut tx, scope(), HASH)
            .await
            .expect_err("读不到行必须返错");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    /// 定稿：同一事务置 COMPLETED 并写回响应，语句形态与参数齐备。
    #[tokio::test]
    async fn finish_marks_completed_and_writes_back_the_response() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        PgIdempotencyStore::new(7)
            .finish(&mut tx, scope(), 201, b"{\"id\":1}")
            .await
            .expect("定稿可完成");
        assert!(tx.has_side_effect());
    }

    /// 定稿未命中在途行即内部错误：防把他人已定稿的键覆盖。
    #[tokio::test]
    async fn finish_without_a_matching_in_progress_row_fails() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        let mut tx = tx_over(conn);
        let err = PgIdempotencyStore::new(7)
            .finish(&mut tx, scope(), 200, b"{}")
            .await
            .expect_err("零影响行必须返错");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    /// 非 PgTx 句柄一律拒绝（downcast 纪律）。
    #[tokio::test]
    async fn non_pg_handle_is_rejected() {
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
        let err = PgIdempotencyStore::new(7)
            .try_begin(&mut handle, scope(), HASH)
            .await
            .expect_err("非 PgTx 句柄必须拒绝");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    /// 行解码的形态防御：列形态不符即拒。
    #[test]
    fn row_decoding_rejects_bad_shape() {
        assert!(decode_key_row(&[DbValue::Int64(1)]).is_err());
        assert!(decode_key_row(&[DbValue::Text("OTHER".to_string())]).is_err());
    }
}
