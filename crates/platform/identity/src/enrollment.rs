//! MFA 登记用例：TOTP 注册 begin/complete 与注销（04 §5.2，规格 §6.2）。
//!
//! 登记引用（enrollment_ref）无状态：base64url(紧凑 JSON 载荷 +
//! HMAC-SHA256)，寿命短（600 秒）且绑定用户，种子经 KmsBackend wrap
//! 后信封随引用携带，complete 时 unwrap 回放校验一次性码。
//! 最后因子禁删经 [`guard_last_factor`]（读账号 is_mfa_required）。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_ACCOUNT_INACTIVE, PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED,
    PLATFORM_AUTHN_MFA_INVALID, PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::kms::{Aad, CipherEnvelope, KeyDomainId, KmsBackend};
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::SecurityContext;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::config::IdentityPolicies;
use crate::mfa::{b64url_decode, guard_last_factor, wrap_totp_seed};
use crate::ports::{AccountStore, CredentialStore, NewCredential};
use crate::session::base64url_encode;
use crate::totp::verify_totp;
use crate::types::{AccountStatus, CredentialKind, CredentialStatus};

type HmacSha256 = Hmac<Sha256>;

/// 登记引用寿命：600 秒（一次性录入窗口，U-B-14 临时取值）。
pub const MFA_ENROLLMENT_TTL_SECONDS: i64 = 600;

/// begin 响应：引用与 base32 种子（种子仅此处出现一次）。
#[derive(Clone, Debug)]
pub struct TotpEnrollmentBegin {
    pub enrollment_ref: String,
    pub secret_base32: String,
}

/// 登记引用载荷（签名对象）。
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct TotpEnrollmentPayload {
    user_id: uuid::Uuid,
    secret_ref: String,
    envelope_hex: String,
    expires_at_unix: i64,
}

/// MFA 登记用例服务。
pub struct MfaEnrollmentService<U: UnitOfWork> {
    uow: Arc<U>,
    accounts: Arc<dyn AccountStore>,
    credentials: Arc<dyn CredentialStore>,
    kms: Arc<dyn KmsBackend>,
    totp_domain: KeyDomainId,
    key: [u8; 32],
    policies: IdentityPolicies,
}

