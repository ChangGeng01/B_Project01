//! `platform_core` 应急账号与重新认证挑战的 SQL 实现体：
//! breakglass_activations（迁移 V202610120940）与 reauth_challenges
//! （迁移 V202610120925，过期清理由身份域后台任务承担）。
//!
//! 状态迁移一律带起态条件（CAS）：七态状态机的合法性判定在
//! ep-platform-identity 用例层，本文件只保证「仅当现状态等于
//! from 时落 to」的落库语义。`allowed_action_set` text[] 以逗号
//! 串进出（DbValue 无数组变体）。

use chrono::DateTime;
use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::UserAccount;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_platform_identity::ports::{BreakglassStore, ChallengeCleanup, NewBreakglass};
use ep_platform_identity::types::{BreakglassAction, BreakglassRow, BreakglassStatus};

use crate::conn::DbValue;
use crate::platform_core::identity_accounts::{
    col_opt_text, col_opt_ts, col_opt_uuid, col_text, col_uuid, ensure_one, shape_err,
};
use crate::tx::PgTx;

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "身份域读写必须在 PostgreSQL 事务内执行",
        )
    })
}

/// 建单即进入待批（DRAFT 只存在于用例层构造瞬间）。
const INSERT_BREAKGLASS_STMT: &str = "insert into platform_core.breakglass_activations \
     (id, doc_no, status, user_id, requested_by, reason, allowed_action_set) \
     values ($1, $2, 'PENDING_APPROVAL', $3, $4, $5, string_to_array($6, ','))";

const GET_BREAKGLASS_STMT: &str = "select id, doc_no, status, user_id, requested_by, \
     approved_by, reason, approval_ref, array_to_string(allowed_action_set, ','), \
     activated_at, expires_at, closed_at, rotated_at, rotation_result \
     from platform_core.breakglass_activations where id = $1";

const TRANSITION_BREAKGLASS_STMT: &str = "update platform_core.breakglass_activations \
     set status = $3 where id = $1 and status = $2";

const APPROVE_BREAKGLASS_STMT: &str = "update platform_core.breakglass_activations \
     set status = 'APPROVED', approved_by = $2, approval_ref = $3 \
     where id = $1 and status = 'PENDING_APPROVAL'";

const ACTIVATE_BREAKGLASS_STMT: &str = "update platform_core.breakglass_activations \
     set status = 'ACTIVE', activated_at = $2, expires_at = $3 \
     where id = $1 and status = 'APPROVED'";

const CLOSE_BREAKGLASS_STMT: &str = "update platform_core.breakglass_activations \
     set status = 'CLOSED', closed_at = $2 \
     where id = $1 and status = 'ACTIVE'";

/// 到期失效终态写回：EXPIRED 或 CLOSED 由用例给定；CLOSED 一并记 closed_at。
const FINALIZE_BREAKGLASS_STMT: &str = "update platform_core.breakglass_activations \
     set status = $2, rotation_result = $3, \
         closed_at = case when $2::text = 'CLOSED' then $4 else closed_at end \
     where id = $1 and status = 'ACTIVE'";

const LIST_DUE_ACTIVE_STMT: &str = "select id, doc_no, status, user_id, requested_by, \
     approved_by, reason, approval_ref, array_to_string(allowed_action_set, ','), \
     activated_at, expires_at, closed_at, rotated_at, rotation_result \
     from platform_core.breakglass_activations \
     where status = 'ACTIVE' and expires_at <= $1";

const LIST_IDLE_FOR_ROTATION_STMT: &str = "select id, doc_no, status, user_id, \
     requested_by, approved_by, reason, approval_ref, \
     array_to_string(allowed_action_set, ','), activated_at, expires_at, closed_at, \
     rotated_at, rotation_result \
     from platform_core.breakglass_activations \
     where status in ('CLOSED', 'EXPIRED') and (rotated_at is null or rotated_at < $1)";

const MARK_ROTATED_STMT: &str = "update platform_core.breakglass_activations \
     set rotated_at = $2, rotation_result = $3 \
     where id = $1 and status in ('CLOSED', 'EXPIRED')";

