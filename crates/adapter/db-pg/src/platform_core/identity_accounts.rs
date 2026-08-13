//! `platform_core` 身份域账号面 SQL 实现体：user_accounts、
//! user_credentials、user_password_history 三张表（迁移
//! V202610120900/0905/0910）。端口 trait 在 ep-platform-identity，
//! 本文件只做事务内的读写接通，不另立业务判等。
//!
//! bytea 列一律以 hex 文本进出：写入侧 `decode($n, 'hex')`，
//! 读取侧 `encode(col, 'hex')`（DbValue 无字节变体）。

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_platform_authz::types::{hex_decode, hex_encode};
use ep_platform_identity::ports::{
    AccountStore, CredentialStore, NewAccount, NewCredential, PasswordHistoryStore,
};
use ep_platform_identity::types::{
    AccountKind, AccountStatus, CredentialKind, CredentialRow, CredentialStatus, UserAccountRow,
};

use crate::conn::DbValue;
use crate::tx::PgTx;

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "身份域读写必须在 PostgreSQL 事务内执行",
        )
    })
}

const SELECT_ACCOUNT_BY_LOGIN_STMT: &str =
    "select id, account_kind, login_name, employee_no, display_name, home_legal_entity_id, \
     status, clearance_level::int8, security_level::int8, is_mfa_required, created_at \
     from platform_core.user_accounts where login_name = $1";

const SELECT_ACCOUNT_BY_ID_STMT: &str =
    "select id, account_kind, login_name, employee_no, display_name, home_legal_entity_id, \
     status, clearance_level::int8, security_level::int8, is_mfa_required, created_at \
     from platform_core.user_accounts where id = $1";

/// 建号一律落 UNACTIVATED，启用走生命周期端点的状态迁移。
const INSERT_ACCOUNT_STMT: &str = "insert into platform_core.user_accounts \
     (id, account_kind, login_name, employee_no, display_name, home_legal_entity_id, \
      clearance_level, status, is_mfa_required) \
     values ($1, $2, $3, $4, $5, $6, $7::smallint, 'UNACTIVATED', $8)";

const TRANSITION_STATUS_STMT: &str = "update platform_core.user_accounts \
     set status = $3, updated_at = now() where id = $1 and status = $2";

const TRANSITION_STATUS_ANY_STMT: &str = "update platform_core.user_accounts \
     set status = $2, updated_at = now() where id = $1";

const SELECT_CREDENTIALS_ACTIVE_STMT: &str =
    "select id, user_id, credential_kind, verifier, encode(public_key, 'hex'), \
     encode(credential_handle, 'hex'), secret_ref, sign_count, status, \
     security_level::int8, created_at \
     from platform_core.user_credentials where user_id = $1 and status = 'ACTIVE' \
     order by created_at";

const SELECT_CREDENTIAL_OF_KIND_STMT: &str =
    "select id, user_id, credential_kind, verifier, encode(public_key, 'hex'), \
     encode(credential_handle, 'hex'), secret_ref, sign_count, status, \
     security_level::int8, created_at \
     from platform_core.user_credentials \
     where user_id = $1 and credential_kind = $2 and status = 'ACTIVE' \
     order by created_at";

const INSERT_CREDENTIAL_STMT: &str = "insert into platform_core.user_credentials \
     (id, user_id, credential_kind, verifier, public_key, credential_handle, secret_ref, \
      status) \
     values ($1, $2, $3, $4, decode($5, 'hex'), decode($6, 'hex'), $7, 'ACTIVE')";

const SET_CREDENTIAL_STATUS_STMT: &str = "update platform_core.user_credentials \
     set status = $2, \
         revoked_at = case when $2::text = 'REVOKED' \
                           then coalesce(revoked_at, now()) else revoked_at end \
     where id = $1";

const REVOKE_ALL_CREDENTIALS_STMT: &str = "update platform_core.user_credentials \
     set status = 'REVOKED', revoked_at = now() \
     where user_id = $1 and status = 'ACTIVE'";

