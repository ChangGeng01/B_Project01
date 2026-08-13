//! MFA 域：强制判据、最后因子禁删、登录挑战、TOTP/X509/WebAuthn 三形态。
//!
//! 强制 MFA 判据（04:L361）：`is_mfa_required` 真，或持任一 duty_class
//! 非空的有效角色授予，或持含六类高风险权限项的角色——后两支读
//! [`UserAuthzSet`]（platform_authz 用户维度读取面）。

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED, PLATFORM_AUTHN_MFA_INVALID,
    PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::kms::{Aad, KeyRef, KmsBackend, Signature};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::config::X509Policy;
use crate::ports::UserAuthzSet;
use crate::session::base64url_encode;
use crate::totp::verify_totp;
use crate::types::{CredentialKind, CredentialRow, UserAccountRow};

type HmacSha256 = Hmac<Sha256>;

/// 强制 MFA 判据三支的合取判定。
pub fn is_mfa_required(account: &UserAccountRow, authz: &UserAuthzSet) -> bool {
    account.is_mfa_required || authz.has_duty_role() || authz.has_high_risk_permission
}

/// 最后因子禁删：剩余 ACTIVE 第二因子数不足 2 且账号强制 MFA 时拒删。
pub fn guard_last_factor(
    active_second_factors: usize,
    account_mandatory: bool,
) -> Result<(), AppError> {
    if active_second_factors <= 1 && account_mandatory {
        return Err(AppError::new(
            PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN,
            "删除后无剩余第二因子且账号强制 MFA，禁止注销最后因子".to_string(),
        ));
    }
    Ok(())
}

/// 登录挑战载荷：sign-in 第一因子通过后签发，complete-mfa 凭它绑定身份。
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub struct MfaChallengePayload {
    pub user_id: uuid::Uuid,
    pub device_id: String,
    pub client: String,
    pub expires_at_unix: i64,
}

/// 无状态登录挑战服务：挑战 = base64url(紧凑 JSON 载荷 + HMAC-SHA256)。
/// 挑战寿命短（默认 5 分钟）且仅限登录二段使用；签名密钥为进程级随机
/// 材料，重启后旧挑战自然失效（单实例部署首版取舍，见汇报）。
pub struct MfaChallengeService {
    key: [u8; 32],
    ttl_seconds: u64,
}

impl MfaChallengeService {
    pub fn new(ttl_seconds: u64) -> Self {
        let mut key = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut key);
        Self { key, ttl_seconds }
    }

    #[cfg(test)]
    fn with_key(key: [u8; 32], ttl_seconds: u64) -> Self {
        Self { key, ttl_seconds }
    }

    /// 签发挑战（不入库；明文只出现在响应）。
    pub fn issue(
        &self,
        user_id: uuid::Uuid,
        device_id: &str,
        client: &str,
        now: DateTime<Utc>,
    ) -> Result<String, AppError> {
        let payload = MfaChallengePayload {
            user_id,
            device_id: device_id.to_string(),
            client: client.to_string(),
            expires_at_unix: now.timestamp() + i64::try_from(self.ttl_seconds).unwrap_or(i64::MAX),
        };
        let body = serde_json::to_vec(&payload).map_err(|e| {
            AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                format!("MFA 挑战序列化失败：{e}"),
            )
        })?;
        let mac = mac_of(&self.key, &body)?;
        let mut combined = body;
        combined.extend_from_slice(&mac);
        Ok(base64url_encode(&combined))
    }

    /// 校验挑战：MAC 不符返 None（与不存在不可区分）；过期返 Err。
    pub fn verify(
        &self,
        challenge: &str,
        now: DateTime<Utc>,
    ) -> Result<MfaChallengePayload, AppError> {
        let bytes = b64url_decode(challenge).ok_or_else(invalid_challenge)?;
        if bytes.len() <= 32 {
            return Err(invalid_challenge());
        }
        let (body, mac) = bytes.split_at(bytes.len() - 32);
        let want = mac_of(&self.key, body)?;
        if !constant_time_eq(mac, &want) {
            return Err(invalid_challenge());
        }
        let payload: MfaChallengePayload =
            serde_json::from_slice(body).map_err(|_| invalid_challenge())?;
        if payload.expires_at_unix < now.timestamp() {
            return Err(AppError::new(
                PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED,
                "MFA 挑战已过期".to_string(),
            ));
        }
        Ok(payload)
    }
}

