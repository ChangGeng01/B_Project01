//! 应急账号域：七态状态机用例与四要素校验（04 §5.4）。
//!
//! 四要素：单次启用 ≤8h（BreakglassPolicy）；启用要求持非口令单因子凭据；
//! 启用即 platform_ops 台账告警（经 [`OpsAlertRecorder`] 降级窗口端口）；
//! 批准人 ≠ 申请人且批准人持 SECURITY/AUDIT duty_class。
//! 到期失效：撤全部会话、凭据 REVOKED、写 rotation_result 语义；
//! 闲置 12 个月轮换登记（mark_rotated）。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_REQUEST_INVALID_PAYLOAD,
    PLATFORM_SOD_DUTY_CONFLICT, PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::UserAccount;
use ep_foundation::id::Id;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::{DutyClass, SecurityContext};

use crate::config::BreakglassPolicy;
use crate::ports::{
    AuditRecorder, BreakglassStore, CredentialStore, NewBreakglass, OpsAlertRecorder, SessionStore,
    UserAuthzQuery,
};
use crate::types::{
    BreakglassAction, BreakglassRow, BreakglassStatus, CredentialKind, REVOKE_BREAKGLASS_EXPIRED,
};

/// 到期失效写入 rotation_result 的语义字面量：凭据已撤待重建。
pub const ROTATION_RESULT_EXPIRED_REVOKED: &str = "EXPIRED_CREDENTIALS_REVOKED";
/// 闲置轮换写入 rotation_result 的语义字面量。
pub const ROTATION_RESULT_IDLE_REGISTERED: &str = "IDLE_ROTATION_REGISTERED";

/// 提交启用申请入参。
pub struct BreakglassSubmit {
    pub user_id: Id<UserAccount>,
    pub reason: String,
    pub allowed_action_set: Vec<BreakglassAction>,
}

/// 应急账号用例。事务法人上下文一律取调用方 SecurityContext。
pub struct BreakglassService<U: UnitOfWork> {
    uow: Arc<U>,
    store: Arc<dyn BreakglassStore>,
    sessions: Arc<dyn SessionStore>,
    credentials: Arc<dyn CredentialStore>,
    authz_query: Arc<dyn UserAuthzQuery>,
    audit: Arc<dyn AuditRecorder>,
    ops_alert: Arc<dyn OpsAlertRecorder>,
    policy: BreakglassPolicy,
}