/// 仅向前：新计数不大于现存计数时不生效（affected 0）。
const BUMP_SIGN_COUNT_STMT: &str = "update platform_core.user_credentials \
     set sign_count = $2 where id = $1 and sign_count < $2";

const SELECT_RECENT_VERIFIERS_STMT: &str = "select verifier \
     from platform_core.user_password_history \
     where user_id = $1 order by created_at desc limit $2";

const APPEND_PASSWORD_HISTORY_STMT: &str = "insert into platform_core.user_password_history \
     (id, user_id, verifier, created_by) values ($1, $2, $3, $4)";

/// `platform_core.user_accounts` 的存取器。
pub struct PgAccountStore;

#[async_trait::async_trait]
impl AccountStore for PgAccountStore {
    async fn find_by_login_name(
        &self,
        tx: &mut dyn Tx,
        login_name: &str,
    ) -> Result<Option<UserAccountRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                SELECT_ACCOUNT_BY_LOGIN_STMT,
                &[DbValue::Text(login_name.to_string())],
            )
            .await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_account_row(row)?)),
            None => Ok(None),
        }
    }

    async fn get(
        &self,
        tx: &mut dyn Tx,
        id: Id<UserAccount>,
    ) -> Result<Option<UserAccountRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(SELECT_ACCOUNT_BY_ID_STMT, &[DbValue::Uuid(id.as_uuid())])
            .await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_account_row(row)?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, tx: &mut dyn Tx, new: NewAccount) -> Result<Id<UserAccount>, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let affected = pg
            .execute(
                INSERT_ACCOUNT_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Text(new.account_kind.as_str().to_string()),
                    DbValue::Text(new.login_name),
                    opt_text(new.employee_no),
                    DbValue::Text(new.display_name),
                    DbValue::Uuid(new.home_legal_entity_id.as_uuid()),
                    DbValue::Int64(new.clearance_level as i64),
                    DbValue::Bool(new.is_mfa_required),
                ],
            )
            .await?;
        ensure_one(affected, "账号插入未命中一行")?;
        Ok(Id::from_uuid(id))
    }

    async fn transition_status(
        &self,
        tx: &mut dyn Tx,
        id: Id<UserAccount>,
        from: Option<AccountStatus>,
        to: AccountStatus,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = match from {
            Some(from) => {
                pg.execute(
                    TRANSITION_STATUS_STMT,
                    &[
                        DbValue::Uuid(id.as_uuid()),
                        DbValue::Text(from.as_str().to_string()),
                        DbValue::Text(to.as_str().to_string()),
                    ],
                )
                .await?
            }
            None => {
                pg.execute(
                    TRANSITION_STATUS_ANY_STMT,
                    &[
                        DbValue::Uuid(id.as_uuid()),
                        DbValue::Text(to.as_str().to_string()),
                    ],
                )
                .await?
            }
        };
        Ok(affected == 1)
    }
}

/// `platform_core.user_credentials` 的存取器。
pub struct PgCredentialStore;

