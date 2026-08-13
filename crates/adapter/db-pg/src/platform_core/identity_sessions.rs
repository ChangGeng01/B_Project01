//! `platform_core` 身份域会话面 SQL 实现体：sessions、user_devices、
//! account_lockouts、login_attempts 四张表（迁移 V202610120920/0915/
//! 0935/0930）。令牌仅以 SHA-256 摘要的 hex 形态进出，明文令牌
//! 在任何 SQL 与日志中都不出现。

use chrono::{DateTime, Duration, Utc};
use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::context::ClientKind;
use ep_platform_authz::types::hex_encode;
use ep_platform_identity::ports::{
    DeviceStore, LockoutStore, LoginAttemptStore, NewDevice, NewLoginAttempt, NewSession,
    SessionStore,
};
use ep_platform_identity::types::{DeviceRow, DeviceStatus, LockoutRow, SessionRow};

use crate::conn::DbValue;
use crate::platform_core::identity_accounts::{
    col_bool, col_hex, col_i64, col_opt_text, col_opt_ts, col_opt_uuid, col_text, col_ts, col_uuid,
    ensure_one, shape_err,
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

fn client_str(c: ClientKind) -> &'static str {
    match c {
        ClientKind::Win => "win",
        ClientKind::Mac => "mac",
        ClientKind::Ios => "ios",
        ClientKind::Android => "android",
        ClientKind::Portal => "portal",
        ClientKind::Ops => "ops",
    }
}

/// `last_seen_at` 与 `issued_at` 同参（$6 复用）；`client` 取库默认
/// 字面量 'ops'（NewSession 端口不携带客户端，登录流水已另记
/// login_attempts.client）。
const INSERT_SESSION_STMT: &str = "insert into platform_core.sessions \
     (id, user_id, user_device_row_id, token_hash, active_legal_entity_id, client, \
      issued_at, expires_at, idle_expires_at, last_seen_at, is_breakglass) \
     values ($1, $2, $3, decode($4, 'hex'), $5, 'ops', $6, $7, $8, $6, $9)";

const FIND_SESSION_BY_DIGEST_STMT: &str = "select id, user_id, user_device_row_id, \
     encode(token_hash, 'hex'), active_legal_entity_id, issued_at, expires_at, \
     idle_expires_at, last_seen_at, revoked_at, revoke_reason, is_breakglass \
     from platform_core.sessions \
     where encode(token_hash, 'hex') = $1 and revoked_at is null \
     and expires_at > now() and idle_expires_at > now()";

const LIST_USER_SESSIONS_STMT: &str = "select id, user_id, user_device_row_id, \
     encode(token_hash, 'hex'), active_legal_entity_id, issued_at, expires_at, \
     idle_expires_at, last_seen_at, revoked_at, revoke_reason, is_breakglass \
     from platform_core.sessions \
     where user_id = $1 and revoked_at is null and expires_at > now() \
     order by issued_at";

const REVOKE_SESSION_STMT: &str = "update platform_core.sessions \
     set revoked_at = now(), revoke_reason = $2 \
     where id = $1 and revoked_at is null";

const REVOKE_ALL_SESSIONS_STMT: &str = "update platform_core.sessions \
     set revoked_at = now(), revoke_reason = $2 \
     where user_id = $1 and revoked_at is null";

const REVOKE_SESSIONS_BY_DEVICE_STMT: &str = "update platform_core.sessions \
     set revoked_at = now(), revoke_reason = $3 \
     where user_id = $1 and user_device_row_id = $2 and revoked_at is null";

/// 到期失效：绝对到期或空闲到期任一命中即撤，理由固定 EXPIRED。
const EXPIRE_OVERDUE_SESSIONS_STMT: &str = "update platform_core.sessions \
     set revoked_at = now(), revoke_reason = 'EXPIRED' \
     where revoked_at is null and (expires_at <= $1 or idle_expires_at <= $1)";

const FIND_DEVICE_STMT: &str = "select id, user_id, device_id, client, \
     encode(public_key, 'hex'), attestation_ref, restricted_legal_entity_id, status \
     from platform_core.user_devices \
     where user_id = $1 and device_id = $2 and status = 'ACTIVE'";

const LIST_DEVICES_STMT: &str = "select id, user_id, device_id, client, \
     encode(public_key, 'hex'), attestation_ref, restricted_legal_entity_id, status \
     from platform_core.user_devices where user_id = $1 order by created_at";