fn invalid_challenge() -> AppError {
    AppError::new(PLATFORM_AUTHN_MFA_INVALID, "MFA 挑战形态非法".to_string())
}

fn mac_of(key: &[u8; 32], body: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            format!("MFA 挑战签名失败：{e}"),
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

/// TOTP 校验：解出种子后按 skew 判码。种子经 KmsBackend unwrap 取得，
/// 生命周期仅限本调用（不出函数、不落日志）。
pub async fn verify_totp_credential(
    kms: &dyn KmsBackend,
    domain: ep_foundation::port::kms::KeyDomainId,
    credential: &CredentialRow,
    code: &str,
    now: DateTime<Utc>,
    skew_steps: u32,
) -> Result<bool, AppError> {
    let Some(secret_ref) = &credential.secret_ref else {
        return Err(AppError::new(
            PLATFORM_AUTHN_MFA_INVALID,
            "TOTP 凭据缺 secret_ref".to_string(),
        ));
    };
    let aad = Aad::new(secret_ref.as_bytes().to_vec());
    let Some(envelope_hex) = &credential.verifier else {
        return Err(AppError::new(
            PLATFORM_AUTHN_MFA_INVALID,
            "TOTP 凭据缺信封载体".to_string(),
        ));
    };
    let envelope_bytes = ep_platform_authz::types::hex_decode(envelope_hex).ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "TOTP 信封十六进制形态非法".to_string(),
        )
    })?;
    let envelope = ep_foundation::port::kms::CipherEnvelope::new(envelope_bytes);
    let seed = kms.unwrap(domain, &aad, &envelope).await?;
    let ok = verify_totp(
        &seed,
        code,
        u64::try_from(now.timestamp()).unwrap_or(0),
        skew_steps,
    );
    // 种子即时清零，缩短明文驻留。
    drop(seed);
    Ok(ok)
}

/// TOTP 种子封装：wrap 后返回 (secret_ref, 信封十六进制, base32 种子)。
/// base32 种子仅在注册响应中出现一次（供认证器 App 录入）。
pub async fn wrap_totp_seed(
    kms: &dyn KmsBackend,
    domain: ep_foundation::port::kms::KeyDomainId,
    user_id: uuid::Uuid,
    version: u32,
) -> Result<(String, String, String), AppError> {
    let mut seed = [0u8; 20];
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut seed);
    let secret_ref = crate::totp::totp_secret_ref(user_id, version);
    let aad = Aad::new(secret_ref.as_bytes().to_vec());
    // KeyPurpose 四变体无 TOTP 专用值：种子属凭据载体，取 Field
    // （行内敏感字段信封加密）最近义，取舍见汇报。
    let envelope = kms
        .wrap(
            domain,
            ep_foundation::port::kms::KeyPurpose::Field,
            &aad,
            &seed,
        )
        .await?;
    let envelope_hex = ep_platform_authz::types::hex_encode(envelope.as_bytes());
    let b32 = base32_encode(&seed);
    Ok((secret_ref, envelope_hex, b32))
}

/// X509_CERT 第一因子：挑战签名验签。信任锚取配置的
/// `secret://pki/client_ca#1`，验签经 KmsBackend::verify。
/// 证书解析依赖最小化取舍：首版不解析 DER 证书链，凭据 verifier 存
/// 证书指纹（SHA-256 十六进制），挑战验签直接以信任锚 KeyRef 承接。
pub struct X509ChallengeVerifier {
    trust_anchor_ref: KeyRef,
}

