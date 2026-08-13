//! 测试基座：内存态端口实现与伪事务。
//!
//! 事务闭包永远提交（与 PgUnitOfWork 的 Ok 即提交一致），
//! 用于断言「失败路径 Ok(LoginOutcome) 提交不回滚」语义。

#![cfg(test)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::kms::{
    Aad, BlindIndex, CipherEnvelope, KeyDomainId, KeyPurpose, KeyRef, KmsBackend, Signature,
};
use ep_foundation::port::tx::{BoxFuture, IsolationKind, SnapshotCtx, Tx, TxId, UnitOfWork};
use ep_foundation::security::context::DutyClass;
use ep_foundation::security::context::SecurityContext;

use crate::ports::{
    AccountStore, AuditRecorder, CredentialStore, DeviceStore, LockoutStore, LoginAttemptStore,
    NewAccount, NewCredential, NewDevice, NewLoginAttempt, NewSession, OpsAlertRecorder,
    PasswordHistoryStore, PendingEventRecorder, SessionStore,
};
use crate::types::{
    AccountStatus, BreakglassRow, CredentialKind, CredentialRow, CredentialStatus, DeviceRow,
    DeviceStatus, LockoutRow, LoginAttemptOutcome, SessionRow, UserAccountRow,
};

/// 共享内存态：全部 Fake 实现体经 Arc 克隆消费同一份。
#[derive(Default)]
pub struct MemState {
    pub accounts: Vec<UserAccountRow>,
    pub credentials: Vec<CredentialRow>,
    pub password_history: Vec<(Id<ep_foundation::id::marker::UserAccount>, String)>,
    pub devices: Vec<DeviceRow>,
    pub sessions: Vec<SessionRow>,
    pub lockouts: Vec<LockoutRow>,
    pub attempts: Vec<NewLoginAttempt>,
    pub breakglass: Vec<BreakglassRow>,
    pub duties: Vec<DutyClass>,
    pub high_risk: bool,
    pub legal_entities: Vec<Id<LegalEntity>>,
    pub open_high_risk_requests: u64,
    pub installed_les: Vec<Id<LegalEntity>>,
    pub granted_les: Vec<Id<LegalEntity>>,
    pub audits: Vec<(String, String)>,
    pub pending_events: Vec<(String, String)>,
    pub ops_alerts: Vec<(String, String)>,
    pub challenges_expired_calls: u32,
}

pub type MemHandle = Arc<Mutex<MemState>>;

pub fn mem() -> MemHandle {
    Arc::new(Mutex::new(MemState::default()))
}

pub fn lock(h: &MemHandle) -> std::sync::MutexGuard<'_, MemState> {
    h.lock().unwrap_or_else(|p| p.into_inner())
}

/// 伪事务句柄：只承载法人上下文。
pub struct FakeTx {
    le: Id<LegalEntity>,
}

impl Tx for FakeTx {
    fn tx_id(&self) -> TxId {
        TxId(uuid::Uuid::nil())
    }
    fn isolation(&self) -> IsolationKind {
        IsolationKind::ReadCommitted
    }
    fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.le
    }
    fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
        self
    }
}

/// 内存 UnitOfWork：Ok 即提交（含失败路径 Ok 变体），Err 回滚。
pub struct InMemoryUow;

#[async_trait::async_trait]
impl UnitOfWork for InMemoryUow {
    async fn transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        let mut tx = FakeTx {
            le: ctx.legal_entity_id,
        };
        body(&mut tx).await
    }

    async fn snapshot_transact<T, F>(&self, _ctx: &SecurityContext, _body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'s> FnOnce(&'s dyn SnapshotCtx) -> BoxFuture<'s, Result<T, AppError>>
            + Send
            + 'static,
    {
        Err(AppError::new(
            ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
            "内存 UoW 不支持快照事务",
        ))
    }
}

/// 账号存储：读路径直接查共享态，写路径支持建号与状态迁移。
pub struct MemAccountStore(pub MemHandle);