impl<U: UnitOfWork> BreakglassService<U> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uow: Arc<U>,
        store: Arc<dyn BreakglassStore>,
        sessions: Arc<dyn SessionStore>,
        credentials: Arc<dyn CredentialStore>,
        authz_query: Arc<dyn UserAuthzQuery>,
        audit: Arc<dyn AuditRecorder>,
        ops_alert: Arc<dyn OpsAlertRecorder>,
        policy: BreakglassPolicy,
    ) -> Self {
        Self {
            uow,
            store,
            sessions,
            credentials,
            authz_query,
            audit,
            ops_alert,
            policy,
        }
    }

    /// 提交申请：DRAFT → PENDING_APPROVAL 同事务两跳。
    pub async fn submit(
        &self,
        ctx: &SecurityContext,
        input: BreakglassSubmit,
    ) -> Result<uuid::Uuid, AppError> {
        if input.allowed_action_set.is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "应急启用必须声明至少一类允许动作",
            ));
        }
        if input.reason.trim().is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "应急启用必须说明理由",
            ));
        }
        let (store, requested_by) = (self.store.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let doc_no = format!("BG-{}", uuid::Uuid::now_v7());
                    let id = store
                        .insert(
                            tx,
                            NewBreakglass {
                                doc_no,
                                user_id: input.user_id,
                                requested_by,
                                reason: input.reason,
                                allowed_action_set: input.allowed_action_set,
                            },
                        )
                        .await?;
                    store
                        .transition(
                            tx,
                            id,
                            BreakglassStatus::Draft,
                            BreakglassStatus::PendingApproval,
                        )
                        .await?;
                    Ok(id)
                })
            })
            .await
    }

    /// 批准：SoD（批准人≠申请人）与职责判据（SECURITY/AUDIT）双校验。
    pub async fn approve(
        &self,
        ctx: &SecurityContext,
        id: uuid::Uuid,
        approval_ref: &str,
    ) -> Result<(), AppError> {
        let (store, authz_q, approver, approval_ref) = (
            self.store.clone(),
            self.authz_query.clone(),
            ctx.user_id,
            approval_ref.to_string(),
        );
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let row = fetch(&*store, tx, id).await?;
                    if row.status != BreakglassStatus::PendingApproval {
                        return Err(state_error(&row, "待批准"));
                    }
                    if row.requested_by == approver {
                        return Err(AppError::new(
                            PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN,
                            "应急批准人不得为申请人本人",
                        ));
                    }
                    let duties = authz_q.user_duty_classes(tx, approver).await?;
                    let eligible = duties
                        .iter()
                        .any(|d| matches!(d, DutyClass::Security | DutyClass::Audit));
                    if !eligible {
                        return Err(AppError::new(
                            PLATFORM_SOD_DUTY_CONFLICT,
                            "应急批准人必须持 SECURITY 或 AUDIT 职责角色",
                        ));
                    }
                    store.approve(tx, id, approver, &approval_ref).await?;
                    Ok(())
                })
            })
            .await
    }

    /// 启用：≤8h 时长上限与「持非口令单因子」校验；启用即台账告警。
    pub async fn activate(
        &self,
        ctx: &SecurityContext,
        id: uuid::Uuid,
        duration_seconds: u64,
    ) -> Result<(), AppError> {
        if duration_seconds == 0 || duration_seconds > self.policy.max_session_seconds {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!(
                    "应急启用时长须在 1..={} 秒内（≤8h）",
                    self.policy.max_session_seconds
                ),
            ));
        }
        let (store, credentials, ops_alert, audit) = (
            self.store.clone(),
            self.credentials.clone(),
            self.ops_alert.clone(),
            self.audit.clone(),
        );
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let row = fetch(&*store, tx, id).await?;
                    if row.status != BreakglassStatus::Approved {
                        return Err(state_error(&row, "已批准"));
                    }
                    // 强制非口令单因子：应急账号必须已有非口令 ACTIVE 凭据。
                    let has_non_password =
                        store_non_password_factor(&*credentials, tx, row.user_id).await?;
                    if !has_non_password {
                        return Err(AppError::new(
                            PLATFORM_REQUEST_INVALID_PAYLOAD,
                            "应急账号启用前必须登记非口令单因子凭据",
                        ));
                    }
                    let now = Utc::now();
                    let expires_at = now
                        + chrono::Duration::seconds(
                            i64::try_from(duration_seconds).unwrap_or(i64::MAX),
                        );
                    store.activate(tx, id, now, expires_at).await?;
                    // platform_ops 台账告警语义（降级窗口端口复用面）。
                    ops_alert.alert_breakglass_activated(
                        &row.doc_no,
                        &row.user_id.as_uuid().to_string(),
                    );
                    audit.record(
                        "BREAKGLASS_ACTIVATED",
                        &format!("doc_no={} user={}", row.doc_no, row.user_id.as_uuid()),
                    );
                    Ok(())
                })
            })
            .await
    }

    /// 主动关闭：撤该应急账号全部会话后置 CLOSED。
    pub async fn close(&self, ctx: &SecurityContext, id: uuid::Uuid) -> Result<(), AppError> {
        let (store, sessions) = (self.store.clone(), self.sessions.clone());
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let row = fetch(&*store, tx, id).await?;
                    if row.status != BreakglassStatus::Active {
                        return Err(state_error(&row, "启用中"));
                    }
                    let now = Utc::now();
                    sessions
                        .revoke_all_for_user(tx, row.user_id, "BREAKGLASS_CLOSED")
                        .await?;
                    store.close(tx, id, now).await?;
                    Ok(())
                })
            })
            .await
    }

    /// 到期失效（job-worker）：撤会话、凭据 REVOKED、写轮换语义。
    pub async fn expire_due(
        &self,
        ctx: &SecurityContext,
        now: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        let (store, sessions, credentials) = (
            self.store.clone(),
            self.sessions.clone(),
            self.credentials.clone(),
        );
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let due = store.list_due_active(tx, now).await?;
                    let mut n = 0u64;
                    for row in due {
                        sessions
                            .revoke_all_for_user(tx, row.user_id, REVOKE_BREAKGLASS_EXPIRED)
                            .await?;
                        credentials.revoke_all_for_user(tx, row.user_id).await?;
                        store
                            .finalize_with_rotation(
                                tx,
                                row.id,
                                BreakglassStatus::Expired,
                                ROTATION_RESULT_EXPIRED_REVOKED,
                                now,
                            )
                            .await?;
                        n += 1;
                    }
                    Ok(n)
                })
            })
            .await
    }

    /// 闲置轮换登记（job-worker）：终态且 rotated_at 早于 cutoff 者。
    pub async fn rotate_idle(
        &self,
        ctx: &SecurityContext,
        now: DateTime<Utc>,
    ) -> Result<u64, AppError> {
        let days = i64::from(self.policy.idle_rotation_days);
        let cutoff = now - chrono::Duration::days(days);
        let store = self.store.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let idle = store.list_idle_for_rotation(tx, cutoff).await?;
                    let mut n = 0u64;
                    for row in idle {
                        store
                            .mark_rotated(tx, row.id, ROTATION_RESULT_IDLE_REGISTERED, now)
                            .await?;
                        n += 1;
                    }
                    Ok(n)
                })
            })
            .await
    }
}

