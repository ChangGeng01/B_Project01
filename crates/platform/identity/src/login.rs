//! 登录域：sign-in 九步算法与 complete-mfa 二段完成。
//!
//! 步序（04 §4.3，步 1/2 微调：准入需 user_id，故先以请求侧缺省主体
//! 准入、账号查找入事务，取舍见汇报）：
//! 准入信号量 → 查账号 → 锁定检查（FOR UPDATE 语义在实现体）→ 第一因子
//! → MFA 判定（读 authz 快照面）→ 设备校验（单法人限定取交集）→ 会话上限
//! → 令牌生成 → 四项同事务提交。
//! 失败路径一律 `Ok(LoginTxOutcome::Rejected)` 提交不回滚：
//! login_attempts 与 account_lockouts 同事务落库（基线 10.3）。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_ACCOUNT_INACTIVE, PLATFORM_AUTHN_ACCOUNT_LOCKED,
    PLATFORM_AUTHN_CREDENTIAL_INVALID, PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
    PLATFORM_AUTHN_MFA_INVALID, PLATFORM_SYSTEM_INTERNAL_ERROR,
    PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED,
};
use ep_foundation::error::{AppError, ErrorCode};
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::port::kms::KmsBackend;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::{ClientKind, RequestId, SecurityContext, TraceId};
use ep_platform_authz::AdmissionGate;

use crate::config::IdentityPolicies;
use crate::context_build::{build_session_context, pre_auth_context, SessionContextInput};
use crate::mfa::{is_mfa_required, MfaChallengePayload, MfaChallengeService};
use crate::password::PasswordService;
use crate::ports::{
    AccountStore, AuditRecorder, CredentialStore, DeviceStore, LockoutStore, LoginAttemptStore,
    NewLoginAttempt, NewSession, SessionStore, UserAuthzQuery, UserAuthzSet,
};
use crate::session::{expiry_pair, new_session_token, token_digest};
use crate::types::{
    AccountKind, CredentialKind, LoginAttemptOutcome, UserAccountRow, REVOKE_SESSION_LIMIT_EXCEEDED,
};

/// MFA 挑战有效期（秒）：sign-in 与 complete-mfa 之间的人工窗口。
pub const MFA_CHALLENGE_TTL_SECONDS: u64 = 300;

/// sign-in 入参。口令仅存在于本结构，任何日志不得引用。
pub struct SignInRequest {
    pub login_name: String,
    pub password: String,
    pub device_id: String,
    pub client: String,
    pub source_addr: String,
    pub request_id: String,
    pub trace_id: String,
    /// 门户端点传 Some(Portal)，强制账号形态匹配。
    pub expected_kind: Option<AccountKind>,
}

/// complete-mfa 的第二因子凭证。
pub enum SecondFactorProof {
    Totp { code: String },
}

/// complete-mfa 入参。
pub struct CompleteMfaRequest {
    pub challenge: String,
    pub proof: SecondFactorProof,
    pub source_addr: String,
    pub request_id: String,
    pub trace_id: String,
}

/// 登录成功产物：明文令牌只在此出现一次。
pub struct SignInSuccess {
    pub session_token: String,
    pub session_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub active_legal_entity_id: ep_foundation::id::Id<LegalEntity>,
    pub expires_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub is_breakglass: bool,
    pub context: SecurityContext,
}

/// sign-in 结果：直接成功或需第二因子。成功产物装箱，避免
/// 大载荷变体把整个枚举撑大（clippy::large_enum_variant）。
pub enum SignInOutcome {
    Authenticated(Box<SignInSuccess>),
    MfaRequired { challenge: String },
}