#[async_trait::async_trait]
impl CredentialStore for PgCredentialStore {
    async fn list_active(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<CredentialRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                SELECT_CREDENTIALS_ACTIVE_STMT,
                &[DbValue::Uuid(user_id.as_uuid())],
            )
            .await?;
        rows.iter().map(|row| decode_credential_row(row)).collect()
    }

    async fn active_of_kind(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        kind: CredentialKind,
    ) -> Result<Option<CredentialRow>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                SELECT_CREDENTIAL_OF_KIND_STMT,
                &[
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Text(kind.as_str().to_string()),
                ],
            )
            .await?;
        match rows.first() {
            Some(row) => Ok(Some(decode_credential_row(row)?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, tx: &mut dyn Tx, new: NewCredential) -> Result<uuid::Uuid, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let id = uuid::Uuid::now_v7();
        let affected = pg
            .execute(
                INSERT_CREDENTIAL_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(new.user_id.as_uuid()),
                    DbValue::Text(new.credential_kind.as_str().to_string()),
                    opt_text(new.verifier),
                    DbValue::Text(new.public_key.map(hex_of_pub).unwrap_or_default()),
                    DbValue::Text(new.credential_handle.map(hex_of_pub).unwrap_or_default()),
                    opt_text(new.secret_ref),
                ],
            )
            .await?;
        ensure_one(affected, "凭据插入未命中一行")?;
        Ok(id)
    }

    async fn set_status(
        &self,
        tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        status: CredentialStatus,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                SET_CREDENTIAL_STATUS_STMT,
                &[
                    DbValue::Uuid(credential_id),
                    DbValue::Text(status.as_str().to_string()),
                ],
            )
            .await?;
        Ok(affected == 1)
    }

    async fn revoke_all_for_user(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        pg.execute(
            REVOKE_ALL_CREDENTIALS_STMT,
            &[DbValue::Uuid(user_id.as_uuid())],
        )
        .await
    }

    async fn bump_sign_count(
        &self,
        tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        new_count: i64,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                BUMP_SIGN_COUNT_STMT,
                &[DbValue::Uuid(credential_id), DbValue::Int64(new_count)],
            )
            .await?;
        Ok(affected == 1)
    }
}

/// `platform_core.user_password_history` 的存取器（仅追加表）。
pub struct PgPasswordHistoryStore;

#[async_trait::async_trait]
impl PasswordHistoryStore for PgPasswordHistoryStore {
    async fn recent_verifiers(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        n: usize,
    ) -> Result<Vec<String>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                SELECT_RECENT_VERIFIERS_STMT,
                &[
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Int64(i64::try_from(n).unwrap_or(i64::MAX)),
                ],
            )
            .await?;
        rows.iter()
            .map(|row| match row.first() {
                Some(DbValue::Text(s)) => Ok(s.clone()),
                _ => Err(shape_err("口令历史行的 verifier 列形态不符")),
            })
            .collect()
    }

    async fn append(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        verifier: String,
        created_by: Id<UserAccount>,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let affected = pg
            .execute(
                APPEND_PASSWORD_HISTORY_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Text(verifier),
                    DbValue::Uuid(created_by.as_uuid()),
                ],
            )
            .await?;
        ensure_one(affected, "口令历史追加未命中一行")
    }
}

/// 公开载体（public_key/credential_handle）以原文 hex 落 bytea。
fn hex_of_pub(raw: String) -> String {
    hex_encode(raw.as_bytes())
}

pub(crate) fn opt_text(v: Option<String>) -> DbValue {
    match v {
        Some(s) => DbValue::Text(s),
        None => DbValue::Null,
    }
}

pub(crate) fn ensure_one(affected: u64, msg: &'static str) -> Result<(), AppError> {
    if affected != 1 {
        return Err(AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, msg));
    }
    Ok(())
}

pub(crate) fn shape_err(msg: &'static str) -> AppError {
    AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, msg)
}

pub(crate) fn col_text(row: &[DbValue], idx: usize) -> Result<String, AppError> {
    match row.get(idx) {
        Some(DbValue::Text(s)) => Ok(s.clone()),
        _ => Err(shape_err("身份域行的文本列形态不符")),
    }
}

pub(crate) fn col_opt_text(row: &[DbValue], idx: usize) -> Result<Option<String>, AppError> {
    match row.get(idx) {
        Some(DbValue::Text(s)) => Ok(Some(s.clone())),
        Some(DbValue::Null) | None => Ok(None),
        _ => Err(shape_err("身份域行的可空文本列形态不符")),
    }
}

pub(crate) fn col_uuid(row: &[DbValue], idx: usize) -> Result<uuid::Uuid, AppError> {
    match row.get(idx) {
        Some(DbValue::Uuid(u)) => Ok(*u),
        _ => Err(shape_err("身份域行的 uuid 列形态不符")),
    }
}

pub(crate) fn col_opt_uuid(row: &[DbValue], idx: usize) -> Result<Option<uuid::Uuid>, AppError> {
    match row.get(idx) {
        Some(DbValue::Uuid(u)) => Ok(Some(*u)),
        Some(DbValue::Null) | None => Ok(None),
        _ => Err(shape_err("身份域行的可空 uuid 列形态不符")),
    }
}