const INSERT_DEVICE_STMT: &str = "insert into platform_core.user_devices \
     (id, user_id, device_id, client, public_key, attestation_ref, \
      restricted_legal_entity_id, status) \
     values ($1, $2, $3, $4, decode($5, 'hex'), $6, $7, 'ACTIVE')";

const REVOKE_DEVICE_STMT: &str = "update platform_core.user_devices \
     set status = 'REVOKED', revoked_at = now() \
     where id = $1 and status <> 'REVOKED'";

/// FOR UPDATE 行锁语义：锁定检查与失败计数在同一事务内串行。
const LOCKOUT_FOR_UPDATE_STMT: &str = "select id, user_id, failure_count::int8, \
     window_started_at, locked_until, last_failure_at \
     from platform_core.account_lockouts where user_id = $1 for update";

const LOCKOUT_INSERT_ZERO_STMT: &str = "insert into platform_core.account_lockouts \
     (id, user_id, failure_count, window_started_at) values ($1, $2, 0, $3)";

const LOCKOUT_WRITE_STMT: &str = "update platform_core.account_lockouts \
     set failure_count = $2, window_started_at = $3, locked_until = $4, \
         last_failure_at = $5 \
     where id = $1";

const LOCKOUT_RESET_STMT: &str = "update platform_core.account_lockouts \
     set failure_count = 0, locked_until = null where user_id = $1";

const APPEND_ATTEMPT_STMT: &str = "insert into platform_core.login_attempts \
     (id, user_id, login_name_hash, outcome, client, source_addr, occurred_at) \
     values ($1, $2, decode($3, 'hex'), $4, $5, $6, $7)";

/// `platform_core.sessions` 的存取器。
pub struct PgSessionStore;

#[async_trait::async_trait]
impl SessionStore for PgSessionStore {
    async fn insert(&self, tx: &mut dyn Tx, new: NewSession) -> Result<uuid::Uuid, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let affected = pg
            .execute(
                INSERT_SESSION_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(new.user_id.as_uuid()),
                    DbValue::Uuid(new.user_device_row_id),
                    DbValue::Text(hex_encode(&new.token_hash)),
                    DbValue::Uuid(new.active_legal_entity_id.as_uuid()),
                    DbValue::Timestamp(new.issued_at),
                    DbValue::Timestamp(new.expires_at),
                    DbValue::Timestamp(new.idle_expires_at),
                    DbValue::Bool(new.is_breakglass),
                ],
            )
            .await?;
        ensure_one(affected, "会话插入未命中一行")?;
        Ok(id)
    }

    async fn find_active_by_digest(
        &self,
        tx: &mut dyn Tx,
        token_hash: &[u8],
    ) -> Result<Option<SessionRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                FIND_SESSION_BY_DIGEST_STMT,
                &[DbValue::Text(hex_encode(token_hash))],
            )
            .await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_session_row(row)?)),
            None => Ok(None),
        }
    }

    async fn list_active_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<SessionRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(LIST_USER_SESSIONS_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        rows.iter().map(|row| decode_session_row(row)).collect()
    }

    async fn revoke(
        &self,
        tx: &mut dyn Tx,
        session_id: uuid::Uuid,
        reason: &str,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                REVOKE_SESSION_STMT,
                &[DbValue::Uuid(session_id), DbValue::Text(reason.to_string())],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn revoke_all_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        reason: &str,
    ) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(
            REVOKE_ALL_SESSIONS_STMT,
            &[
                DbValue::Uuid(user_id.as_uuid()),
                DbValue::Text(reason.to_string()),
            ],
        )
        .await
    }

    async fn revoke_by_device(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        device_row_id: uuid::Uuid,
        reason: &str,
    ) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(
            REVOKE_SESSIONS_BY_DEVICE_STMT,
            &[
                DbValue::Uuid(user_id.as_uuid()),
                DbValue::Uuid(device_row_id),
                DbValue::Text(reason.to_string()),
            ],
        )
        .await
    }

    async fn extend_idle(
        &self,
        tx: &mut dyn Tx,
        session_ids: &[uuid::Uuid],
        idle_expires_at: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        if session_ids.is_empty() {
            return Ok(0);
        }
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let (stmt, params) = extend_idle_stmt(session_ids, idle_expires_at);
        pg.execute(&stmt, &params).await
    }

    async fn expire_overdue(&self, tx: &mut dyn Tx, now: DateTime<Utc>) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(EXPIRE_OVERDUE_SESSIONS_STMT, &[DbValue::Timestamp(now)])
            .await
    }
}

