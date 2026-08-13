//! 账号生命周期域：登出、会话清单/撤销、设备登记/远程注销、
//! 口令重置、激活/移交/停用（04 §5.2/§5.3）。
//!
//! 停用级联：即时撤全部会话与设备凭据，并经 [`PendingEventRecorder`]
//! 登记 `platform.user_account.deactivated.v1`（Outbox 表属阶段 3b，
//! 首版走日志先例，见汇报）。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_CREDENTIAL_INVALID, PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_REQUEST_INVALID_PAYLOAD,
    PLATFORM_USER_ACCOUNT_PENDING_APPROVAL_TASKS,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::SecurityContext;
use ep_platform_authz::sod::{check_duty_exclusion, violation_error};

use crate::config::IdentityPolicies;
use crate::password::{check_policy, PasswordService};
use crate::ports::{
    AccountStore, AuditRecorder, CredentialStore, DeviceStore, NewCredential, NewDevice,
    PasswordHistoryStore, PendingEventRecorder, SessionStore, UserAuthzQuery,
};
use crate::session::token_digest;
use crate::types::{
    AccountStatus, CredentialKind, CredentialStatus, DeviceRow, SessionRow, UserAccountRow,
    REVOKE_ACCOUNT_DEACTIVATED, REVOKE_DEVICE_REVOKED, REVOKE_SIGN_OUT,
};

/// 停用事件的登记类型（event-catalog 同步登记）。
pub const EVENT_USER_ACCOUNT_DEACTIVATED: &str = "platform.user_account.deactivated.v1";

/// 设备登记入参（对外面；user_id 取调用上下文）。
pub struct DeviceRegisterInput {
    pub device_id: String,
    pub client: ep_foundation::security::context::ClientKind,
    pub public_key: Option<String>,
    pub attestation_ref: Option<String>,
    pub restricted_legal_entity_id: Option<Id<LegalEntity>>,
}

/// 身份生命周期用例。
pub struct LifecycleService<U: UnitOfWork> {
    uow: Arc<U>,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    password_history: Arc<dyn PasswordHistoryStore>,
    devices: Arc<dyn DeviceStore>,
    sessions: Arc<dyn SessionStore>,
    authz_query: Arc<dyn UserAuthzQuery>,
    pending_events: Arc<dyn PendingEventRecorder>,
    audit: Arc<dyn AuditRecorder>,
    password_service: Arc<PasswordService>,
    policies: IdentityPolicies,
}