/// 过期重新认证挑战置 EXPIRED（仅 ISSUED/VERIFIED 两态可过期）。
const EXPIRE_CHALLENGES_STMT: &str = "update platform_core.reauth_challenges \
     set status = 'EXPIRED' \
     where status in ('ISSUED', 'VERIFIED') and expires_at <= $1";

/// `platform_core.breakglass_activations` 的存取器。
pub struct PgBreakglassStore;

#[async_trait::async_trait]
impl BreakglassStore for PgBreakglassStore {
    async fn insert(&self, tx: &mut dyn Tx, new: NewBreakglass) -> Result<uuid::Uuid, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let actions: Vec<&str> = new.allowed_action_set.iter().map(|a| a.as_str()).collect();
        let affected = pg
            .execute(
                INSERT_BREAKGLASS_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Text(new.doc_no),
                    DbValue::Uuid(new.user_id.as_uuid()),
                    DbValue::Uuid(new.requested_by.as_uuid()),
                    DbValue::Text(new.reason),
                    DbValue::Text(actions.join(",")),
                ],
            )
            .await?;
        ensure_one(affected, "应急申请插入未命中一行")?;
        Ok(id)
    }

    async fn get(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
    ) -> Result<Option<BreakglassRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg.query(GET_BREAKGLASS_STMT, &[DbValue::Uuid(id)]).await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_breakglass_row(row)?)),
            None => Ok(None),
        }
    }

    async fn transition(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        from: BreakglassStatus,
        to: BreakglassStatus,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                TRANSITION_BREAKGLASS_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Text(from.as_str().to_string()),
                    DbValue::Text(to.as_str().to_string()),
                ],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn approve(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        approved_by: Id<UserAccount>,
        approval_ref: &str,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                APPROVE_BREAKGLASS_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(approved_by.as_uuid()),
                    DbValue::Text(approval_ref.to_string()),
                ],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn activate(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        activated_at: DateTime<chrono::Utc>,
        expires_at: DateTime<chrono::Utc>,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                ACTIVATE_BREAKGLASS_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Timestamp(activated_at),
                    DbValue::Timestamp(expires_at),
                ],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn close(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        now: DateTime<chrono::Utc>,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                CLOSE_BREAKGLASS_STMT,
                &[DbValue::Uuid(id), DbValue::Timestamp(now)],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn finalize_with_rotation(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        to: BreakglassStatus,
        rotation_result: &str,
        now: DateTime<chrono::Utc>,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                FINALIZE_BREAKGLASS_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Text(to.as_str().to_string()),
                    DbValue::Text(rotation_result.to_string()),
                    DbValue::Timestamp(now),
                ],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn list_due_active(
        &self,
        tx: &mut dyn Tx,
        now: DateTime<chrono::Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(LIST_DUE_ACTIVE_STMT, &[DbValue::Timestamp(now)])
            .await?;
        rows.iter().map(|row| decode_breakglass_row(row)).collect()
    }

    async fn list_idle_for_rotation(
        &self,
        tx: &mut dyn Tx,
        cutoff: DateTime<chrono::Utc>,
    ) -> Result<Vec<BreakglassRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(LIST_IDLE_FOR_ROTATION_STMT, &[DbValue::Timestamp(cutoff)])
            .await?;
        rows.iter().map(|row| decode_breakglass_row(row)).collect()
    }

    async fn mark_rotated(
        &self,
        tx: &mut dyn Tx,
        id: uuid::Uuid,
        rotation_result: &str,
        now: DateTime<chrono::Utc>,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                MARK_ROTATED_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Timestamp(now),
                    DbValue::Text(rotation_result.to_string()),
                ],
            )
            .await?;
        Ok(affected == 1)
    }
}

/// `platform_core.reauth_challenges` 的过期清理（job-worker 周期任务）。
pub struct PgChallengeCleanup;

#[async_trait::async_trait]
impl ChallengeCleanup for PgChallengeCleanup {
    async fn expire_overdue(
        &self,
        tx: &mut dyn Tx,
        now: DateTime<chrono::Utc>,
    ) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(EXPIRE_CHALLENGES_STMT, &[DbValue::Timestamp(now)])
            .await
    }
}