/// 事务内拒绝变体：映射到已登记错误码，失败路径提交不回滚。
enum Rejection {
    CredentialInvalid(&'static str),
    AccountLocked,
    AccountInactive,
    DeviceUnregistered,
    MfaEnrollmentRequired,
    MfaInvalid,
}

impl Rejection {
    fn code(self) -> (ErrorCode, &'static str) {
        match self {
            Rejection::CredentialInvalid(m) => (PLATFORM_AUTHN_CREDENTIAL_INVALID, m),
            Rejection::AccountLocked => (PLATFORM_AUTHN_ACCOUNT_LOCKED, "账号处于锁定期"),
            Rejection::AccountInactive => (PLATFORM_AUTHN_ACCOUNT_INACTIVE, "账号状态不可登录"),
            Rejection::DeviceUnregistered => {
                (PLATFORM_AUTHN_DEVICE_NOT_REGISTERED, "设备未登记或已注销")
            }
            Rejection::MfaEnrollmentRequired => (
                PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED,
                "强制 MFA 但尚无已登记的第二因子",
            ),
            Rejection::MfaInvalid => (PLATFORM_AUTHN_MFA_INVALID, "第二因子校验未通过"),
        }
    }
}

/// 事务闭包产物：永远 Ok 返回（提交不回滚）。大载荷两字段装箱，
/// 与 MfaPending/Rejected 变体的尺寸保持同一量级。
enum LoginTxOutcome {
    Succeeded {
        account: Box<UserAccountRow>,
        authz: Box<UserAuthzSet>,
        token: String,
        session_id: uuid::Uuid,
        active_le: ep_foundation::id::Id<LegalEntity>,
        device_id: String,
        expires_at: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
        is_breakglass: bool,
    },
    MfaPending {
        challenge: String,
    },
    Rejected {
        rejection: Rejection,
    },
}

/// 登录用例。泛型 U：装配注入 PgUnitOfWork，测试注入内存 UoW。
pub struct LoginService<U: UnitOfWork> {
    uow: Arc<U>,
    admission: Arc<AdmissionGate>,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    devices: Arc<dyn DeviceStore>,
    sessions: Arc<dyn SessionStore>,
    lockouts: Arc<dyn LockoutStore>,
    attempts: Arc<dyn LoginAttemptStore>,
    authz_query: Arc<dyn UserAuthzQuery>,
    audit: Arc<dyn AuditRecorder>,
    password_service: Arc<PasswordService>,
    challenges: Arc<MfaChallengeService>,
    kms: Arc<dyn KmsBackend>,
    totp_domain: ep_foundation::port::kms::KeyDomainId,
    policies: IdentityPolicies,
}

impl<U: UnitOfWork> LoginService<U> {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        uow: Arc<U>,
        admission: Arc<AdmissionGate>,
        accounts: Arc<dyn AccountStore>,
        credentials: Arc<dyn CredentialStore>,
        devices: Arc<dyn DeviceStore>,
        sessions: Arc<dyn SessionStore>,
        lockouts: Arc<dyn LockoutStore>,
        attempts: Arc<dyn LoginAttemptStore>,
        authz_query: Arc<dyn UserAuthzQuery>,
        audit: Arc<dyn AuditRecorder>,
        password_service: Arc<PasswordService>,
        challenges: Arc<MfaChallengeService>,
        kms: Arc<dyn KmsBackend>,
        totp_domain: ep_foundation::port::kms::KeyDomainId,
        policies: IdentityPolicies,
    ) -> Self {
        Self {
            uow,
            admission,
            accounts,
            credentials,
            devices,
            sessions,
            lockouts,
            attempts,
            authz_query,
            audit,
            password_service,
            challenges,
            kms,
            totp_domain,
            policies,
        }
    }

