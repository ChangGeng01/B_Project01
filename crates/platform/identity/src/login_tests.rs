//! 登录九步主线测试：harness 以内存 UoW + 全端口 Fake 注入。
//!
//! 覆盖：成功建会话、未知用户固定成本、锁定策略边界（5/15/30）、
//! 强制 MFA 三支判定、会话上限裁剪、设备校验、complete-mfa TOTP
//! 全链与失败路径提交不回滚语义。

use std::sync::Arc;

use chrono::{Duration, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_ACCOUNT_LOCKED, PLATFORM_AUTHN_CREDENTIAL_INVALID,
    PLATFORM_AUTHN_DEVICE_NOT_REGISTERED, PLATFORM_AUTHN_MFA_INVALID,
    PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED,
};
use ep_foundation::error::{AppError, ErrorCode};
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::port::kms::KeyDomainId;
use ep_foundation::security::context::{ClientKind, DutyClass};
use ep_platform_authz::{AdmissionConfig, AdmissionGate, SilentMetricsSink};

use super::*;
use crate::config::{Argon2Params, IdentityPolicies};
use crate::password::PasswordService;
use crate::session::token_digest;
use crate::testutil::{
    attempt_outcomes, lock, mem, FakeKms, InMemoryUow, MemAccountStore, MemAudit,
    MemCredentialStore, MemDeviceStore, MemHandle, MemLockoutStore, MemLoginAttemptStore,
    MemSessionStore,
};
use crate::testutil_extra::MemUserAuthzQuery;
use crate::totp::{totp_code, totp_secret_ref};
use crate::types::{
    AccountKind, AccountStatus, CredentialKind, CredentialRow, CredentialStatus, DeviceRow,
    DeviceStatus, LoginAttemptOutcome, SessionRow, UserAccountRow, REVOKE_SESSION_LIMIT_EXCEEDED,
};

const HOME_LE: u128 = 0x1E;

struct Harness {
    h: MemHandle,
    svc: LoginService<InMemoryUow>,
    kms: Arc<FakeKms>,
    pws: Arc<PasswordService>,
}

fn harness() -> Harness {
    let h = mem();
    let kms = Arc::new(FakeKms::new());
    // 测试用最小参数：dev 档未优化，默认 64MiB 在单测里过重。
    let pws = Arc::new(
        PasswordService::new(Argon2Params {
            memory_kib: 8,
            iterations: 1,
            parallelism: 1,
        })
        .expect("测试参数合法"),
    );
    let svc = LoginService::new(
        Arc::new(InMemoryUow),
        Arc::new(AdmissionGate::new(
            AdmissionConfig::default(),
            Arc::new(SilentMetricsSink),
        )),
        Arc::new(MemAccountStore(h.clone())),
        Arc::new(MemCredentialStore(h.clone())),
        Arc::new(MemDeviceStore(h.clone())),
        Arc::new(MemSessionStore(h.clone())),
        Arc::new(MemLockoutStore(h.clone())),
        Arc::new(MemLoginAttemptStore(h.clone())),
        Arc::new(MemUserAuthzQuery(h.clone())),
        Arc::new(MemAudit(h.clone())),
        pws.clone(),
        Arc::new(MfaChallengeService::new(MFA_CHALLENGE_TTL_SECONDS)),
        kms.clone(),
        KeyDomainId(uuid::Uuid::from_u128(0xA11CE)),
        IdentityPolicies::default(),
    );
    Harness { h, svc, kms, pws }
}

fn seed_account(h: &MemHandle, n: u128, login: &str, mfa_required: bool) -> Id<UserAccount> {
    let id = Id::from_uuid(uuid::Uuid::from_u128(n));
    lock(h).accounts.push(UserAccountRow {
        id,
        account_kind: AccountKind::Employee,
        login_name: login.to_string(),
        employee_no: None,
        display_name: login.to_string(),
        home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(HOME_LE)),
        status: AccountStatus::Active,
        clearance_level: 20,
        security_level: 30,
        is_mfa_required: mfa_required,
        created_at: Utc::now(),
    });
    id
}

fn seed_password(h: &MemHandle, user: Id<UserAccount>, phc: String) {
    lock(h).credentials.push(CredentialRow {
        id: uuid::Uuid::from_u128(0xC0DE),
        user_id: user,
        credential_kind: CredentialKind::Password,
        verifier: Some(phc),
        public_key: None,
        credential_handle: None,
        secret_ref: None,
        sign_count: 0,
        status: CredentialStatus::Active,
        security_level: 0,
        created_at: Utc::now(),
    });
}