impl X509ChallengeVerifier {
    pub fn new(policy: &X509Policy) -> Self {
        Self {
            trust_anchor_ref: KeyRef::new(policy.trust_anchor_ref.clone()),
        }
    }

    /// 挑战载荷 = 登录名与随机 nonce 的紧凑拼接；双方可重构。
    pub fn challenge_payload(login_name: &str, nonce: &str) -> Vec<u8> {
        format!("{login_name}:{nonce}").into_bytes()
    }

    pub async fn verify(
        &self,
        kms: &dyn KmsBackend,
        credential: &CredentialRow,
        payload: &[u8],
        signature: &[u8],
    ) -> Result<bool, AppError> {
        if credential.credential_kind != CredentialKind::X509Cert {
            return Ok(false);
        }
        kms.verify(
            &self.trust_anchor_ref,
            payload,
            &Signature::new(signature.to_vec()),
        )
        .await
    }
}

/// WebAuthn 断言校验（首版登记/断言面）：rp_id 与 origin 绑定、
/// 签名计数单调、ECDSA P-256 验签。验签消息为 SHA-256(挑战||rp_id)
/// 的定长摘要——完整 WebAuthn 二进制格式（authenticator_data/客户端数据
/// JSON 原文）的解析属后续加厚，首版取舍见汇报。
#[allow(clippy::too_many_arguments)]
pub fn verify_webauthn_assertion(
    stored_public_key_hex: &str,
    stored_sign_count: i64,
    claimed_sign_count: i64,
    challenge: &str,
    rp_id: &str,
    origin: &str,
    allowed_origins: &[String],
    signature_der: &[u8],
) -> Result<bool, AppError> {
    if !allowed_origins.iter().any(|o| o == origin) {
        return Ok(false);
    }
    if claimed_sign_count <= stored_sign_count {
        return Ok(false);
    }
    use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
    use sha2::Digest;
    let pk_bytes =
        ep_platform_authz::types::hex_decode(stored_public_key_hex).ok_or_else(|| {
            AppError::new(
                PLATFORM_AUTHN_MFA_INVALID,
                "WebAuthn 公钥形态非法".to_string(),
            )
        })?;
    let key = VerifyingKey::from_sec1_bytes(&pk_bytes)
        .map_err(|_| AppError::new(PLATFORM_AUTHN_MFA_INVALID, "WebAuthn 公钥非法".to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(challenge.as_bytes());
    hasher.update(rp_id.as_bytes());
    let digest = hasher.finalize();
    let sig = Signature::from_der(signature_der).map_err(|_| {
        AppError::new(
            PLATFORM_AUTHN_MFA_INVALID,
            "WebAuthn 签名形态非法".to_string(),
        )
    })?;
    Ok(key.verify(&digest, &sig).is_ok())
}

/// RFC 4648 base32 编码（无填充），供 TOTP 种子一次性展示。
pub fn base32_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = String::new();
    let mut buffer: u64 = 0;
    let mut bits = 0u32;
    for b in bytes {
        buffer = (buffer << 8) | u64::from(*b);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(char::from(ALPHABET[(buffer >> bits & 31) as usize]));
        }
    }
    if bits > 0 {
        out.push(char::from(ALPHABET[(buffer << (5 - bits) & 31) as usize]));
    }
    out
}