    /// sign-in 九步。准入拒绝直接 Err（503 语义）；账号不存在执行
    /// 固定成本伪 Argon2id 后再走失败事务，使响应时间同分布。
    pub async fn sign_in(
        &self,
        req: SignInRequest,
        now: DateTime<Utc>,
    ) -> Result<SignInOutcome, AppError> {
        let client = client_kind(&req.client);
        let request_id = req.request_id.clone();
        let trace_id = req.trace_id.clone();
        let pre_ctx = pre_auth_lookup_ctx(&req)?;
        let _permit = self.admission.admit(&pre_ctx).await?;
        let (uow, accounts, credentials, devices, sessions, lockouts, attempts) = (
            self.uow.clone(),
            self.accounts.clone(),
            self.credentials.clone(),
            self.devices.clone(),
            self.sessions.clone(),
            self.lockouts.clone(),
            self.attempts.clone(),
        );
        let (authz_q, audit, pws, challenges, policy) = (
            self.authz_query.clone(),
            self.audit.clone(),
            self.password_service.clone(),
            self.challenges.clone(),
            self.policies.clone(),
        );
        let outcome = uow
            .transact(&pre_ctx, move |tx| {
                Box::pin(run_sign_in_tx(
                    tx,
                    req,
                    now,
                    client,
                    accounts,
                    credentials,
                    devices,
                    sessions,
                    lockouts,
                    attempts,
                    authz_q,
                    audit,
                    pws,
                    challenges,
                    policy,
                ))
            })
            .await?;
        self.map_outcome(outcome, client, &request_id, &trace_id)
    }

    /// complete-mfa：校验无状态挑战后走会话建立收尾。
    pub async fn complete_mfa(
        &self,
        req: CompleteMfaRequest,
        now: DateTime<Utc>,
    ) -> Result<SignInSuccess, AppError> {
        let payload = self.challenges.verify(&req.challenge, now)?;
        let client = client_kind(&payload.client);
        let request_id = req.request_id.clone();
        let trace_id = req.trace_id.clone();
        let (uow, accounts, credentials, devices, sessions, lockouts, attempts) = (
            self.uow.clone(),
            self.accounts.clone(),
            self.credentials.clone(),
            self.devices.clone(),
            self.sessions.clone(),
            self.lockouts.clone(),
            self.attempts.clone(),
        );
        let (authz_q, audit, kms, policy, totp_domain) = (
            self.authz_query.clone(),
            self.audit.clone(),
            self.kms.clone(),
            self.policies.clone(),
            self.totp_domain,
        );
        let outcome = uow
            .transact(&system_ctx_for(&payload.user_id)?, move |tx| {
                Box::pin(run_complete_mfa_tx(
                    tx,
                    payload,
                    req,
                    now,
                    client,
                    accounts,
                    credentials,
                    devices,
                    sessions,
                    lockouts,
                    attempts,
                    authz_q,
                    audit,
                    kms,
                    policy,
                    totp_domain,
                ))
            })
            .await?;
        match self.map_outcome(outcome, client, &request_id, &trace_id)? {
            SignInOutcome::Authenticated(s) => Ok(*s),
            SignInOutcome::MfaRequired { .. } => Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "complete-mfa 不得再要求第二因子",
            )),
        }
    }

    /// 事务产物 → 对外结果：Rejected 映射错误码，成功装配 19 字段上下文。
    fn map_outcome(
        &self,
        outcome: LoginTxOutcome,
        client: ClientKind,
        request_id: &str,
        trace_id: &str,
    ) -> Result<SignInOutcome, AppError> {
        match outcome {
            LoginTxOutcome::Rejected { rejection, .. } => {
                let (code, msg) = rejection.code();
                Err(AppError::new(code, msg.to_string()))
            }
            LoginTxOutcome::MfaPending { challenge } => {
                Ok(SignInOutcome::MfaRequired { challenge })
            }
            LoginTxOutcome::Succeeded {
                account,
                authz,
                token,
                session_id,
                active_le,
                device_id,
                expires_at,
                idle_expires_at,
                is_breakglass,
            } => {
                let context = build_session_context(SessionContextInput {
                    account: (*account).clone(),
                    session_id,
                    legal_entity_id: active_le,
                    device_id,
                    client,
                    authz: *authz,
                    request_id: request_id.to_string(),
                    trace_id: trace_id.to_string(),
                    is_breakglass,
                });
                // 字符集以 foundation 冻结实现为准：非法设备标识回落预认证形态。
                let context = match context {
                    Ok(c) => c,
                    Err(_) => pre_auth_context(&account, "PREAUTH", request_id, trace_id)?,
                };
                Ok(SignInOutcome::Authenticated(Box::new(SignInSuccess {
                    session_token: token,
                    session_id,
                    user_id: account.id.as_uuid(),
                    active_legal_entity_id: active_le,
                    expires_at,
                    idle_expires_at,
                    is_breakglass,
                    context,
                })))
            }
        }
    }
}