fn seed_device(
    h: &MemHandle,
    user: Id<UserAccount>,
    restricted: Option<Id<LegalEntity>>,
) -> uuid::Uuid {
    let id = uuid::Uuid::from_u128(0xDE01CE);
    lock(h).devices.push(DeviceRow {
        id,
        user_id: user,
        device_id: "DEV-01".to_string(),
        client: ClientKind::Win,
        public_key: None,
        attestation_ref: None,
        restricted_legal_entity_id: restricted,
        status: DeviceStatus::Active,
    });
    id
}

fn seed_session(h: &MemHandle, user: Id<UserAccount>, issued_at: chrono::DateTime<Utc>) {
    lock(h).sessions.push(SessionRow {
        id: uuid::Uuid::now_v7(),
        user_id: user,
        user_device_row_id: uuid::Uuid::from_u128(0xDE01CE),
        token_hash: vec![0],
        active_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(HOME_LE)),
        issued_at,
        expires_at: issued_at + Duration::hours(8),
        idle_expires_at: issued_at + Duration::minutes(30),
        last_seen_at: issued_at,
        revoked_at: None,
        revoke_reason: None,
        is_breakglass: false,
    });
}

/// TOTP 凭据直建：FakeKms 仓内登记 aad→种子，信封即 aad 字节。
fn seed_totp(hn: &Harness, user: Id<UserAccount>) -> [u8; 20] {
    let seed = [7u8; 20];
    let secret_ref = totp_secret_ref(user.as_uuid(), 1);
    let aad_bytes = secret_ref.as_bytes().to_vec();
    hn.kms
        .store
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .insert(aad_bytes.clone(), seed.to_vec());
    let verifier_hex = ep_platform_authz::types::hex_encode(&aad_bytes);
    lock(&hn.h).credentials.push(CredentialRow {
        id: uuid::Uuid::from_u128(0x7070),
        user_id: user,
        credential_kind: CredentialKind::Totp,
        verifier: Some(verifier_hex),
        public_key: None,
        credential_handle: None,
        secret_ref: Some(secret_ref),
        sign_count: 0,
        status: CredentialStatus::Active,
        security_level: 0,
        created_at: Utc::now(),
    });
    seed
}

fn req(login: &str, password: &str) -> SignInRequest {
    SignInRequest {
        login_name: login.to_string(),
        password: password.to_string(),
        device_id: "DEV-01".to_string(),
        client: "win".to_string(),
        source_addr: "127.0.0.1".to_string(),
        request_id: "req0000001".to_string(),
        trace_id: "0".repeat(32),
        expected_kind: None,
    }
}

fn must_err<T>(r: Result<T, AppError>) -> ErrorCode {
    match r {
        Ok(_) => panic!("应当拒绝"),
        Err(e) => e.code,
    }
}

fn must_ok(r: Result<SignInOutcome, AppError>) -> SignInOutcome {
    match r {
        Ok(o) => o,
        Err(e) => panic!("应当成功：{}", e.code.0),
    }
}

fn must_success(r: Result<SignInSuccess, AppError>) -> SignInSuccess {
    match r {
        Ok(s) => s,
        Err(e) => panic!("应当成功：{}", e.code.0),
    }
}

#[tokio::test]
async fn happy_path_creates_session_and_resets_lockout() {
    let hn = harness();
    let user = seed_account(&hn.h, 1, "alice", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let now = Utc::now();
    // 先失败一次推进锁定计数，再成功：验证成功即重置。
    let _ = must_err(hn.svc.sign_in(req("alice", "wrong-pass-XX"), now).await);
    assert_eq!(lock(&hn.h).lockouts[0].failure_count, 1);
    let out = must_ok(hn.svc.sign_in(req("alice", "Ab1!Ab1!Ab1!"), now).await);
    let SignInOutcome::Authenticated(s) = out else {
        panic!("应为直接成功")
    };
    assert_eq!(s.session_token.len(), 43, "32 字节令牌 base64url 43 位");
    assert_eq!(s.user_id, user.as_uuid());
    assert!(!s.is_breakglass);
    assert_eq!(s.context.user_id, user, "19 字段上下文用户段一致");
    let st = lock(&hn.h);
    assert_eq!(st.sessions.len(), 1);
    assert_eq!(
        st.sessions[0].token_hash,
        token_digest(&s.session_token).to_vec()
    );
    assert_eq!(st.lockouts[0].failure_count, 0, "成功即重置锁定计数");
    assert!(st.lockouts[0].locked_until.is_none());
    drop(st);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![
            LoginAttemptOutcome::CredentialInvalid,
            LoginAttemptOutcome::Success
        ]
    );
}