impl<U: UnitOfWork> LifecycleService<U> {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        uow: Arc<U>,
        accounts: Arc<dyn AccountStore>,
        credentials: Arc<dyn CredentialStore>,
        password_history: Arc<dyn PasswordHistoryStore>,
        devices: Arc<dyn DeviceStore>,
        sessions: Arc<dyn SessionStore>,
        authz_query: Arc<dyn UserAuthzQuery>,
        pending_events: Arc<dyn PendingEventRecorder>,
        audit: Arc<dyn AuditRecorder>,
        password_service: Arc<PasswordService>,
        policies: IdentityPolicies,
    ) -> Self {
        Self {
            uow,
            accounts,
            credentials,
            password_history,
            devices,
            sessions,
            authz_query,
            pending_events,
            audit,
            password_service,
            policies,
        }
    }

    /// sign-out：按令牌摘要定位会话并撤销（理由 SIGN_OUT）。
    pub async fn sign_out(&self, ctx: &SecurityContext, token: &str) -> Result<bool, AppError> {
        let (sessions, user_id) = (self.sessions.clone(), ctx.user_id);
        let digest = token_digest(token).to_vec();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let Some(row) = sessions.find_active_by_digest(tx, &digest).await? else {
                        return Ok(false);
                    };
                    if row.user_id != user_id {
                        return Ok(false);
                    }
                    sessions.revoke(tx, row.id, REVOKE_SIGN_OUT).await
                })
            })
            .await
    }

    /// 会话清单（本人维度；端点侧再按职责限读）。
    pub async fn list_my_sessions(
        &self,
        ctx: &SecurityContext,
    ) -> Result<Vec<SessionRow>, AppError> {
        let (sessions, user_id) = (self.sessions.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move { sessions.list_active_for_user(tx, user_id).await })
            })
            .await
    }

    /// 撤销本人名下指定会话。
    pub async fn revoke_my_session(
        &self,
        ctx: &SecurityContext,
        session_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        let (sessions, user_id) = (self.sessions.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let rows = sessions.list_active_for_user(tx, user_id).await?;
                    if !rows.iter().any(|s| s.id == session_id) {
                        return Ok(false);
                    }
                    sessions.revoke(tx, session_id, REVOKE_SIGN_OUT).await
                })
            })
            .await
    }

    /// 设备登记（单法人限定可选）。
    pub async fn register_device(
        &self,
        ctx: &SecurityContext,
        input: DeviceRegisterInput,
    ) -> Result<uuid::Uuid, AppError> {
        let (devices, user_id) = (self.devices.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    devices
                        .insert(
                            tx,
                            NewDevice {
                                user_id,
                                device_id: input.device_id,
                                client: input.client,
                                public_key: input.public_key,
                                attestation_ref: input.attestation_ref,
                                restricted_legal_entity_id: input.restricted_legal_entity_id,
                            },
                        )
                        .await
                })
            })
            .await
    }

    /// 远程注销设备：设备 REVOKED 且级联撤该设备全部会话。
    pub async fn revoke_device(
        &self,
        ctx: &SecurityContext,
        device_row_id: uuid::Uuid,
    ) -> Result<u64, AppError> {
        let (devices, sessions, user_id) =
            (self.devices.clone(), self.sessions.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let mine = devices.list(tx, user_id).await?;
                    if !mine.iter().any(|d| d.id == device_row_id) {
                        return Err(AppError::new(
                            PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
                            "设备不存在或不属于当前用户",
                        ));
                    }
                    devices.revoke(tx, device_row_id).await?;
                    sessions
                        .revoke_by_device(tx, user_id, device_row_id, REVOKE_DEVICE_REVOKED)
                        .await
                })
            })
            .await
    }

    /// identity/me：当前账号档案。
    pub async fn me(&self, ctx: &SecurityContext) -> Result<UserAccountRow, AppError> {
        let (accounts, user_id) = (self.accounts.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    accounts.get(tx, user_id).await?.ok_or_else(|| {
                        AppError::new(PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, "账号不存在或不可见")
                    })
                })
            })
            .await
    }

    /// me/legal-entities：已安装法人逐个探测授权（不 OR 展开）。
    pub async fn me_legal_entities(
        &self,
        ctx: &SecurityContext,
    ) -> Result<Vec<Id<LegalEntity>>, AppError> {
        let (authz_q, user_id) = (self.authz_query.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let installed = authz_q.installed_legal_entities(tx).await?;
                    let mut granted = Vec::new();
                    for le in installed {
                        if authz_q.probe_legal_entity_grant(tx, user_id, le).await? {
                            granted.push(le);
                        }
                    }
                    Ok(granted)
                })
            })
            .await
    }

    /// 重置口令：策略校验 → 历史不重复 → 新凭据替换旧凭据 → 历史追加。
    pub async fn reset_password(
        &self,
        ctx: &SecurityContext,
        new_password: &str,
        now: DateTime<Utc>,
    ) -> Result<(), AppError> {
        check_policy(new_password, &self.policies.password)?;
        let (credentials, history, pws, policy, user_id) = (
            self.credentials.clone(),
            self.password_history.clone(),
            self.password_service.clone(),
            self.policies.clone(),
            ctx.user_id,
        );
        let new_password = new_password.to_string();
        let _ = now; // 有效期基准取新凭据 created_at（实现体落库时刻）。
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let recent = history
                        .recent_verifiers(tx, user_id, policy.password.history_size)
                        .await?;
                    if pws.matches_history(&new_password, &recent) {
                        return Err(AppError::new(
                            PLATFORM_AUTHN_CREDENTIAL_INVALID,
                            format!(
                                "新口令不得与最近 {} 代历史口令重复",
                                policy.password.history_size
                            ),
                        ));
                    }
                    let verifier = pws.hash(&new_password)?;
                    if let Some(old) = credentials
                        .active_of_kind(tx, user_id, CredentialKind::Password)
                        .await?
                    {
                        credentials
                            .set_status(tx, old.id, CredentialStatus::Revoked)
                            .await?;
                    }
                    credentials
                        .insert(
                            tx,
                            NewCredential {
                                user_id,
                                credential_kind: CredentialKind::Password,
                                verifier: Some(verifier.clone()),
                                public_key: None,
                                credential_handle: None,
                                secret_ref: None,
                            },
                        )
                        .await?;
                    history.append(tx, user_id, verifier, user_id).await
                })
            })
            .await
    }

    /// 激活：UNACTIVATED → ACTIVE（CAS 限定起态）。
    pub async fn activate_account(
        &self,
        ctx: &SecurityContext,
        target: Id<UserAccount>,
    ) -> Result<bool, AppError> {
        let accounts = self.accounts.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    accounts
                        .transition_status(
                            tx,
                            target,
                            Some(AccountStatus::Unactivated),
                            AccountStatus::Active,
                        )
                        .await
                })
            })
            .await
    }

    /// 移交：SoD 职责互斥校验（authz 纯函数）+ 未结审批待办校验。
    pub async fn transfer_account(
        &self,
        ctx: &SecurityContext,
        from: Id<UserAccount>,
        to: Id<UserAccount>,
    ) -> Result<(), AppError> {
        if from == to {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "移交的源账号与目标账号不得相同",
            ));
        }
        let (authz_q, audit) = (self.authz_query.clone(), self.audit.clone());
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let open = authz_q.count_open_high_risk_requests(tx, from).await?;
                    if open > 0 {
                        return Err(AppError::new(
                            PLATFORM_USER_ACCOUNT_PENDING_APPROVAL_TASKS,
                            format!("源账号尚有 {open} 项未结高风险审批待办"),
                        ));
                    }
                    let duties = authz_q.user_duty_classes(tx, to).await?;
                    if let Some(v) = check_duty_exclusion(&duties) {
                        return Err(violation_error(&v));
                    }
                    audit.record(
                        "USER_ACCOUNT_TRANSFER",
                        &format!("from={} to={}", from.as_uuid(), to.as_uuid()),
                    );
                    Ok(())
                })
            })
            .await
    }

    /// 停用：即时撤全部会话、凭据、设备，并登记 deactivated.v1 事件。
    pub async fn deactivate(
        &self,
        ctx: &SecurityContext,
        target: Id<UserAccount>,
    ) -> Result<(), AppError> {
        let (accounts, credentials, devices, sessions, pending, audit) = (
            self.accounts.clone(),
            self.credentials.clone(),
            self.devices.clone(),
            self.sessions.clone(),
            self.pending_events.clone(),
            self.audit.clone(),
        );
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let Some(row) = accounts.get(tx, target).await? else {
                        return Err(AppError::new(
                            PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                            "账号不存在或不可见",
                        ));
                    };
                    if row.status == AccountStatus::Deactivated {
                        return Ok(());
                    }
                    accounts
                        .transition_status(tx, target, None, AccountStatus::Deactivated)
                        .await?;
                    sessions
                        .revoke_all_for_user(tx, target, REVOKE_ACCOUNT_DEACTIVATED)
                        .await?;
                    credentials.revoke_all_for_user(tx, target).await?;
                    for d in devices.list(tx, target).await? {
                        devices.revoke(tx, d.id).await?;
                    }
                    audit.record(
                        "USER_ACCOUNT_DEACTIVATED",
                        &format!("user={}", target.as_uuid()),
                    );
                    Ok(())
                })
            })
            .await?;
        // Outbox 表属阶段 3b：首版经占位端口登记待发出事件（日志先例）。
        pending.record_pending(
            EVENT_USER_ACCOUNT_DEACTIVATED,
            &target.as_uuid().to_string(),
        );
        Ok(())
    }
}