/// 客户端字面量到 ClientKind（非法回落 Ops，登记面已校验）。
pub fn client_kind(raw: &str) -> ClientKind {
    match raw {
        "win" => ClientKind::Win,
        "mac" => ClientKind::Mac,
        "ios" => ClientKind::Ios,
        "android" => ClientKind::Android,
        "portal" => ClientKind::Portal,
        _ => ClientKind::Ops,
    }
}

/// sign-in 事务的法人上下文：请求侧缺省主体（账号尚未查明）。
fn pre_auth_lookup_ctx(req: &SignInRequest) -> Result<SecurityContext, AppError> {
    let le = ep_foundation::id::Id::<LegalEntity>::from_uuid(uuid::Uuid::nil());
    let request_id = RequestId::new(&req.request_id).or_else(|_| RequestId::new("PREAUTH0000"))?;
    let trace_id = TraceId::new(&req.trace_id).or_else(|_| TraceId::new(&"0".repeat(32)))?;
    Ok(SecurityContext::system(le, request_id, trace_id))
}

/// complete-mfa 事务的法人上下文（无状态挑战已绑定用户；法人取缺省，
/// 身份九表无 RLS，不受影响）。
fn system_ctx_for(_user_id: &uuid::Uuid) -> Result<SecurityContext, AppError> {
    let request_id = RequestId::new("mfa00000000")
        .or_else(|_| RequestId::new("00000000"))
        .or_else(|_| RequestId::new("mfa-challenge"))
        .map_err(|e| {
            AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                format!("请求标识构造失败：{e}"),
            )
        })?;
    let trace_id = TraceId::new(&"0".repeat(32)).map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            format!("跟踪标识构造失败：{e}"),
        )
    })?;
    let le = ep_foundation::id::Id::<LegalEntity>::from_uuid(uuid::Uuid::nil());
    Ok(SecurityContext::system(le, request_id, trace_id))
}