/// 续期语句按会话数动态编号占位符：`id in ($2..$n)`，$1 为新到期时刻。
fn extend_idle_stmt(
    session_ids: &[uuid::Uuid],
    idle_expires_at: DateTime<Utc>,
) -> (String, Vec<DbValue>) {
    let mut stmt = String::from(
        "update platform_core.sessions set idle_expires_at = $1, last_seen_at = now() \
         where revoked_at is null and id in (",
    );
    let mut params = vec![DbValue::Timestamp(idle_expires_at)];
    for (i, id) in session_ids.iter().enumerate() {
        if i > 0 {
            stmt.push_str(", ");
        }
        stmt.push_str(&format!("${}", i + 2));
        params.push(DbValue::Uuid(*id));
    }
    stmt.push(')');
    (stmt, params)
}

/// `platform_core.user_devices` 的存取器。
pub struct PgDeviceStore;

#[async_trait::async_trait]
impl DeviceStore for PgDeviceStore {
    async fn find_active(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        device_id: &str,
    ) -> Result<Option<DeviceRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                FIND_DEVICE_STMT,
                &[
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Text(device_id.to_string()),
                ],
            )
            .await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_device_row(row)?)),
            None => Ok(None),
        }
    }

    async fn list(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<DeviceRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(LIST_DEVICES_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        rows.iter().map(|row| decode_device_row(row)).collect()
    }

    async fn insert(&self, tx: &mut dyn Tx, new: NewDevice) -> Result<uuid::Uuid, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let affected = pg
            .execute(
                INSERT_DEVICE_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(new.user_id.as_uuid()),
                    DbValue::Text(new.device_id),
                    DbValue::Text(client_str(new.client).to_string()),
                    DbValue::Text(new.public_key.map(hex_of_pub).unwrap_or_default()),
                    opt_text(new.attestation_ref),
                    match new.restricted_legal_entity_id {
                        Some(le) => DbValue::Uuid(le.as_uuid()),
                        None => DbValue::Null,
                    },
                ],
            )
            .await?;
        ensure_one(affected, "设备插入未命中一行")?;
        Ok(id)
    }

    async fn revoke(&self, tx: &mut dyn Tx, device_row_id: uuid::Uuid) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(REVOKE_DEVICE_STMT, &[DbValue::Uuid(device_row_id)])
            .await?;
        Ok(affected == 1)
    }
}

fn hex_of_pub(raw: String) -> String {
    hex_encode(raw.as_bytes())
}

fn opt_text(v: Option<String>) -> DbValue {
    match v {
        Some(s) => DbValue::Text(s),
        None => DbValue::Null,
    }
}

/// `platform_core.account_lockouts` 的存取器。
pub struct PgLockoutStore;

#[async_trait::async_trait]
impl LockoutStore for PgLockoutStore {
    async fn lock_for_update(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<LockoutRow, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(LOCKOUT_FOR_UPDATE_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        if let Some(row) = rows.first() {
            return decode_lockout_row(row);
        }
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let now = Utc::now();
        let affected = pg
            .execute(
                LOCKOUT_INSERT_ZERO_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Timestamp(now),
                ],
            )
            .await?;
        ensure_one(affected, "锁定零值行插入未命中一行")?;
        Ok(LockoutRow {
            id,
            user_id,
            failure_count: 0,
            window_started_at: Some(now),
            locked_until: None,
            last_failure_at: None,
        })
    }

    /// 窗口外先清零再计一次；满 max_failures 即上锁并清零计数。
    /// 推进逻辑与用例层内存实现体逐字同款（锁定策略 5/15min/30min）。
    async fn record_failure(
        &self,
        tx: &mut dyn Tx,
        row: &LockoutRow,
        max_failures: u32,
        window_seconds: u64,
        duration_seconds: u64,
        now: DateTime<Utc>,
    ) -> Result<LockoutRow, AppError> {
        let mut next = row.clone();
        let in_window = next
            .window_started_at
            .map(|w| (now - w).num_seconds() < i64::try_from(window_seconds).unwrap_or(i64::MAX))
            .unwrap_or(false);
        if !in_window {
            next.failure_count = 0;
            next.window_started_at = Some(now);
        }
        next.failure_count += 1;
        next.last_failure_at = Some(now);
        if next.failure_count >= max_failures {
            next.locked_until =
                Some(now + Duration::seconds(i64::try_from(duration_seconds).unwrap_or(i64::MAX)));
            next.failure_count = 0;
        }
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                LOCKOUT_WRITE_STMT,
                &[
                    DbValue::Uuid(next.id),
                    DbValue::Int64(i64::from(next.failure_count)),
                    match next.window_started_at {
                        Some(t) => DbValue::Timestamp(t),
                        None => DbValue::Null,
                    },
                    match next.locked_until {
                        Some(t) => DbValue::Timestamp(t),
                        None => DbValue::Null,
                    },
                    DbValue::Timestamp(now),
                ],
            )
            .await?;
        ensure_one(affected, "锁定计数写回未命中一行")?;
        Ok(next)
    }

    async fn reset(&self, tx: &mut dyn Tx, user_id: Id<UserAccount>) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(LOCKOUT_RESET_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        Ok(())
    }
}