/// base64url 解码（挑战/登记引用反序列化用；仅接受无填充形态）。
pub(crate) fn b64url_decode(text: &str) -> Option<Vec<u8>> {
    let val = |c: u8| -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'-' => Some(62),
            b'_' => Some(63),
            _ => None,
        }
    };
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut bits = 0u32;
    for &c in bytes {
        let v = val(c)?;
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    // 尾随填充位必须全零，否则形态非规范（追加字符不可区分于原文）。
    if bits > 0 && acc & ((1u32 << bits) - 1) != 0 {
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AccountKind, AccountStatus};
    use ep_foundation::id::marker::LegalEntity;
    use ep_foundation::id::Id;
    use ep_foundation::security::context::DutyClass;

    fn account(mfa_required: bool) -> UserAccountRow {
        UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            account_kind: AccountKind::Employee,
            login_name: "alice".into(),
            employee_no: None,
            display_name: "Alice".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 20,
            security_level: 30,
            is_mfa_required: mfa_required,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn mandatory_mfa_has_three_branches() {
        let mut authz = UserAuthzSet::default();
        assert!(!is_mfa_required(&account(false), &authz), "三支皆空不强制");
        assert!(is_mfa_required(&account(true), &authz), "账号标记强制");
        authz.duty_classes = vec![DutyClass::Security];
        assert!(is_mfa_required(&account(false), &authz), "duty 角色强制");
        let authz = UserAuthzSet {
            has_high_risk_permission: true,
            ..Default::default()
        };
        assert!(is_mfa_required(&account(false), &authz), "高风险权限强制");
    }

    #[test]
    fn last_factor_is_guarded_only_when_mandatory() {
        assert!(guard_last_factor(1, true).is_err());
        assert!(guard_last_factor(0, true).is_err());
        assert!(guard_last_factor(1, false).is_ok(), "非强制账号可删到零");
        assert!(guard_last_factor(2, true).is_ok(), "剩余两因子可删");
        let err = guard_last_factor(1, true).expect_err("拒");
        assert_eq!(err.code, PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN);
    }

    #[test]
    fn challenge_round_trips_and_expires() {
        let svc = MfaChallengeService::with_key([7; 32], 300);
        let now = Utc::now();
        let token = svc
            .issue(uuid::Uuid::from_u128(9), "DEV-01", "win", now)
            .expect("签发");
        let payload = svc.verify(&token, now).expect("立即校验通过");
        assert_eq!(payload.user_id, uuid::Uuid::from_u128(9));
        assert_eq!(payload.device_id, "DEV-01");
        let expired = svc.verify(&token, now + chrono::Duration::seconds(301));
        assert_eq!(
            expired.expect_err("过期拒").code,
            PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED
        );
        assert!(svc.verify("garbage", now).is_err(), "垃圾输入拒");
        let tampered = format!("{token}x");
        assert!(svc.verify(&tampered, now).is_err(), "篡改 MAC 拒");
    }

    #[test]
    fn base32_matches_rfc_4648_vectors() {
        assert_eq!(base32_encode(b""), "");
        assert_eq!(base32_encode(b"f"), "MY");
        assert_eq!(base32_encode(b"fo"), "MZXQ");
        assert_eq!(base32_encode(b"foo"), "MZXW6");
        assert_eq!(base32_encode(b"foob"), "MZXW6YQ");
        assert_eq!(base32_encode(b"fooba"), "MZXW6YTB");
        assert_eq!(base32_encode(b"foobar"), "MZXW6YTBOI");
    }

    #[test]
    fn b64url_decode_inverts_encode() {
        let bytes: Vec<u8> = (0..48).collect();
        let encoded = base64url_encode(&bytes);
        assert_eq!(b64url_decode(&encoded).as_deref(), Some(bytes.as_slice()));
        assert!(b64url_decode("$$$").is_none());
    }

    #[test]
    fn webauthn_assertion_rejects_replay_and_foreign_origin() {
        let origins = vec!["https://ep.local".to_string()];
        let err_free = verify_webauthn_assertion(
            "00",
            5,
            5,
            "chal",
            "ep.local",
            "https://ep.local",
            &origins,
            &[],
        );
        assert!(matches!(err_free, Ok(false)), "计数不前进即拒");
        let origin_free = verify_webauthn_assertion(
            "00",
            1,
            2,
            "chal",
            "ep.local",
            "https://evil.example",
            &origins,
            &[],
        );
        assert!(matches!(origin_free, Ok(false)), "origin 不在白名单拒");
    }

    #[test]
    fn x509_challenge_payload_is_reconstructible() {
        let p = X509ChallengeVerifier::challenge_payload("alice", "n0nce");
        assert_eq!(p, b"alice:n0nce");
    }
}