#[expect(clippy::too_many_arguments)]
async fn run_sign_in_tx(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    req: SignInRequest,
    now: DateTime<Utc>,
    client: ClientKind,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    devices: Arc<dyn DeviceStore>,
    sessions: Arc<dyn SessionStore>,
    lockouts: Arc<dyn LockoutStore>,
    attempts: Arc<dyn LoginAttemptStore>,
    authz_q: Arc<dyn UserAuthzQuery>,
    audit: Arc<dyn AuditRecorder>,
    pws: Arc<PasswordService>,
    challenges: Arc<MfaChallengeService>,
    policy: IdentityPolicies,
) -> Result<LoginTxOutcome, AppError> {
    // 步 2：查账号；未知用户先烧固定成本再落失败流水（提交不回滚）。
    let Some(account) = accounts.find_by_login_name(tx, &req.login_name).await? else {
        pws.burn_dummy_cost();
        record_attempt(
            &*attempts,
            tx,
            None,
            &req,
            LoginAttemptOutcome::CredentialInvalid,
            client,
            now,
        )
        .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::CredentialInvalid("账号或口令无效"),
        });
    };
    if let Some(kind) = req.expected_kind {
        if account.account_kind != kind {
            record_attempt(
                &*attempts,
                tx,
                Some(account.id),
                &req,
                LoginAttemptOutcome::AccountInactive,
                client,
                now,
            )
            .await?;
            return Ok(LoginTxOutcome::Rejected {
                rejection: Rejection::AccountInactive,
            });
        }
    }
    // 步 3：锁定检查（FOR UPDATE 语义在 LockoutStore 实现体）。
    let lock_row = lockouts.lock_for_update(tx, account.id).await?;
    if lock_row.locked_until.is_some_and(|t| t > now) {
        record_attempt(
            &*attempts,
            tx,
            Some(account.id),
            &req,
            LoginAttemptOutcome::AccountLocked,
            client,
            now,
        )
        .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::AccountLocked,
        });
    }
    if !account.status.is_signable() {
        record_attempt(
            &*attempts,
            tx,
            Some(account.id),
            &req,
            LoginAttemptOutcome::AccountInactive,
            client,
            now,
        )
        .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::AccountInactive,
        });
    }
    // 步 4：第一因子（口令）。
    if let Some(rejection) = verify_first_factor(
        tx,
        &account,
        &req.password,
        &*credentials,
        &*attempts,
        &req,
        client,
        now,
        &pws,
        &policy,
    )
    .await?
    {
        let lock_p = &policy.lockout;
        lockouts
            .record_failure(
                tx,
                &lock_row,
                lock_p.max_failures,
                lock_p.window_seconds,
                lock_p.duration_seconds,
                now,
            )
            .await?;
        let attempt = LoginAttemptOutcome::CredentialInvalid;
        record_attempt(&*attempts, tx, Some(account.id), &req, attempt, client, now).await?;
        return Ok(LoginTxOutcome::Rejected { rejection });
    }
    // 步 5：MFA 判定（读 authz 快照面一次）。
    let authz = authz_q
        .load_user_authz(tx, account.id, account.home_legal_entity_id)
        .await?;
    if is_mfa_required(&account, &authz) {
        return mfa_pending_path(
            tx,
            &account,
            &authz,
            &req,
            now,
            client,
            &*credentials,
            &*attempts,
            &*lockouts,
            &challenges,
        )
        .await;
    }
    // 步 6-9：设备 → 会话上限 → 令牌 → 提交。
    finish_login(
        tx,
        account,
        authz,
        req.device_id,
        false,
        &req.source_addr,
        now,
        client,
        devices,
        sessions,
        lockouts,
        attempts,
        audit,
        policy,
    )
    .await
}

/// 第一因子校验：失败返回拒绝变体；口令过期视同无效（需重置）。
#[allow(clippy::too_many_arguments)]
async fn verify_first_factor(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    account: &UserAccountRow,
    password: &str,
    credentials: &dyn CredentialStore,
    attempts: &dyn LoginAttemptStore,
    req: &SignInRequest,
    client: ClientKind,
    now: DateTime<Utc>,
    pws: &PasswordService,
    policy: &IdentityPolicies,
) -> Result<Option<Rejection>, AppError> {
    let Some(cred) = credentials
        .active_of_kind(tx, account.id, CredentialKind::Password)
        .await?
    else {
        record_attempt(
            attempts,
            tx,
            Some(account.id),
            req,
            LoginAttemptOutcome::CredentialInvalid,
            client,
            now,
        )
        .await?;
        return Ok(Some(Rejection::CredentialInvalid("无口令凭据")));
    };
    let ok = cred
        .verifier
        .as_deref()
        .is_some_and(|phc| pws.verify(phc, password));
    let expired = PasswordService::is_expired(cred.created_at, now, &policy.password);
    if !ok || expired {
        return Ok(Some(Rejection::CredentialInvalid("账号或口令无效")));
    }
    Ok(None)
}

/// MFA 判定为强：已登记第二因子 → 签发挑战；未登记 → 拒绝。
#[allow(clippy::too_many_arguments)]
async fn mfa_pending_path(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    account: &UserAccountRow,
    _authz: &UserAuthzSet,
    req: &SignInRequest,
    now: DateTime<Utc>,
    client: ClientKind,
    credentials: &dyn CredentialStore,
    attempts: &dyn LoginAttemptStore,
    lockouts: &dyn LockoutStore,
    challenges: &MfaChallengeService,
) -> Result<LoginTxOutcome, AppError> {
    let seconds = credentials
        .list_active(tx, account.id)
        .await?
        .iter()
        .filter(|c| c.credential_kind.is_second_factor())
        .count();
    if seconds == 0 {
        record_attempt(
            attempts,
            tx,
            Some(account.id),
            req,
            LoginAttemptOutcome::MfaInvalid,
            client,
            now,
        )
        .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::MfaEnrollmentRequired,
        });
    }
    let challenge = challenges.issue(account.id.as_uuid(), &req.device_id, &req.client, now)?;
    record_attempt(
        attempts,
        tx,
        Some(account.id),
        req,
        LoginAttemptOutcome::MfaRequired,
        client,
        now,
    )
    .await?;
    // 第一因子已过：锁定计数重置，挑战期不受失败窗口影响。
    lockouts.reset(tx, account.id).await?;
    Ok(LoginTxOutcome::MfaPending { challenge })
}