impl<U: UnitOfWork> MfaEnrollmentService<U> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uow: Arc<U>,
        accounts: Arc<dyn AccountStore>,
        credentials: Arc<dyn CredentialStore>,
        kms: Arc<dyn KmsBackend>,
        totp_domain: KeyDomainId,
        policies: IdentityPolicies,
    ) -> Self {
        let mut key = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut key);
        Self {
            uow,
            accounts,
            credentials,
            kms,
            totp_domain,
            key,
            policies,
        }
    }

    #[cfg(test)]
    fn with_key(
        uow: Arc<U>,
        accounts: Arc<dyn AccountStore>,
        credentials: Arc<dyn CredentialStore>,
        kms: Arc<dyn KmsBackend>,
        totp_domain: KeyDomainId,
        key: [u8; 32],
        policies: IdentityPolicies,
    ) -> Self {
        Self {
            uow,
            accounts,
            credentials,
            kms,
            totp_domain,
            key,
            policies,
        }
    }

    /// TOTP 注册第一步：校验可登记性后 wrap 新种子，签发登记引用。
    pub async fn begin_totp(
        &self,
        ctx: &SecurityContext,
        now: DateTime<Utc>,
    ) -> Result<TotpEnrollmentBegin, AppError> {
        self.ensure_can_enroll(ctx).await?;
        let (secret_ref, envelope_hex, b32) = wrap_totp_seed(
            self.kms.as_ref(),
            self.totp_domain,
            ctx.user_id.as_uuid(),
            1,
        )
        .await?;
        let enrollment_ref =
            self.sign_reference(&secret_ref, &envelope_hex, ctx.user_id.as_uuid(), now)?;
        Ok(TotpEnrollmentBegin {
            enrollment_ref,
            secret_base32: b32,
        })
    }

    /// TOTP 注册第二步：验引用 → unwrap 种子 → 校验一次性码 → 落凭据。
    pub async fn complete_totp(
        &self,
        ctx: &SecurityContext,
        enrollment_ref: &str,
        code: &str,
        now: DateTime<Utc>,
    ) -> Result<uuid::Uuid, AppError> {
        let payload = self.verify_reference(enrollment_ref, ctx.user_id.as_uuid(), now)?;
        let seed = self.unwrap_seed(&payload).await?;
        let ok = verify_totp(
            &seed,
            code,
            u64::try_from(now.timestamp()).unwrap_or(0),
            self.policies.totp.skew_steps,
        );
        if !ok {
            return Err(AppError::new(
                PLATFORM_AUTHN_MFA_INVALID,
                "TOTP 码校验失败".to_string(),
            ));
        }
        self.insert_totp_credential(ctx, &payload).await
    }

    /// 注销本人名下第二因子凭据；最后因子禁删（账号强制 MFA 时）。
    /// 引用不属于本人返回 Ok(false)。
    pub async fn unenroll(
        &self,
        ctx: &SecurityContext,
        credential_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        let (accounts, credentials, user_id) =
            (self.accounts.clone(), self.credentials.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let Some(account) = accounts.get(tx, user_id).await? else {
                        return Err(missing_account());
                    };
                    let active = credentials.list_active(tx, user_id).await?;
                    let Some(target) = active.iter().find(|c| c.id == credential_id) else {
                        return Ok(false);
                    };
                    if !target.credential_kind.is_second_factor() {
                        return Err(AppError::new(
                            PLATFORM_AUTHN_MFA_INVALID,
                            "口令凭据不经本面注销".to_string(),
                        ));
                    }
                    let factors = active
                        .iter()
                        .filter(|c| c.credential_kind.is_second_factor())
                        .count();
                    guard_last_factor(factors, account.is_mfa_required)?;
                    credentials
                        .set_status(tx, credential_id, CredentialStatus::Revoked)
                        .await
                })
            })
            .await
    }

    /// 可登记性前置：账号存在且 ACTIVE，且无 ACTIVE 的 TOTP 凭据。
    async fn ensure_can_enroll(&self, ctx: &SecurityContext) -> Result<(), AppError> {
        let (accounts, credentials, user_id) =
            (self.accounts.clone(), self.credentials.clone(), ctx.user_id);
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let Some(account) = accounts.get(tx, user_id).await? else {
                        return Err(missing_account());
                    };
                    if account.status != AccountStatus::Active {
                        return Err(AppError::new(
                            PLATFORM_AUTHN_ACCOUNT_INACTIVE,
                            "账号非 ACTIVE，禁止登记 MFA".to_string(),
                        ));
                    }
                    let dup = credentials
                        .active_of_kind(tx, user_id, CredentialKind::Totp)
                        .await?;
                    if dup.is_some() {
                        return Err(duplicate_totp());
                    }
                    Ok(())
                })
            })
            .await
    }

    fn sign_reference(
        &self,
        secret_ref: &str,
        envelope_hex: &str,
        user_id: uuid::Uuid,
        now: DateTime<Utc>,
    ) -> Result<String, AppError> {
        let payload = TotpEnrollmentPayload {
            user_id,
            secret_ref: secret_ref.to_string(),
            envelope_hex: envelope_hex.to_string(),
            expires_at_unix: now.timestamp() + MFA_ENROLLMENT_TTL_SECONDS,
        };
        let body = serde_json::to_vec(&payload).map_err(|e| {
            AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                format!("MFA 登记引用序列化失败：{e}"),
            )
        })?;
        let mac = mac_of(&self.key, &body)?;
        let mut combined = body;
        combined.extend_from_slice(&mac);
        Ok(base64url_encode(&combined))
    }

    fn verify_reference(
        &self,
        enrollment_ref: &str,
        user_id: uuid::Uuid,
        now: DateTime<Utc>,
    ) -> Result<TotpEnrollmentPayload, AppError> {
        let bytes = b64url_decode(enrollment_ref).ok_or_else(invalid_reference)?;
        if bytes.len() <= 32 {
            return Err(invalid_reference());
        }
        let (body, mac) = bytes.split_at(bytes.len() - 32);
        let want = mac_of(&self.key, body)?;
        if !constant_time_eq(mac, &want) {
            return Err(invalid_reference());
        }
        let payload: TotpEnrollmentPayload =
            serde_json::from_slice(body).map_err(|_| invalid_reference())?;
        if payload.user_id != user_id {
            return Err(invalid_reference());
        }
        if payload.expires_at_unix < now.timestamp() {
            return Err(AppError::new(
                PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED,
                "MFA 登记引用已过期".to_string(),
            ));
        }
        Ok(payload)
    }

    async fn unwrap_seed(&self, payload: &TotpEnrollmentPayload) -> Result<Vec<u8>, AppError> {
        let aad = Aad::new(payload.secret_ref.as_bytes().to_vec());
        let bytes = ep_platform_authz::types::hex_decode(&payload.envelope_hex)
            .ok_or_else(invalid_reference)?;
        let envelope = CipherEnvelope::new(bytes);
        self.kms.unwrap(self.totp_domain, &aad, &envelope).await
    }

    async fn insert_totp_credential(
        &self,
        ctx: &SecurityContext,
        payload: &TotpEnrollmentPayload,
    ) -> Result<uuid::Uuid, AppError> {
        let (credentials, user_id) = (self.credentials.clone(), ctx.user_id);
        let secret_ref = payload.secret_ref.clone();
        let envelope_hex = payload.envelope_hex.clone();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    // 并发面二次去重：begin 与 complete 之间的窗口。
                    let dup = credentials
                        .active_of_kind(tx, user_id, CredentialKind::Totp)
                        .await?;
                    if dup.is_some() {
                        return Err(duplicate_totp());
                    }
                    credentials
                        .insert(
                            tx,
                            NewCredential {
                                user_id,
                                credential_kind: CredentialKind::Totp,
                                verifier: Some(envelope_hex),
                                public_key: None,
                                credential_handle: None,
                                secret_ref: Some(secret_ref),
                            },
                        )
                        .await
                })
            })
            .await
    }
}