/// 设备行可见性辅助（端点层复用）。
pub fn device_is_active(row: &DeviceRow) -> bool {
    matches!(row.status, crate::types::DeviceStatus::Active)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Argon2Params;
    use crate::context_build::pre_auth_context;
    use crate::testutil::{
        lock, mem, InMemoryUow, MemAccountStore, MemAudit, MemCredentialStore, MemDeviceStore,
        MemPasswordHistoryStore, MemPendingEvents, MemSessionStore,
    };
    use crate::testutil_extra::MemUserAuthzQuery;
    use crate::types::{AccountKind, CredentialRow, CredentialStatus, DeviceStatus};
    use ep_foundation::security::context::ClientKind;

    fn account(user: u128) -> UserAccountRow {
        UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(user)),
            account_kind: AccountKind::Employee,
            login_name: format!("u{user}"),
            employee_no: None,
            display_name: "T".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: false,
            created_at: Utc::now(),
        }
    }

    fn ctx_for(user: u128) -> SecurityContext {
        pre_auth_context(&account(user), "DEV-01", "0199aaaa", &"0".repeat(32)).expect("合法")
    }

    fn svc(h: &crate::testutil::MemHandle) -> LifecycleService<InMemoryUow> {
        let pws = Arc::new(
            PasswordService::new(Argon2Params {
                memory_kib: 8,
                iterations: 1,
                parallelism: 1,
            })
            .expect("参数合法"),
        );
        LifecycleService::new(
            Arc::new(InMemoryUow),
            Arc::new(MemAccountStore(h.clone())),
            Arc::new(MemCredentialStore(h.clone())),
            Arc::new(MemPasswordHistoryStore(h.clone())),
            Arc::new(MemDeviceStore(h.clone())),
            Arc::new(MemSessionStore(h.clone())),
            Arc::new(MemUserAuthzQuery(h.clone())),
            Arc::new(MemPendingEvents(h.clone())),
            Arc::new(MemAudit(h.clone())),
            pws,
            IdentityPolicies::default(),
        )
    }

    fn seed_session(
        h: &crate::testutil::MemHandle,
        user: u128,
        device_row: u128,
        digest: Vec<u8>,
    ) -> uuid::Uuid {
        let id = uuid::Uuid::now_v7();
        let now = Utc::now();
        lock(h).sessions.push(SessionRow {
            id,
            user_id: Id::from_uuid(uuid::Uuid::from_u128(user)),
            user_device_row_id: uuid::Uuid::from_u128(device_row),
            token_hash: digest,
            active_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            issued_at: now,
            expires_at: now + chrono::Duration::hours(8),
            idle_expires_at: now + chrono::Duration::minutes(30),
            last_seen_at: now,
            revoked_at: None,
            revoke_reason: None,
            is_breakglass: false,
        });
        id
    }

    #[tokio::test]
    async fn sign_out_revokes_own_session_only() {
        let h = mem();
        let svc = svc(&h);
        let digest = token_digest("tok-abc").to_vec();
        seed_session(&h, 1, 9, digest.clone());
        assert!(!svc
            .sign_out(&ctx_for(2), "tok-abc")
            .await
            .expect("他人会话不可销"));
        assert!(svc
            .sign_out(&ctx_for(1), "tok-abc")
            .await
            .expect("销自己会话"));
        assert_eq!(
            lock(&h).sessions[0].revoke_reason.as_deref(),
            Some(REVOKE_SIGN_OUT)
        );
        assert!(!svc
            .sign_out(&ctx_for(1), "tok-abc")
            .await
            .expect("重复销返否"));
    }

    #[tokio::test]
    async fn revoke_device_cascades_its_sessions() {
        let h = mem();
        let svc = svc(&h);
        let device_row = uuid::Uuid::from_u128(9);
        lock(&h).devices.push(DeviceRow {
            id: device_row,
            user_id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            device_id: "DEV-01".into(),
            client: ClientKind::Win,
            public_key: None,
            attestation_ref: None,
            restricted_legal_entity_id: None,
            status: DeviceStatus::Active,
        });
        seed_session(&h, 1, 9, vec![1; 32]);
        seed_session(&h, 1, 9, vec![2; 32]);
        seed_session(&h, 1, 8, vec![3; 32]);
        let n = svc
            .revoke_device(&ctx_for(1), device_row)
            .await
            .expect("注销");
        assert_eq!(n, 2, "仅级联该设备的会话");
        let st = lock(&h);
        assert_eq!(st.devices[0].status, DeviceStatus::Revoked);
        assert_eq!(st.sessions[2].revoked_at, None);
    }

    #[tokio::test]
    async fn reset_password_rejects_history_reuse_and_rotates_credential() {
        let h = mem();
        let svc = svc(&h);
        svc.reset_password(&ctx_for(1), "Ab1!Ab1!Ab1!", Utc::now())
            .await
            .expect("首次重置");
        let err = svc
            .reset_password(&ctx_for(1), "Ab1!Ab1!Ab1!", Utc::now())
            .await
            .expect_err("历史重复拒");
        assert_eq!(err.code.0, "PLATFORM.AUTHN.CREDENTIAL_INVALID");
        svc.reset_password(&ctx_for(1), "Xy9#Xy9#Xy9#", Utc::now())
            .await
            .expect("新口令过");
        let st = lock(&h);
        assert_eq!(st.credentials.len(), 2);
        assert_eq!(
            st.credentials[0].status,
            CredentialStatus::Revoked,
            "旧凭据撤销"
        );
        assert_eq!(st.password_history.len(), 2);
    }

    #[tokio::test]
    async fn deactivate_cascades_sessions_credentials_devices_and_records_event() {
        let h = mem();
        let svc = svc(&h);
        lock(&h).accounts.push(account(7));
        lock(&h).credentials.push(CredentialRow {
            id: uuid::Uuid::from_u128(71),
            user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
            credential_kind: CredentialKind::Password,
            verifier: Some("$argon2id$x".into()),
            public_key: None,
            credential_handle: None,
            secret_ref: None,
            sign_count: 0,
            status: CredentialStatus::Active,
            security_level: 30,
            created_at: Utc::now(),
        });
        lock(&h).devices.push(DeviceRow {
            id: uuid::Uuid::from_u128(72),
            user_id: Id::from_uuid(uuid::Uuid::from_u128(7)),
            device_id: "DEV-07".into(),
            client: ClientKind::Win,
            public_key: None,
            attestation_ref: None,
            restricted_legal_entity_id: None,
            status: DeviceStatus::Active,
        });
        seed_session(&h, 7, 72, vec![9; 32]);
        svc.deactivate(&ctx_for(1), Id::from_uuid(uuid::Uuid::from_u128(7)))
            .await
            .expect("停用");
        let st = lock(&h);
        assert_eq!(st.accounts[0].status, AccountStatus::Deactivated);
        assert_eq!(
            st.sessions[0].revoke_reason.as_deref(),
            Some(REVOKE_ACCOUNT_DEACTIVATED)
        );
        assert_eq!(st.credentials[0].status, CredentialStatus::Revoked);
        assert_eq!(st.devices[0].status, DeviceStatus::Revoked);
        assert_eq!(st.pending_events[0].0, EVENT_USER_ACCOUNT_DEACTIVATED);
    }

    #[tokio::test]
    async fn transfer_rejects_pending_approvals() {
        let h = mem();
        let svc = svc(&h);
        lock(&h).open_high_risk_requests = 2;
        let err = svc
            .transfer_account(
                &ctx_for(1),
                Id::from_uuid(uuid::Uuid::from_u128(3)),
                Id::from_uuid(uuid::Uuid::from_u128(4)),
            )
            .await
            .expect_err("未结待办拒");
        assert_eq!(err.code.0, "PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS");
        lock(&h).open_high_risk_requests = 0;
        svc.transfer_account(
            &ctx_for(1),
            Id::from_uuid(uuid::Uuid::from_u128(3)),
            Id::from_uuid(uuid::Uuid::from_u128(4)),
        )
        .await
        .expect("无待办过");
        assert!(lock(&h)
            .audits
            .iter()
            .any(|(k, _)| k == "USER_ACCOUNT_TRANSFER"));
    }

    #[tokio::test]
    async fn activate_only_from_unactivated() {
        let h = mem();
        let svc = svc(&h);
        let mut acc = account(6);
        acc.status = AccountStatus::Unactivated;
        lock(&h).accounts.push(acc);
        let target = Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(6));
        assert!(svc
            .activate_account(&ctx_for(1), target)
            .await
            .expect("激活"));
        assert!(!svc
            .activate_account(&ctx_for(1), target)
            .await
            .expect("重复激活返否"));
        assert_eq!(lock(&h).accounts[0].status, AccountStatus::Active);
    }
}