/// 步 6-9 收尾：设备校验、会话上限、令牌、四项同事务提交。
#[expect(clippy::too_many_arguments)]
async fn finish_login(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    account: UserAccountRow,
    authz: UserAuthzSet,
    device_id: String,
    is_breakglass: bool,
    source_addr: &str,
    now: DateTime<Utc>,
    client: ClientKind,
    devices: Arc<dyn DeviceStore>,
    sessions: Arc<dyn SessionStore>,
    lockouts: Arc<dyn LockoutStore>,
    attempts: Arc<dyn LoginAttemptStore>,
    audit: Arc<dyn AuditRecorder>,
    policy: IdentityPolicies,
) -> Result<LoginTxOutcome, AppError> {
    // 步 6：设备校验与单法人限定（上下文取交集）。
    let Some(device) = devices.find_active(tx, account.id, &device_id).await? else {
        attempts
            .append(
                tx,
                NewLoginAttempt {
                    user_id: Some(account.id),
                    login_name_hash: crate::session::login_name_hash(&account.login_name).to_vec(),
                    outcome: LoginAttemptOutcome::DeviceUnregistered,
                    client,
                    source_addr: source_addr.to_string(),
                    occurred_at: now,
                },
            )
            .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::DeviceUnregistered,
        });
    };
    let active_le = device
        .restricted_legal_entity_id
        .unwrap_or(account.home_legal_entity_id);
    // 步 7：会话上限——已满载即撤最早一条（审计语义占位调用）。
    if sessions.list_active_for_user(tx, account.id).await?.len() >= policy.session.max_per_user {
        prune_over_limit(tx, &account, &*sessions, &*audit).await?;
    }
    // 步 8：令牌生成（明文只进响应，摘要入库）。
    let token = new_session_token();
    let digest = token_digest(&token);
    let (expires_at, idle_expires_at) = expiry_pair(now, &policy.session);
    // 步 9：会话 + 流水 + 锁定重置同事务提交。
    let session_id = sessions
        .insert(
            tx,
            NewSession {
                user_id: account.id,
                user_device_row_id: device.id,
                token_hash: digest.to_vec(),
                active_legal_entity_id: active_le,
                issued_at: now,
                expires_at,
                idle_expires_at,
                is_breakglass,
            },
        )
        .await?;
    attempts
        .append(
            tx,
            NewLoginAttempt {
                user_id: Some(account.id),
                login_name_hash: crate::session::login_name_hash(&account.login_name).to_vec(),
                outcome: LoginAttemptOutcome::Success,
                client,
                source_addr: source_addr.to_string(),
                occurred_at: now,
            },
        )
        .await?;
    lockouts.reset(tx, account.id).await?;
    Ok(LoginTxOutcome::Succeeded {
        account: Box::new(account),
        authz: Box::new(authz),
        token,
        session_id,
        active_le,
        device_id,
        expires_at,
        idle_expires_at,
        is_breakglass,
    })
}

/// 超限裁剪：撤最早活跃会话，理由 SESSION_LIMIT_EXCEEDED 落审计占位。
async fn prune_over_limit(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    account: &UserAccountRow,
    sessions: &dyn SessionStore,
    audit: &dyn AuditRecorder,
) -> Result<(), AppError> {
    let active = sessions.list_active_for_user(tx, account.id).await?;
    if let Some(victim) = active.iter().min_by_key(|s| s.issued_at) {
        sessions
            .revoke(tx, victim.id, REVOKE_SESSION_LIMIT_EXCEEDED)
            .await?;
        audit.record(
            "SESSION_LIMIT_EXCEEDED",
            &format!("user={}", account.id.as_uuid()),
        );
    }
    Ok(())
}