/// `platform_core.login_attempts` 的存取器（仅追加流水）。
pub struct PgLoginAttemptStore;

#[async_trait::async_trait]
impl LoginAttemptStore for PgLoginAttemptStore {
    async fn append(&self, tx: &mut dyn Tx, new: NewLoginAttempt) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                APPEND_ATTEMPT_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    match new.user_id {
                        Some(u) => DbValue::Uuid(u.as_uuid()),
                        None => DbValue::Null,
                    },
                    DbValue::Text(hex_encode(&new.login_name_hash)),
                    DbValue::Text(new.outcome.as_str().to_string()),
                    DbValue::Text(client_str(new.client).to_string()),
                    DbValue::Text(new.source_addr),
                    DbValue::Timestamp(new.occurred_at),
                ],
            )
            .await?;
        ensure_one(affected, "登录尝试流水追加未命中一行")
    }
}

fn decode_session_row(row: &[DbValue]) -> Result<SessionRow, AppError> {
    let token_hash = col_hex(row, 3)?.ok_or_else(|| shape_err("会话行的令牌摘要列缺失"))?;
    Ok(SessionRow {
        id: col_uuid(row, 0)?,
        user_id: Id::from_uuid(col_uuid(row, 1)?),
        user_device_row_id: col_uuid(row, 2)?,
        token_hash,
        active_legal_entity_id: Id::<LegalEntity>::from_uuid(col_uuid(row, 4)?),
        issued_at: col_ts(row, 5)?,
        expires_at: col_ts(row, 6)?,
        idle_expires_at: col_ts(row, 7)?,
        last_seen_at: col_ts(row, 8)?,
        revoked_at: col_opt_ts(row, 9)?,
        revoke_reason: col_opt_text(row, 10)?,
        is_breakglass: col_bool(row, 11)?,
    })
}

fn client_from_db(raw: &str) -> Result<ClientKind, AppError> {
    match raw {
        "win" => Ok(ClientKind::Win),
        "mac" => Ok(ClientKind::Mac),
        "ios" => Ok(ClientKind::Ios),
        "android" => Ok(ClientKind::Android),
        "portal" => Ok(ClientKind::Portal),
        "ops" => Ok(ClientKind::Ops),
        _ => Err(shape_err("设备行的 client 字面量非法")),
    }
}

fn decode_device_row(row: &[DbValue]) -> Result<DeviceRow, AppError> {
    let status =
        DeviceStatus::parse(&col_text(row, 7)?).ok_or_else(|| shape_err("设备状态字面量非法"))?;
    let public_key = col_hex(row, 4)?
        .map(|b| String::from_utf8(b).map_err(|_| shape_err("设备公开载体不是合法 UTF-8")))
        .transpose()?;
    Ok(DeviceRow {
        id: col_uuid(row, 0)?,
        user_id: Id::from_uuid(col_uuid(row, 1)?),
        device_id: col_text(row, 2)?,
        client: client_from_db(&col_text(row, 3)?)?,
        public_key,
        attestation_ref: col_opt_text(row, 5)?,
        restricted_legal_entity_id: col_opt_uuid(row, 6)?.map(Id::<LegalEntity>::from_uuid),
        status,
    })
}