fn missing_account() -> AppError {
    AppError::new(
        PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
        "账号不存在，无法登记 MFA".to_string(),
    )
}

fn duplicate_totp() -> AppError {
    AppError::new(
        PLATFORM_AUTHN_MFA_INVALID,
        "TOTP 已登记，重复登记拒".to_string(),
    )
}

fn invalid_reference() -> AppError {
    AppError::new(
        PLATFORM_AUTHN_MFA_INVALID,
        "MFA 登记引用形态非法".to_string(),
    )
}

fn mac_of(key: &[u8; 32], body: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            format!("MFA 登记引用签名失败：{e}"),
        )
    })?;
    mac.update(body);
    Ok(mac.finalize().into_bytes().to_vec())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_build::pre_auth_context;
    use crate::testutil::{
        lock, mem, FakeKms, InMemoryUow, MemAccountStore, MemCredentialStore, MemHandle,
    };
    use crate::totp::{totp_code, totp_secret_ref};
    use crate::types::UserAccountRow;
    use ep_foundation::error::codes::PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN;
    use ep_foundation::id::marker::{LegalEntity, UserAccount};
    use ep_foundation::id::Id;

    const DOMAIN: u128 = 0xA11CE;

    fn account_row(id: u128, mfa_required: bool, status: AccountStatus) -> UserAccountRow {
        UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(id)),
            account_kind: crate::types::AccountKind::Employee,
            login_name: format!("u{id:x}"),
            employee_no: None,
            display_name: format!("U{id:x}"),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: mfa_required,
            created_at: Utc::now(),
        }
    }

    fn ctx_of(id: u128) -> SecurityContext {
        let account = account_row(id, false, AccountStatus::Active);
        pre_auth_context(&account, "DEV-01", "req00001", &"0".repeat(32)).expect("合法")
    }

    fn svc(h: &MemHandle, kms: Arc<FakeKms>) -> MfaEnrollmentService<InMemoryUow> {
        MfaEnrollmentService::with_key(
            Arc::new(InMemoryUow),
            Arc::new(MemAccountStore(h.clone())),
            Arc::new(MemCredentialStore(h.clone())),
            kms,
            KeyDomainId(uuid::Uuid::from_u128(DOMAIN)),
            [9; 32],
            IdentityPolicies::default(),
        )
    }

    fn seed_of(kms: &FakeKms, user: u128) -> Vec<u8> {
        let reference = totp_secret_ref(uuid::Uuid::from_u128(user), 1);
        lock_free(kms)
            .get(reference.as_bytes())
            .cloned()
            .expect("种子已 wrap")
    }

    fn lock_free(
        kms: &FakeKms,
    ) -> std::sync::MutexGuard<'_, std::collections::HashMap<Vec<u8>, Vec<u8>>> {
        kms.store.lock().unwrap_or_else(|p| p.into_inner())
    }

    #[tokio::test]
    async fn begin_then_complete_registers_totp_credential() {
        let h = mem();
        lock(&h)
            .accounts
            .push(account_row(1, false, AccountStatus::Active));
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms.clone());
        let c = ctx_of(1);
        let now = Utc::now();
        let begin = s.begin_totp(&c, now).await.expect("begin 通过");
        assert!(!begin.secret_base32.is_empty(), "base32 种子仅出现一次");
        let seed = seed_of(&kms, 1);
        let code = totp_code(&seed, now.timestamp() as u64).expect("合法种子");
        let id = s
            .complete_totp(&c, &begin.enrollment_ref, &code, now)
            .await
            .expect("complete 通过");
        let creds = lock(&h).credentials.clone();
        assert_eq!(creds.len(), 1);
        assert_eq!(creds[0].id, id);
        assert_eq!(creds[0].credential_kind, CredentialKind::Totp);
        assert_eq!(
            creds[0].secret_ref.as_deref(),
            Some(totp_secret_ref(uuid::Uuid::from_u128(1), 1).as_str())
        );
    }

    #[tokio::test]
    async fn begin_rejects_missing_inactive_or_duplicate() {
        let h = mem();
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms.clone());
        let now = Utc::now();
        let err = s.begin_totp(&ctx_of(7), now).await.expect_err("无账号拒");
        assert_eq!(err.code, PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED);
        lock(&h)
            .accounts
            .push(account_row(8, false, AccountStatus::Suspended));
        let err = s
            .begin_totp(&ctx_of(8), now)
            .await
            .expect_err("非 ACTIVE 拒");
        assert_eq!(err.code, PLATFORM_AUTHN_ACCOUNT_INACTIVE);
        lock(&h)
            .accounts
            .push(account_row(9, false, AccountStatus::Active));
        let begin = s.begin_totp(&ctx_of(9), now).await.expect("begin");
        // 未 complete 不落凭据，二次 begin 仍放行；拦截点在 active_of_kind。
        assert!(s.begin_totp(&ctx_of(9), now).await.is_ok());
        drop(begin);
    }

    #[tokio::test]
    async fn complete_rejects_wrong_code_expired_or_foreign_reference() {
        let h = mem();
        lock(&h)
            .accounts
            .push(account_row(1, false, AccountStatus::Active));
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms.clone());
        let c = ctx_of(1);
        let now = Utc::now();
        let begin = s.begin_totp(&c, now).await.expect("begin");
        let seed = seed_of(&kms, 1);
        let wrong = {
            let code = totp_code(&seed, now.timestamp() as u64).expect("码");
            let v: u32 = code.parse().unwrap_or(0);
            format!("{:06}", (v + 1) % 1_000_000)
        };
        let err = s
            .complete_totp(&c, &begin.enrollment_ref, &wrong, now)
            .await
            .expect_err("错码拒");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_INVALID);
        let later = now + chrono::Duration::seconds(MFA_ENROLLMENT_TTL_SECONDS + 1);
        let code = totp_code(&seed, later.timestamp() as u64).expect("码");
        let err = s
            .complete_totp(&c, &begin.enrollment_ref, &code, later)
            .await
            .expect_err("过期拒");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED);
        let other = ctx_of(2);
        lock(&h)
            .accounts
            .push(account_row(2, false, AccountStatus::Active));
        let err = s
            .complete_totp(&other, &begin.enrollment_ref, &code, now)
            .await
            .expect_err("他人引用拒");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_INVALID);
        assert!(s.complete_totp(&c, "garbage", "000000", now).await.is_err());
    }

    #[tokio::test]
    async fn complete_blocks_duplicate_after_direct_insert() {
        let h = mem();
        lock(&h)
            .accounts
            .push(account_row(1, false, AccountStatus::Active));
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms.clone());
        let c = ctx_of(1);
        let now = Utc::now();
        let begin = s.begin_totp(&c, now).await.expect("begin");
        // 模拟并发面另一路已落 TOTP 凭据。
        lock(&h).credentials.push(crate::types::CredentialRow {
            id: uuid::Uuid::now_v7(),
            user_id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            credential_kind: CredentialKind::Totp,
            verifier: Some("aa".into()),
            public_key: None,
            credential_handle: None,
            secret_ref: Some("secret://kms/totp/x#0".into()),
            sign_count: 0,
            status: CredentialStatus::Active,
            security_level: 0,
            created_at: now,
        });
        let seed = seed_of(&kms, 1);
        let code = totp_code(&seed, now.timestamp() as u64).expect("码");
        let err = s
            .complete_totp(&c, &begin.enrollment_ref, &code, now)
            .await
            .expect_err("重复登记拒");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_INVALID);
        assert_eq!(lock(&h).credentials.len(), 1, "未新增凭据");
    }

    #[tokio::test]
    async fn unenroll_revokes_factor_and_guards_last_one() {
        let h = mem();
        lock(&h)
            .accounts
            .push(account_row(1, true, AccountStatus::Active));
        let user = Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(1));
        for kind in [CredentialKind::Totp, CredentialKind::WebauthnPlatform] {
            lock(&h).credentials.push(crate::types::CredentialRow {
                id: uuid::Uuid::now_v7(),
                user_id: user,
                credential_kind: kind,
                verifier: None,
                public_key: None,
                credential_handle: None,
                secret_ref: None,
                sign_count: 0,
                status: CredentialStatus::Active,
                security_level: 0,
                created_at: Utc::now(),
            });
        }
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms);
        let c = ctx_of(1);
        let webauthn = lock(&h).credentials[1].id;
        assert!(
            s.unenroll(&c, webauthn).await.expect("注销第二因子"),
            "两因子可删一"
        );
        let totp = lock(&h).credentials[0].id;
        let err = s.unenroll(&c, totp).await.expect_err("最后因子禁删");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN);
        assert!(
            !s.unenroll(&c, uuid::Uuid::now_v7()).await.expect("查询"),
            "非本人引用返 false"
        );
    }

    #[tokio::test]
    async fn unenroll_allows_last_factor_when_not_mandatory() {
        let h = mem();
        lock(&h)
            .accounts
            .push(account_row(1, false, AccountStatus::Active));
        let user = Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(1));
        lock(&h).credentials.push(crate::types::CredentialRow {
            id: uuid::Uuid::now_v7(),
            user_id: user,
            credential_kind: CredentialKind::Totp,
            verifier: None,
            public_key: None,
            credential_handle: None,
            secret_ref: None,
            sign_count: 0,
            status: CredentialStatus::Active,
            security_level: 0,
            created_at: Utc::now(),
        });
        let kms = Arc::new(FakeKms::new());
        let s = svc(&h, kms);
        let id = lock(&h).credentials[0].id;
        assert!(s.unenroll(&ctx_of(1), id).await.expect("非强制可删到零"));
        assert_eq!(lock(&h).credentials[0].status, CredentialStatus::Revoked);
    }
}