#[expect(clippy::too_many_arguments)]
async fn run_complete_mfa_tx(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    payload: MfaChallengePayload,
    req: CompleteMfaRequest,
    now: DateTime<Utc>,
    client: ClientKind,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    devices: Arc<dyn DeviceStore>,
    sessions: Arc<dyn SessionStore>,
    lockouts: Arc<dyn LockoutStore>,
    attempts: Arc<dyn LoginAttemptStore>,
    authz_q: Arc<dyn UserAuthzQuery>,
    audit: Arc<dyn AuditRecorder>,
    kms: Arc<dyn KmsBackend>,
    policy: IdentityPolicies,
    totp_domain: ep_foundation::port::kms::KeyDomainId,
) -> Result<LoginTxOutcome, AppError> {
    let user_id = ep_foundation::id::Id::from_uuid(payload.user_id);
    let Some(account) = accounts.get(tx, user_id).await? else {
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::AccountInactive,
        });
    };
    if !account.status.is_signable() {
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::AccountInactive,
        });
    }
    // 第二因子校验：失败同样提交（流水 + 锁定推进）。
    let verified = match &req.proof {
        SecondFactorProof::Totp { code } => {
            let cred = credentials
                .active_of_kind(tx, user_id, CredentialKind::Totp)
                .await?;
            match cred {
                Some(c) => {
                    crate::mfa::verify_totp_credential(
                        &*kms,
                        totp_domain,
                        &c,
                        code,
                        now,
                        policy.totp.skew_steps,
                    )
                    .await?
                }
                None => false,
            }
        }
    };
    if !verified {
        let lock_row = lockouts.lock_for_update(tx, user_id).await?;
        let lp = &policy.lockout;
        lockouts
            .record_failure(
                tx,
                &lock_row,
                lp.max_failures,
                lp.window_seconds,
                lp.duration_seconds,
                now,
            )
            .await?;
        attempts
            .append(
                tx,
                NewLoginAttempt {
                    user_id: Some(user_id),
                    login_name_hash: crate::session::login_name_hash(&account.login_name).to_vec(),
                    outcome: LoginAttemptOutcome::MfaInvalid,
                    client,
                    source_addr: req.source_addr.clone(),
                    occurred_at: now,
                },
            )
            .await?;
        return Ok(LoginTxOutcome::Rejected {
            rejection: Rejection::MfaInvalid,
        });
    }
    let authz = authz_q
        .load_user_authz(tx, user_id, account.home_legal_entity_id)
        .await?;
    let is_breakglass = account.account_kind == AccountKind::Breakglass;
    finish_login(
        tx,
        account,
        authz,
        payload.device_id,
        is_breakglass,
        &req.source_addr,
        now,
        client,
        devices,
        sessions,
        lockouts,
        attempts,
        audit,
        policy,
    )
    .await
}

/// 登录流水落库：登录名以 SHA-256 摘要入库。
async fn record_attempt(
    store: &dyn LoginAttemptStore,
    tx: &mut dyn ep_foundation::port::tx::Tx,
    user_id: Option<ep_foundation::id::Id<ep_foundation::id::marker::UserAccount>>,
    req: &SignInRequest,
    outcome: LoginAttemptOutcome,
    client: ClientKind,
    now: DateTime<Utc>,
) -> Result<(), AppError> {
    store
        .append(
            tx,
            NewLoginAttempt {
                user_id,
                login_name_hash: crate::session::login_name_hash(&req.login_name).to_vec(),
                outcome,
                client,
                source_addr: req.source_addr.clone(),
                occurred_at: now,
            },
        )
        .await
}

#[cfg(test)]
#[path = "login_tests.rs"]
mod tests;