async fn fetch(
    store: &dyn BreakglassStore,
    tx: &mut dyn ep_foundation::port::tx::Tx,
    id: uuid::Uuid,
) -> Result<BreakglassRow, AppError> {
    store.get(tx, id).await?.ok_or_else(|| {
        AppError::new(
            PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
            "应急启用单不存在或不可见",
        )
    })
}

fn state_error(row: &BreakglassRow, want: &str) -> AppError {
    AppError::new(
        PLATFORM_REQUEST_INVALID_PAYLOAD,
        format!("应急启用单当前状态 {}，须处于{want}", row.status.as_str()),
    )
}

async fn store_non_password_factor(
    credentials: &dyn CredentialStore,
    tx: &mut dyn ep_foundation::port::tx::Tx,
    user_id: Id<UserAccount>,
) -> Result<bool, AppError> {
    Ok(credentials
        .list_active(tx, user_id)
        .await?
        .iter()
        .any(|c| c.credential_kind != CredentialKind::Password))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{
        mem, InMemoryUow, MemAudit, MemCredentialStore, MemOpsAlerts, MemSessionStore,
    };
    use crate::testutil_extra::{MemBreakglassStore, MemUserAuthzQuery};
    use crate::types::{CredentialRow, CredentialStatus, SessionRow};
    use ep_foundation::id::marker::LegalEntity;
    use ep_foundation::security::context::{RequestId, TraceId};

    fn ctx() -> SecurityContext {
        let le = Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2));
        SecurityContext::system(
            le,
            RequestId::new("0199aaaa").expect("合法"),
            TraceId::new(&"0".repeat(32)).expect("合法"),
        )
    }

    fn service(h: &crate::testutil::MemHandle) -> BreakglassService<InMemoryUow> {
        BreakglassService::new(
            Arc::new(InMemoryUow),
            Arc::new(MemBreakglassStore(h.clone())),
            Arc::new(MemSessionStore(h.clone())),
            Arc::new(MemCredentialStore(h.clone())),
            Arc::new(MemUserAuthzQuery(h.clone())),
            Arc::new(MemAudit(h.clone())),
            Arc::new(MemOpsAlerts(h.clone())),
            BreakglassPolicy::default(),
        )
    }

    fn seed_factor(h: &crate::testutil::MemHandle, user_id: Id<UserAccount>) {
        crate::testutil::lock(h).credentials.push(CredentialRow {
            id: uuid::Uuid::from_u128(77),
            user_id,
            credential_kind: CredentialKind::Totp,
            verifier: Some("envelope".into()),
            public_key: None,
            credential_handle: None,
            secret_ref: Some("secret://kms/totp/x#1".into()),
            sign_count: 0,
            status: CredentialStatus::Active,
            security_level: 30,
            created_at: Utc::now(),
        });
    }

    async fn submitted(svc: &BreakglassService<InMemoryUow>, user: u128) -> uuid::Uuid {
        svc.submit(
            &ctx(),
            BreakglassSubmit {
                user_id: Id::from_uuid(uuid::Uuid::from_u128(user)),
                reason: "主控台不可用".into(),
                allowed_action_set: vec![BreakglassAction::UnlockOrResetAdmin],
            },
        )
        .await
        .expect("提交成功")
    }

    #[tokio::test]
    async fn submit_requires_actions_and_reason() {
        let h = mem();
        let svc = service(&h);
        let err = svc
            .submit(
                &ctx(),
                BreakglassSubmit {
                    user_id: Id::from_uuid(uuid::Uuid::from_u128(1)),
                    reason: "x".into(),
                    allowed_action_set: vec![],
                },
            )
            .await
            .expect_err("空动作集拒");
        assert_eq!(err.code.0, "PLATFORM.REQUEST.INVALID_PAYLOAD");
    }

    #[tokio::test]
    async fn approve_enforces_sod_and_duty() {
        let h = mem();
        let svc = service(&h);
        let id = submitted(&svc, 5).await;
        // 自批拒：system ctx 的 user_id 即申请人（同 ctx 提交）。
        let err = svc.approve(&ctx(), id, "AP-1").await.expect_err("自批拒");
        assert_eq!(err.code.0, "PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN");
        // 他人但无 SECURITY/AUDIT 职责拒。
        {
            let mut st = crate::testutil::lock(&h);
            if let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) {
                b.requested_by = Id::from_uuid(uuid::Uuid::from_u128(999));
            }
        }
        let err = svc.approve(&ctx(), id, "AP-1").await.expect_err("无职责拒");
        assert_eq!(err.code.0, "PLATFORM.SOD.DUTY_CONFLICT");
        // 持 SECURITY 职责通过。
        crate::testutil::lock(&h).duties = vec![DutyClass::Security];
        svc.approve(&ctx(), id, "AP-1").await.expect("批准成功");
    }

    #[tokio::test]
    async fn activate_rejects_over_8h_and_missing_factor() {
        let h = mem();
        let svc = service(&h);
        let id = submitted(&svc, 5).await;
        prepare_approved(&h, id);
        let err = svc
            .activate(&ctx(), id, 28_801)
            .await
            .expect_err("超 8h 拒");
        assert_eq!(err.code.0, "PLATFORM.REQUEST.INVALID_PAYLOAD");
        let err = svc
            .activate(&ctx(), id, 3_600)
            .await
            .expect_err("无非口令因子拒");
        assert_eq!(err.code.0, "PLATFORM.REQUEST.INVALID_PAYLOAD");
        seed_factor(&h, Id::from_uuid(uuid::Uuid::from_u128(5)));
        svc.activate(&ctx(), id, 3_600).await.expect("启用成功");
        let st = crate::testutil::lock(&h);
        assert_eq!(st.ops_alerts.len(), 1, "启用即台账告警");
        assert!(st.audits.iter().any(|(k, _)| k == "BREAKGLASS_ACTIVATED"));
    }

    fn prepare_approved(h: &crate::testutil::MemHandle, id: uuid::Uuid) {
        let mut st = crate::testutil::lock(h);
        if let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) {
            b.status = BreakglassStatus::Approved;
            b.requested_by = Id::from_uuid(uuid::Uuid::from_u128(999));
        }
    }

    #[tokio::test]
    async fn expire_due_revokes_sessions_credentials_and_writes_rotation() {
        let h = mem();
        let svc = service(&h);
        let user = Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(5));
        let id = submitted(&svc, 5).await;
        let now = Utc::now();
        {
            let mut st = crate::testutil::lock(&h);
            if let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) {
                b.status = BreakglassStatus::Active;
                b.activated_at = Some(now - chrono::Duration::hours(9));
                b.expires_at = Some(now - chrono::Duration::minutes(1));
            }
            st.sessions.push(SessionRow {
                id: uuid::Uuid::from_u128(42),
                user_id: user,
                user_device_row_id: uuid::Uuid::from_u128(9),
                token_hash: vec![0; 32],
                active_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
                issued_at: now,
                expires_at: now + chrono::Duration::hours(8),
                idle_expires_at: now + chrono::Duration::minutes(30),
                last_seen_at: now,
                revoked_at: None,
                revoke_reason: None,
                is_breakglass: true,
            });
        }
        seed_factor(&h, user);
        let n = svc.expire_due(&ctx(), now).await.expect("到期失效");
        assert_eq!(n, 1);
        let st = crate::testutil::lock(&h);
        assert_eq!(
            st.sessions[0].revoke_reason.as_deref(),
            Some("BREAKGLASS_EXPIRED")
        );
        assert_eq!(st.credentials[0].status, CredentialStatus::Revoked);
        assert_eq!(
            st.breakglass[0].rotation_result.as_deref(),
            Some(ROTATION_RESULT_EXPIRED_REVOKED)
        );
        assert_eq!(st.breakglass[0].status, BreakglassStatus::Expired);
    }

    #[tokio::test]
    async fn rotate_idle_marks_terminal_rows() {
        let h = mem();
        let svc = service(&h);
        let id = submitted(&svc, 5).await;
        {
            let mut st = crate::testutil::lock(&h);
            if let Some(b) = st.breakglass.iter_mut().find(|b| b.id == id) {
                b.status = BreakglassStatus::Closed;
                b.closed_at = Some(Utc::now() - chrono::Duration::days(400));
            }
        }
        let n = svc.rotate_idle(&ctx(), Utc::now()).await.expect("轮换登记");
        assert_eq!(n, 1);
        let st = crate::testutil::lock(&h);
        assert_eq!(
            st.breakglass[0].rotation_result.as_deref(),
            Some(ROTATION_RESULT_IDLE_REGISTERED)
        );
        assert!(st.breakglass[0].rotated_at.is_some());
    }
}