#[tokio::test]
async fn unknown_user_burns_dummy_cost_and_commits_attempt() {
    let hn = harness();
    let now = Utc::now();
    let err = must_err(hn.svc.sign_in(req("ghost", "Ab1!Ab1!Ab1!"), now).await);
    assert_eq!(err, PLATFORM_AUTHN_CREDENTIAL_INVALID);
    let st = lock(&hn.h);
    assert_eq!(st.attempts.len(), 1, "失败流水提交不回滚");
    assert!(st.attempts[0].user_id.is_none(), "未知用户不绑定 user_id");
    assert_eq!(
        st.attempts[0].outcome,
        LoginAttemptOutcome::CredentialInvalid
    );
}

#[tokio::test]
async fn wrong_password_advances_lockout_and_commits() {
    let hn = harness();
    let user = seed_account(&hn.h, 2, "bob", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let now = Utc::now();
    let err = must_err(hn.svc.sign_in(req("bob", "Wrong-1-pass"), now).await);
    assert_eq!(err, PLATFORM_AUTHN_CREDENTIAL_INVALID);
    let st = lock(&hn.h);
    let row = st
        .lockouts
        .iter()
        .find(|r| r.user_id == user)
        .expect("锁定行在");
    assert_eq!(row.failure_count, 1);
    assert!(row.window_started_at.is_some(), "失败窗口已开启");
    drop(st);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![LoginAttemptOutcome::CredentialInvalid]
    );
}