fn decode_action_set(joined: &str) -> Result<Vec<BreakglassAction>, AppError> {
    if joined.is_empty() {
        return Ok(Vec::new());
    }
    joined
        .split(',')
        .map(|raw| BreakglassAction::parse(raw).ok_or_else(|| shape_err("应急允许动作字面量非法")))
        .collect()
}

fn decode_breakglass_row(row: &[DbValue]) -> Result<BreakglassRow, AppError> {
    let status = BreakglassStatus::parse(&col_text(row, 2)?)
        .ok_or_else(|| shape_err("应急状态字面量非法"))?;
    Ok(BreakglassRow {
        id: col_uuid(row, 0)?,
        doc_no: col_text(row, 1)?,
        status,
        user_id: Id::from_uuid(col_uuid(row, 3)?),
        requested_by: Id::from_uuid(col_uuid(row, 4)?),
        approved_by: col_opt_uuid(row, 5)?.map(Id::from_uuid),
        reason: col_text(row, 6)?,
        approval_ref: col_opt_text(row, 7)?,
        allowed_action_set: decode_action_set(&col_text(row, 8)?)?,
        activated_at: col_opt_ts(row, 9)?,
        expires_at: col_opt_ts(row, 10)?,
        closed_at: col_opt_ts(row, 11)?,
        rotated_at: col_opt_ts(row, 12)?,
        rotation_result: col_opt_text(row, 13)?,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use ep_foundation::id::marker::LegalEntity;
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

    #[test]
    fn statements_carry_state_preconditions() {
        assert!(
            INSERT_BREAKGLASS_STMT.contains("'PENDING_APPROVAL'"),
            "建单即待批"
        );
        assert!(
            APPROVE_BREAKGLASS_STMT.contains("and status = 'PENDING_APPROVAL'"),
            "批准只命中待批行"
        );
        assert!(
            ACTIVATE_BREAKGLASS_STMT.contains("and status = 'APPROVED'"),
            "启用只命中已批行"
        );
        assert!(CLOSE_BREAKGLASS_STMT.contains("and status = 'ACTIVE'"));
        assert!(
            FINALIZE_BREAKGLASS_STMT.contains("and status = 'ACTIVE'"),
            "到期失效只命中启用中行"
        );
        assert!(
            TRANSITION_BREAKGLASS_STMT.contains("where id = $1 and status = $2"),
            "条件迁移带起态 CAS"
        );
        assert!(
            MARK_ROTATED_STMT.contains("status in ('CLOSED', 'EXPIRED')"),
            "闲置轮换只登记终态行"
        );
    }

    #[test]
    fn action_set_round_trips_through_comma_join() {
        assert!(INSERT_BREAKGLASS_STMT.contains("string_to_array($6, ',')"));
        assert!(GET_BREAKGLASS_STMT.contains("array_to_string(allowed_action_set, ',')"));
        let got =
            decode_action_set("UNLOCK_OR_RESET_ADMIN,TRIGGER_BACKUP_OR_RESTORE").expect("可解");
        assert_eq!(got.len(), 2);
        assert!(decode_action_set("NOT_A_CLASS").is_err());
    }

    #[test]
    fn challenge_expiry_only_touches_open_states() {
        assert!(
            EXPIRE_CHALLENGES_STMT.contains("status in ('ISSUED', 'VERIFIED')"),
            "已核销/已失败的挑战不再过期处置：{EXPIRE_CHALLENGES_STMT}"
        );
    }

    #[tokio::test]
    async fn insert_reports_side_effect_and_new_id() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let id = PgBreakglassStore
            .insert(
                &mut tx,
                NewBreakglass {
                    doc_no: "BG-0001".into(),
                    user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
                    requested_by: Id::from_uuid(uuid::Uuid::from_u128(8)),
                    reason: "演练".into(),
                    allowed_action_set: vec![BreakglassAction::UnlockOrResetAdmin],
                },
            )
            .await
            .expect("建单可完成");
        assert_ne!(id, uuid::Uuid::nil());
        assert!(tx.has_side_effect());
        assert!(
            LIST_IDLE_FOR_ROTATION_STMT.contains("rotated_at is null or rotated_at < $1"),
            "未轮换或轮换早于 cutoff 均入闲置清单"
        );
        let _ = Utc::now();
    }
}