pub(crate) fn col_ts(
    row: &[DbValue],
    idx: usize,
) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    match row.get(idx) {
        Some(DbValue::Timestamp(t)) => Ok(*t),
        _ => Err(shape_err("身份域行的时间列形态不符")),
    }
}

pub(crate) fn col_opt_ts(
    row: &[DbValue],
    idx: usize,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, AppError> {
    match row.get(idx) {
        Some(DbValue::Timestamp(t)) => Ok(Some(*t)),
        Some(DbValue::Null) | None => Ok(None),
        _ => Err(shape_err("身份域行的可空时间列形态不符")),
    }
}

pub(crate) fn col_i64(row: &[DbValue], idx: usize) -> Result<i64, AppError> {
    match row.get(idx) {
        Some(DbValue::Int64(n)) => Ok(*n),
        _ => Err(shape_err("身份域行的整型列形态不符")),
    }
}

pub(crate) fn col_bool(row: &[DbValue], idx: usize) -> Result<bool, AppError> {
    match row.get(idx) {
        Some(DbValue::Bool(b)) => Ok(*b),
        _ => Err(shape_err("身份域行的布尔列形态不符")),
    }
}

/// hex 文本列还原字节（encode(col,'hex') 投影），空串视为 NULL。
pub(crate) fn col_hex(row: &[DbValue], idx: usize) -> Result<Option<Vec<u8>>, AppError> {
    match row.get(idx) {
        Some(DbValue::Null) | None => Ok(None),
        Some(DbValue::Text(s)) if s.is_empty() => Ok(None),
        Some(DbValue::Text(s)) => hex_decode(s)
            .map(Some)
            .ok_or_else(|| shape_err("hex 列解码失败")),
        _ => Err(shape_err("身份域行的 hex 列形态不符")),
    }
}

fn decode_account_row(row: &[DbValue]) -> Result<UserAccountRow, AppError> {
    let kind =
        AccountKind::parse(&col_text(row, 1)?).ok_or_else(|| shape_err("账号类型字面量非法"))?;
    let status =
        AccountStatus::parse(&col_text(row, 6)?).ok_or_else(|| shape_err("账号状态字面量非法"))?;
    Ok(UserAccountRow {
        id: Id::from_uuid(col_uuid(row, 0)?),
        account_kind: kind,
        login_name: col_text(row, 2)?,
        employee_no: col_opt_text(row, 3)?,
        display_name: col_text(row, 4)?,
        home_legal_entity_id: Id::<LegalEntity>::from_uuid(col_uuid(row, 5)?),
        status,
        clearance_level: u8::try_from(col_i64(row, 7)?).map_err(|_| shape_err("密级越界"))?,
        security_level: u8::try_from(col_i64(row, 8)?).map_err(|_| shape_err("安全级越界"))?,
        is_mfa_required: col_bool(row, 9)?,
        created_at: col_ts(row, 10)?,
    })
}

fn decode_credential_row(row: &[DbValue]) -> Result<CredentialRow, AppError> {
    let kind =
        CredentialKind::parse(&col_text(row, 2)?).ok_or_else(|| shape_err("凭据类型字面量非法"))?;
    let status = CredentialStatus::parse(&col_text(row, 8)?)
        .ok_or_else(|| shape_err("凭据状态字面量非法"))?;
    Ok(CredentialRow {
        id: col_uuid(row, 0)?,
        user_id: Id::from_uuid(col_uuid(row, 1)?),
        credential_kind: kind,
        verifier: col_opt_text(row, 3)?,
        public_key: col_hex(row, 4)?.map(bytes_to_string).transpose()?,
        credential_handle: col_hex(row, 5)?.map(bytes_to_string).transpose()?,
        secret_ref: col_opt_text(row, 6)?,
        sign_count: col_i64(row, 7)?,
        status,
        security_level: u8::try_from(col_i64(row, 9)?).map_err(|_| shape_err("安全级越界"))?,
        created_at: col_ts(row, 10)?,
    })
}

fn bytes_to_string(bytes: Vec<u8>) -> Result<String, AppError> {
    String::from_utf8(bytes).map_err(|_| shape_err("公开载体不是合法 UTF-8"))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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

    fn account_row_values() -> Vec<DbValue> {
        vec![
            DbValue::Uuid(uuid::Uuid::from_u128(7)),
            DbValue::Text("EMPLOYEE".to_string()),
            DbValue::Text("alice".to_string()),
            DbValue::Null,
            DbValue::Text("Alice".to_string()),
            DbValue::Uuid(uuid::Uuid::from_u128(2)),
            DbValue::Text("ACTIVE".to_string()),
            DbValue::Int64(20),
            DbValue::Int64(30),
            DbValue::Bool(false),
            DbValue::Timestamp(chrono::Utc::now()),
        ]
    }

    #[tokio::test]
    async fn insert_account_builds_unactivated_row() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let id = PgAccountStore
            .insert(
                &mut tx,
                NewAccount {
                    account_kind: AccountKind::Employee,
                    login_name: "alice".into(),
                    employee_no: None,
                    display_name: "Alice".into(),
                    home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
                    clearance_level: 20,
                    is_mfa_required: false,
                },
            )
            .await
            .expect("建号可完成");
        assert_ne!(id.as_uuid(), uuid::Uuid::nil());
        assert!(tx.has_side_effect());
        assert!(
            INSERT_ACCOUNT_STMT.contains("'UNACTIVATED'"),
            "建号一律未启用态"
        );
    }

    #[tokio::test]
    async fn find_account_decodes_the_eleven_column_projection() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![account_row_values()]);
        let mut tx = tx_over(conn);
        let row = PgAccountStore
            .find_by_login_name(&mut tx, "alice")
            .await
            .expect("读取可完成")
            .expect("命中一行");
        assert_eq!(row.login_name, "alice");
        assert_eq!(row.clearance_level, 20);
        assert!(matches!(row.account_kind, AccountKind::Employee));
    }

    #[test]
    fn transition_statements_follow_cas_semantics() {
        assert!(
            TRANSITION_STATUS_STMT.contains("where id = $1 and status = $2"),
            "带起态限定即 CAS"
        );
        assert!(
            !TRANSITION_STATUS_ANY_STMT.contains("and status"),
            "无起态限定不附加状态条件"
        );
    }

    #[tokio::test]
    async fn credential_insert_hex_encodes_public_material() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1;
        let mut tx = tx_over(conn);
        let id = PgCredentialStore
            .insert(
                &mut tx,
                NewCredential {
                    user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
                    credential_kind: CredentialKind::WebauthnPlatform,
                    verifier: None,
                    public_key: Some("pk".to_string()),
                    credential_handle: Some("ch".to_string()),
                    secret_ref: None,
                },
            )
            .await
            .expect("建凭据可完成");
        assert_ne!(id, uuid::Uuid::nil());
        assert!(
            INSERT_CREDENTIAL_STMT.contains("decode($5, 'hex')"),
            "bytea 以 hex 绑定显式解码"
        );
    }

    #[test]
    fn bump_sign_count_is_monotonic_forward_only() {
        assert!(
            BUMP_SIGN_COUNT_STMT.contains("sign_count < $2"),
            "仅向前推进签名计数：{BUMP_SIGN_COUNT_STMT}"
        );
    }

    #[tokio::test]
    async fn recent_verifiers_limits_by_generation() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![
            vec![DbValue::Text("$argon2id$v2".to_string())],
            vec![DbValue::Text("$argon2id$v1".to_string())],
        ]);
        let mut tx = tx_over(conn);
        let got = PgPasswordHistoryStore
            .recent_verifiers(&mut tx, Id::from_uuid(uuid::Uuid::from_u128(7)), 5)
            .await
            .expect("读取可完成");
        assert_eq!(got.len(), 2);
        assert!(SELECT_RECENT_VERIFIERS_STMT.contains("order by created_at desc limit $2"));
    }
}