#[tokio::test]
async fn five_failures_lock_account_and_window_blocks() {
    let hn = harness();
    let user = seed_account(&hn.h, 3, "carol", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let now = Utc::now();
    for _ in 0..5 {
        let err = must_err(hn.svc.sign_in(req("carol", "Wrong-1-pass"), now).await);
        assert_eq!(err, PLATFORM_AUTHN_CREDENTIAL_INVALID);
    }
    assert!(
        lock(&hn.h)
            .lockouts
            .iter()
            .find(|r| r.user_id == user)
            .and_then(|r| r.locked_until)
            .is_some(),
        "五连败触发锁定"
    );
    // 锁定期内即使口令正确也拒。
    let err = must_err(hn.svc.sign_in(req("carol", "Ab1!Ab1!Ab1!"), now).await);
    assert_eq!(err, PLATFORM_AUTHN_ACCOUNT_LOCKED);
    let outcomes = attempt_outcomes(&hn.h);
    assert_eq!(outcomes.len(), 6);
    assert_eq!(outcomes[5], LoginAttemptOutcome::AccountLocked);
}

#[tokio::test]
async fn duty_role_forces_mfa_and_issues_challenge() {
    let hn = harness();
    let user = seed_account(&hn.h, 4, "dave", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    seed_totp(&hn, user);
    lock(&hn.h).duties.push(DutyClass::Security);
    let now = Utc::now();
    let out = must_ok(hn.svc.sign_in(req("dave", "Ab1!Ab1!Ab1!"), now).await);
    let SignInOutcome::MfaRequired { challenge } = out else {
        panic!("duty 角色应强制 MFA")
    };
    assert!(!challenge.is_empty());
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![LoginAttemptOutcome::MfaRequired]
    );
    assert!(lock(&hn.h).sessions.is_empty(), "挑战期不建会话");
}

#[tokio::test]
async fn mandatory_without_second_factor_requires_enrollment() {
    let hn = harness();
    let user = seed_account(&hn.h, 5, "erin", true);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let now = Utc::now();
    let err = must_err(hn.svc.sign_in(req("erin", "Ab1!Ab1!Ab1!"), now).await);
    assert_eq!(err, PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![LoginAttemptOutcome::MfaInvalid]
    );
}

#[tokio::test]
async fn session_cap_prunes_oldest_with_audit() {
    let hn = harness();
    let user = seed_account(&hn.h, 6, "frank", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let now = Utc::now();
    seed_session(&hn.h, user, now - Duration::hours(3));
    seed_session(&hn.h, user, now - Duration::hours(2));
    seed_session(&hn.h, user, now - Duration::hours(1));
    let out = must_ok(hn.svc.sign_in(req("frank", "Ab1!Ab1!Ab1!"), now).await);
    assert!(matches!(out, SignInOutcome::Authenticated(_)));
    let st = lock(&hn.h);
    let revoked: Vec<_> = st
        .sessions
        .iter()
        .filter(|s| s.revoked_at.is_some())
        .collect();
    assert_eq!(revoked.len(), 1, "撤最早一条");
    assert_eq!(
        revoked[0].revoke_reason.as_deref(),
        Some(REVOKE_SESSION_LIMIT_EXCEEDED)
    );
    assert_eq!(
        st.sessions
            .iter()
            .filter(|s| s.revoked_at.is_none())
            .count(),
        3,
        "撤一补一保持上限"
    );
    assert!(
        st.audits.iter().any(|(k, _)| k == "SESSION_LIMIT_EXCEEDED"),
        "审计语义落占位面"
    );
}

#[tokio::test]
async fn unregistered_device_is_rejected_and_logged() {
    let hn = harness();
    let user = seed_account(&hn.h, 7, "gina", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    // 不登记设备。
    let now = Utc::now();
    let err = must_err(hn.svc.sign_in(req("gina", "Ab1!Ab1!Ab1!"), now).await);
    assert_eq!(err, PLATFORM_AUTHN_DEVICE_NOT_REGISTERED);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![LoginAttemptOutcome::DeviceUnregistered]
    );
}

#[tokio::test]
async fn complete_mfa_totp_full_chain_builds_context() {
    let hn = harness();
    let user = seed_account(&hn.h, 8, "henry", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let seed = seed_totp(&hn, user);
    lock(&hn.h).duties.push(DutyClass::Audit);
    let now = Utc::now();
    let out = must_ok(hn.svc.sign_in(req("henry", "Ab1!Ab1!Ab1!"), now).await);
    let SignInOutcome::MfaRequired { challenge } = out else {
        panic!("应要求 MFA")
    };
    let code = totp_code(&seed, u64::try_from(now.timestamp()).expect("正时刻")).expect("合法种子");
    let success = must_success(
        hn.svc
            .complete_mfa(
                CompleteMfaRequest {
                    challenge,
                    proof: SecondFactorProof::Totp { code },
                    source_addr: "127.0.0.1".to_string(),
                    request_id: "req0000002".to_string(),
                    trace_id: "0".repeat(32),
                },
                now,
            )
            .await,
    );
    assert!(!success.is_breakglass, "普通账号不标记应急");
    assert_eq!(success.user_id, user.as_uuid());
    assert_eq!(success.context.user_id, user);
    let st = lock(&hn.h);
    assert_eq!(st.sessions.len(), 1);
    assert_eq!(
        st.sessions[0].token_hash,
        token_digest(&success.session_token).to_vec()
    );
    drop(st);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![
            LoginAttemptOutcome::MfaRequired,
            LoginAttemptOutcome::Success
        ]
    );
}

#[tokio::test]
async fn complete_mfa_wrong_code_commits_failure_and_advances_lockout() {
    let hn = harness();
    let user = seed_account(&hn.h, 9, "iris", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    seed_device(&hn.h, user, None);
    let seed = seed_totp(&hn, user);
    lock(&hn.h).duties.push(DutyClass::Security);
    let now = Utc::now();
    let out = must_ok(hn.svc.sign_in(req("iris", "Ab1!Ab1!Ab1!"), now).await);
    let SignInOutcome::MfaRequired { challenge } = out else {
        panic!("应要求 MFA")
    };
    // 两个步长之外的码在 skew ±1 外（撞码概率忽略）。
    let bad =
        totp_code(&seed, u64::try_from(now.timestamp() + 60).expect("正时刻")).expect("合法种子");
    let err = must_err(
        hn.svc
            .complete_mfa(
                CompleteMfaRequest {
                    challenge,
                    proof: SecondFactorProof::Totp { code: bad },
                    source_addr: "127.0.0.1".to_string(),
                    request_id: "req0000003".to_string(),
                    trace_id: "0".repeat(32),
                },
                now,
            )
            .await,
    );
    assert_eq!(err, PLATFORM_AUTHN_MFA_INVALID);
    let st = lock(&hn.h);
    let row = st
        .lockouts
        .iter()
        .find(|r| r.user_id == user)
        .expect("锁定行在");
    assert_eq!(row.failure_count, 1, "第二因子失败推进锁定且提交");
    drop(st);
    assert_eq!(
        attempt_outcomes(&hn.h),
        vec![
            LoginAttemptOutcome::MfaRequired,
            LoginAttemptOutcome::MfaInvalid
        ]
    );
}

#[tokio::test]
async fn device_restriction_takes_intersection_over_home() {
    let hn = harness();
    let user = seed_account(&hn.h, 10, "jack", false);
    seed_password(&hn.h, user, hn.pws.hash("Ab1!Ab1!Ab1!").expect("哈希"));
    let restricted = Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(0x77));
    seed_device(&hn.h, user, Some(restricted));
    let now = Utc::now();
    let out = must_ok(hn.svc.sign_in(req("jack", "Ab1!Ab1!Ab1!"), now).await);
    let SignInOutcome::Authenticated(s) = out else {
        panic!("应成功")
    };
    assert_eq!(s.active_legal_entity_id, restricted, "单法人限定取交集");
}