fn decode_lockout_row(row: &[DbValue]) -> Result<LockoutRow, AppError> {
    Ok(LockoutRow {
        id: col_uuid(row, 0)?,
        user_id: Id::from_uuid(col_uuid(row, 1)?),
        failure_count: u32::try_from(col_i64(row, 2)?).map_err(|_| shape_err("失败计数越界"))?,
        window_started_at: col_opt_ts(row, 3)?,
        locked_until: col_opt_ts(row, 4)?,
        last_failure_at: col_opt_ts(row, 5)?,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::port::tx::{IsolationKind, TxId};
    use ep_platform_identity::types::LoginAttemptOutcome;

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

    fn session_row_values() -> Vec<DbValue> {
        let now = Utc::now();
        vec![
            DbValue::Uuid(uuid::Uuid::from_u128(11)),
            DbValue::Uuid(uuid::Uuid::from_u128(7)),
            DbValue::Uuid(uuid::Uuid::from_u128(9)),
            DbValue::Text(hex_encode(&[3u8; 32])),
            DbValue::Uuid(uuid::Uuid::from_u128(2)),
            DbValue::Timestamp(now),
            DbValue::Timestamp(now),
            DbValue::Timestamp(now),
            DbValue::Timestamp(now),
            DbValue::Null,
            DbValue::Null,
            DbValue::Bool(false),
        ]
    }

    #[test]
    fn session_insert_binds_digest_as_hex_and_reuses_issued_for_last_seen() {
        assert!(INSERT_SESSION_STMT.contains("decode($4, 'hex')"));
        assert!(
            INSERT_SESSION_STMT.contains("$6, $7, $8, $6"),
            "last_seen_at 与 issued_at 同参复用：{INSERT_SESSION_STMT}"
        );
        assert!(
            INSERT_SESSION_STMT.contains("'ops'"),
            "client 取库默认字面量"
        );
    }

    #[tokio::test]
    async fn session_decodes_twelve_column_projection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![session_row_values()]);
        let mut tx = tx_over(conn);
        let row = PgSessionStore
            .find_active_by_digest(&mut tx, &[3u8; 32])
            .await
            .expect("读取可完成")
            .expect("命中一行");
        assert_eq!(row.token_hash, vec![3u8; 32]);
        assert!(row.revoked_at.is_none());
        assert!(FIND_SESSION_BY_DIGEST_STMT.contains("idle_expires_at > now()"));
    }

    #[test]
    fn extend_idle_builds_numbered_placeholders() {
        let ids = [uuid::Uuid::from_u128(1), uuid::Uuid::from_u128(2)];
        let (stmt, params) = extend_idle_stmt(&ids, Utc::now());
        assert!(stmt.contains("id in ($2, $3)"), "占位符逐个编号：{stmt}");
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn expire_overdue_covers_both_deadlines() {
        assert!(
            EXPIRE_OVERDUE_SESSIONS_STMT.contains("expires_at <= $1 or idle_expires_at <= $1"),
            "绝对到期与空闲到期任一命中即撤"
        );
        assert!(EXPIRE_OVERDUE_SESSIONS_STMT.contains("'EXPIRED'"));
    }

    #[tokio::test]
    async fn lockout_failure_outside_window_resets_counter() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let now = Utc::now();
        let row = LockoutRow {
            id: uuid::Uuid::from_u128(5),
            user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
            failure_count: 4,
            window_started_at: Some(now - Duration::minutes(20)),
            locked_until: None,
            last_failure_at: None,
        };
        let got = PgLockoutStore
            .record_failure(&mut tx, &row, 5, 900, 1800, now)
            .await
            .expect("推进可完成");
        assert_eq!(got.failure_count, 1, "窗口外清零后重新计数");
        assert!(got.locked_until.is_none());
    }

    #[tokio::test]
    async fn lockout_fifth_failure_locks_and_resets_counter() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let now = Utc::now();
        let row = LockoutRow {
            id: uuid::Uuid::from_u128(5),
            user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
            failure_count: 4,
            window_started_at: Some(now - Duration::minutes(1)),
            locked_until: None,
            last_failure_at: None,
        };
        let got = PgLockoutStore
            .record_failure(&mut tx, &row, 5, 900, 1800, now)
            .await
            .expect("推进可完成");
        assert_eq!(got.failure_count, 0, "上锁即清零计数");
        assert_eq!(got.locked_until, Some(now + Duration::minutes(30)));
    }

    #[tokio::test]
    async fn attempt_append_hashes_login_name_binding() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        PgLoginAttemptStore
            .append(
                &mut tx,
                NewLoginAttempt {
                    user_id: None,
                    login_name_hash: vec![9; 32],
                    outcome: LoginAttemptOutcome::CredentialInvalid,
                    client: ClientKind::Win,
                    source_addr: "127.0.0.1".into(),
                    occurred_at: Utc::now(),
                },
            )
            .await
            .expect("追加可完成");
        assert!(APPEND_ATTEMPT_STMT.contains("decode($3, 'hex')"));
    }
}