#[async_trait::async_trait]
impl AccountStore for MemAccountStore {
    async fn find_by_login_name(
        &self,
        _tx: &mut dyn Tx,
        login_name: &str,
    ) -> Result<Option<UserAccountRow>, AppError> {
        Ok(lock(&self.0)
            .accounts
            .iter()
            .find(|a| a.login_name == login_name)
            .cloned())
    }
    async fn get(
        &self,
        _tx: &mut dyn Tx,
        id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<Option<UserAccountRow>, AppError> {
        Ok(lock(&self.0).accounts.iter().find(|a| a.id == id).cloned())
    }
    async fn insert(
        &self,
        _tx: &mut dyn Tx,
        new: NewAccount,
    ) -> Result<Id<ep_foundation::id::marker::UserAccount>, AppError> {
        let id = Id::from_uuid(uuid::Uuid::now_v7());
        lock(&self.0).accounts.push(UserAccountRow {
            id,
            account_kind: new.account_kind,
            login_name: new.login_name,
            employee_no: new.employee_no,
            display_name: new.display_name,
            home_legal_entity_id: new.home_legal_entity_id,
            status: AccountStatus::Unactivated,
            clearance_level: new.clearance_level,
            security_level: new.clearance_level,
            is_mfa_required: new.is_mfa_required,
            created_at: Utc::now(),
        });
        Ok(id)
    }
    async fn transition_status(
        &self,
        _tx: &mut dyn Tx,
        id: Id<ep_foundation::id::marker::UserAccount>,
        from: Option<AccountStatus>,
        to: AccountStatus,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(acc) = st.accounts.iter_mut().find(|a| a.id == id) else {
            return Ok(false);
        };
        if let Some(f) = from {
            if acc.status != f {
                return Ok(false);
            }
        }
        acc.status = to;
        Ok(true)
    }
}

pub struct MemCredentialStore(pub MemHandle);

#[async_trait::async_trait]
impl CredentialStore for MemCredentialStore {
    async fn list_active(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<Vec<CredentialRow>, AppError> {
        Ok(lock(&self.0)
            .credentials
            .iter()
            .filter(|c| c.user_id == user_id && c.status == CredentialStatus::Active)
            .cloned()
            .collect())
    }
    async fn active_of_kind(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        kind: CredentialKind,
    ) -> Result<Option<CredentialRow>, AppError> {
        Ok(lock(&self.0)
            .credentials
            .iter()
            .find(|c| {
                c.user_id == user_id
                    && c.credential_kind == kind
                    && c.status == CredentialStatus::Active
            })
            .cloned())
    }
    async fn insert(&self, _tx: &mut dyn Tx, new: NewCredential) -> Result<uuid::Uuid, AppError> {
        let id = uuid::Uuid::now_v7();
        lock(&self.0).credentials.push(CredentialRow {
            id,
            user_id: new.user_id,
            credential_kind: new.credential_kind,
            verifier: new.verifier,
            public_key: new.public_key,
            credential_handle: new.credential_handle,
            secret_ref: new.secret_ref,
            sign_count: 0,
            status: CredentialStatus::Active,
            security_level: 0,
            created_at: Utc::now(),
        });
        Ok(id)
    }
    async fn set_status(
        &self,
        _tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        status: CredentialStatus,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(c) = st.credentials.iter_mut().find(|c| c.id == credential_id) else {
            return Ok(false);
        };
        c.status = status;
        Ok(true)
    }
    async fn revoke_all_for_user(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<u64, AppError> {
        let mut st = lock(&self.0);
        let mut n = 0u64;
        for c in st.credentials.iter_mut().filter(|c| c.user_id == user_id) {
            if c.status == CredentialStatus::Active {
                c.status = CredentialStatus::Revoked;
                n += 1;
            }
        }
        Ok(n)
    }
    async fn bump_sign_count(
        &self,
        _tx: &mut dyn Tx,
        credential_id: uuid::Uuid,
        new_count: i64,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(c) = st.credentials.iter_mut().find(|c| c.id == credential_id) else {
            return Ok(false);
        };
        if new_count <= c.sign_count {
            return Ok(false);
        }
        c.sign_count = new_count;
        Ok(true)
    }
}

pub struct MemPasswordHistoryStore(pub MemHandle);

#[async_trait::async_trait]
impl PasswordHistoryStore for MemPasswordHistoryStore {
    async fn recent_verifiers(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        n: usize,
    ) -> Result<Vec<String>, AppError> {
        Ok(lock(&self.0)
            .password_history
            .iter()
            .rev()
            .filter(|(u, _)| *u == user_id)
            .take(n)
            .map(|(_, v)| v.clone())
            .collect())
    }
    async fn append(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        verifier: String,
        _created_by: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<(), AppError> {
        lock(&self.0).password_history.push((user_id, verifier));
        Ok(())
    }
}

pub struct MemDeviceStore(pub MemHandle);

#[async_trait::async_trait]
impl DeviceStore for MemDeviceStore {
    async fn find_active(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        device_id: &str,
    ) -> Result<Option<DeviceRow>, AppError> {
        Ok(lock(&self.0)
            .devices
            .iter()
            .find(|d| {
                d.user_id == user_id && d.device_id == device_id && d.status == DeviceStatus::Active
            })
            .cloned())
    }
    async fn list(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<Vec<DeviceRow>, AppError> {
        Ok(lock(&self.0)
            .devices
            .iter()
            .filter(|d| d.user_id == user_id)
            .cloned()
            .collect())
    }
    async fn insert(&self, _tx: &mut dyn Tx, new: NewDevice) -> Result<uuid::Uuid, AppError> {
        let id = uuid::Uuid::now_v7();
        lock(&self.0).devices.push(DeviceRow {
            id,
            user_id: new.user_id,
            device_id: new.device_id,
            client: new.client,
            public_key: new.public_key,
            attestation_ref: new.attestation_ref,
            restricted_legal_entity_id: new.restricted_legal_entity_id,
            status: DeviceStatus::Active,
        });
        Ok(id)
    }
    async fn revoke(&self, _tx: &mut dyn Tx, device_row_id: uuid::Uuid) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(d) = st.devices.iter_mut().find(|d| d.id == device_row_id) else {
            return Ok(false);
        };
        d.status = DeviceStatus::Revoked;
        Ok(true)
    }
}

pub struct MemSessionStore(pub MemHandle);

#[async_trait::async_trait]
impl SessionStore for MemSessionStore {
    async fn insert(&self, _tx: &mut dyn Tx, new: NewSession) -> Result<uuid::Uuid, AppError> {
        let id = uuid::Uuid::now_v7();
        lock(&self.0).sessions.push(SessionRow {
            id,
            user_id: new.user_id,
            user_device_row_id: new.user_device_row_id,
            token_hash: new.token_hash,
            active_legal_entity_id: new.active_legal_entity_id,
            issued_at: new.issued_at,
            expires_at: new.expires_at,
            idle_expires_at: new.idle_expires_at,
            last_seen_at: new.issued_at,
            revoked_at: None,
            revoke_reason: None,
            is_breakglass: new.is_breakglass,
        });
        Ok(id)
    }
    async fn find_active_by_digest(
        &self,
        _tx: &mut dyn Tx,
        token_hash: &[u8],
    ) -> Result<Option<SessionRow>, AppError> {
        let now = Utc::now();
        Ok(lock(&self.0)
            .sessions
            .iter()
            .find(|s| s.token_hash == token_hash && s.revoked_at.is_none() && s.expires_at > now)
            .cloned())
    }
    async fn list_active_for_user(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<Vec<SessionRow>, AppError> {
        let mut rows: Vec<SessionRow> = lock(&self.0)
            .sessions
            .iter()
            .filter(|s| s.user_id == user_id && s.revoked_at.is_none())
            .cloned()
            .collect();
        rows.sort_by_key(|s| s.issued_at);
        Ok(rows)
    }
    async fn revoke(
        &self,
        _tx: &mut dyn Tx,
        session_id: uuid::Uuid,
        reason: &str,
    ) -> Result<bool, AppError> {
        let mut st = lock(&self.0);
        let Some(s) = st.sessions.iter_mut().find(|s| s.id == session_id) else {
            return Ok(false);
        };
        if s.revoked_at.is_some() {
            return Ok(false);
        }
        s.revoked_at = Some(Utc::now());
        s.revoke_reason = Some(reason.to_string());
        Ok(true)
    }
    async fn revoke_all_for_user(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        reason: &str,
    ) -> Result<u64, AppError> {
        let mut st = lock(&self.0);
        let mut n = 0u64;
        for s in st.sessions.iter_mut().filter(|s| s.user_id == user_id) {
            if s.revoked_at.is_none() {
                s.revoked_at = Some(Utc::now());
                s.revoke_reason = Some(reason.to_string());
                n += 1;
            }
        }
        Ok(n)
    }
    async fn revoke_by_device(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
        device_row_id: uuid::Uuid,
        reason: &str,
    ) -> Result<u64, AppError> {
        let mut st = lock(&self.0);
        let mut n = 0u64;
        for s in st
            .sessions
            .iter_mut()
            .filter(|s| s.user_id == user_id && s.user_device_row_id == device_row_id)
        {
            if s.revoked_at.is_none() {
                s.revoked_at = Some(Utc::now());
                s.revoke_reason = Some(reason.to_string());
                n += 1;
            }
        }
        Ok(n)
    }
    async fn extend_idle(
        &self,
        _tx: &mut dyn Tx,
        session_ids: &[uuid::Uuid],
        idle_expires_at: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        let mut st = lock(&self.0);
        let mut n = 0u64;
        for s in st.sessions.iter_mut() {
            if session_ids.contains(&s.id) && s.revoked_at.is_none() {
                s.idle_expires_at = idle_expires_at;
                s.last_seen_at = Utc::now();
                n += 1;
            }
        }
        Ok(n)
    }
    async fn expire_overdue(&self, _tx: &mut dyn Tx, now: DateTime<Utc>) -> Result<u64, AppError> {
        let mut st = lock(&self.0);
        let mut n = 0u64;
        for s in st.sessions.iter_mut() {
            if s.revoked_at.is_none() && (s.expires_at <= now || s.idle_expires_at <= now) {
                s.revoked_at = Some(now);
                s.revoke_reason = Some("EXPIRED".to_string());
                n += 1;
            }
        }
        Ok(n)
    }
}

pub struct MemLockoutStore(pub MemHandle);

#[async_trait::async_trait]
impl LockoutStore for MemLockoutStore {
    async fn lock_for_update(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<LockoutRow, AppError> {
        let mut st = lock(&self.0);
        if let Some(r) = st.lockouts.iter().find(|r| r.user_id == user_id) {
            return Ok(r.clone());
        }
        let row = LockoutRow {
            id: uuid::Uuid::now_v7(),
            user_id,
            failure_count: 0,
            window_started_at: None,
            locked_until: None,
            last_failure_at: None,
        };
        st.lockouts.push(row.clone());
        Ok(row)
    }
    async fn record_failure(
        &self,
        _tx: &mut dyn Tx,
        row: &LockoutRow,
        max_failures: u32,
        window_seconds: u64,
        duration_seconds: u64,
        now: DateTime<Utc>,
    ) -> Result<LockoutRow, AppError> {
        let mut st = lock(&self.0);
        let Some(r) = st.lockouts.iter_mut().find(|r| r.id == row.id) else {
            return Err(AppError::new(
                ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
                "锁定行丢失",
            ));
        };
        let in_window = r
            .window_started_at
            .map(|w| (now - w).num_seconds() < i64::try_from(window_seconds).unwrap_or(i64::MAX))
            .unwrap_or(false);
        if !in_window {
            r.failure_count = 0;
            r.window_started_at = Some(now);
        }
        r.failure_count += 1;
        r.last_failure_at = Some(now);
        if r.failure_count >= max_failures {
            r.locked_until = Some(
                now + chrono::Duration::seconds(
                    i64::try_from(duration_seconds).unwrap_or(i64::MAX),
                ),
            );
            r.failure_count = 0;
        }
        Ok(r.clone())
    }
    async fn reset(
        &self,
        _tx: &mut dyn Tx,
        user_id: Id<ep_foundation::id::marker::UserAccount>,
    ) -> Result<(), AppError> {
        let mut st = lock(&self.0);
        if let Some(r) = st.lockouts.iter_mut().find(|r| r.user_id == user_id) {
            r.failure_count = 0;
            r.window_started_at = None;
            r.locked_until = None;
        }
        Ok(())
    }
}

pub struct MemLoginAttemptStore(pub MemHandle);

#[async_trait::async_trait]
impl LoginAttemptStore for MemLoginAttemptStore {
    async fn append(&self, _tx: &mut dyn Tx, new: NewLoginAttempt) -> Result<(), AppError> {
        lock(&self.0).attempts.push(new);
        Ok(())
    }
}

pub struct MemAudit(pub MemHandle);
impl AuditRecorder for MemAudit {
    fn record(&self, kind: &str, detail: &str) {
        lock(&self.0)
            .audits
            .push((kind.to_string(), detail.to_string()));
    }
}

pub struct MemPendingEvents(pub MemHandle);
impl PendingEventRecorder for MemPendingEvents {
    fn record_pending(&self, event_type: &str, subject: &str) {
        lock(&self.0)
            .pending_events
            .push((event_type.to_string(), subject.to_string()));
    }
}

pub struct MemOpsAlerts(pub MemHandle);
impl OpsAlertRecorder for MemOpsAlerts {
    fn alert_breakglass_activated(&self, doc_no: &str, user_id: &str) {
        lock(&self.0)
            .ops_alerts
            .push((doc_no.to_string(), user_id.to_string()));
    }
}

/// 伪 KMS：wrap 记明文、unwrap 回放；verify 恒真（X509 测试另行构造）。
pub struct FakeKms {
    pub store: Mutex<HashMap<Vec<u8>, Vec<u8>>>,
}

impl FakeKms {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl KmsBackend for FakeKms {
    async fn wrap(
        &self,
        _domain: KeyDomainId,
        _purpose: KeyPurpose,
        aad: &Aad,
        plaintext: &[u8],
    ) -> Result<CipherEnvelope, AppError> {
        self.store
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(aad.as_bytes().to_vec(), plaintext.to_vec());
        Ok(CipherEnvelope::new(aad.as_bytes().to_vec()))
    }
    async fn unwrap(
        &self,
        _domain: KeyDomainId,
        aad: &Aad,
        envelope: &CipherEnvelope,
    ) -> Result<Vec<u8>, AppError> {
        let map = self.store.lock().unwrap_or_else(|p| p.into_inner());
        if envelope.as_bytes() != aad.as_bytes() {
            return Err(AppError::new(
                ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
                "信封与 AAD 不符",
            ));
        }
        map.get(aad.as_bytes()).cloned().ok_or_else(|| {
            AppError::new(
                ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
                "信封未登记",
            )
        })
    }
    async fn derive_blind_key(
        &self,
        _legal_entity_id: Id<LegalEntity>,
        _column_fqn: &str,
        _plaintext: &[u8],
    ) -> Result<BlindIndex, AppError> {
        Ok(BlindIndex::new([0; 16]))
    }
    async fn sign(&self, _key: &KeyRef, _payload: &[u8]) -> Result<Signature, AppError> {
        Ok(Signature::new(vec![0u8; 8]))
    }
    async fn verify(
        &self,
        _key: &KeyRef,
        _payload: &[u8],
        signature: &Signature,
    ) -> Result<bool, AppError> {
        Ok(!signature.as_bytes().is_empty())
    }
    async fn health(&self) -> Result<(), AppError> {
        Ok(())
    }
}

/// 汇总测试可见的登录尝试结果序列。
pub fn attempt_outcomes(h: &MemHandle) -> Vec<LoginAttemptOutcome> {
    lock(h).attempts.iter().map(|a| a.outcome).collect()
}
