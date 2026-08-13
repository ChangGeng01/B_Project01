//! 会话域：不透明令牌生成、摘要、上限裁剪与滑动续期合并。
//!
//! 令牌 = 32 字节随机 → base64url（无填充，43 位）；仅 SHA-256 摘要入库，
//! 明文只在登录响应出现一次；不用 JWT（基线 5.6，04:L364）。

use chrono::{DateTime, Utc};
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::config::SessionPolicy;
use crate::types::{SessionRow, REVOKE_SESSION_LIMIT_EXCEEDED};

/// 令牌随机源字节数。
pub const TOKEN_RANDOM_BYTES: usize = 32;
/// base64url 编码后长度（无填充）。
pub const TOKEN_ENCODED_LEN: usize = 43;
/// 会话摘要字节数（SHA-256）。
pub const TOKEN_DIGEST_BYTES: usize = 32;

const B64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// base64url 编码（无填充）。32 字节输入恰输出 43 位。
pub fn base64url_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = u32::from(*chunk.get(1).unwrap_or(&0));
        let b2 = u32::from(*chunk.get(2).unwrap_or(&0));
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(char::from(B64_ALPHABET[(n >> 18 & 63) as usize]));
        out.push(char::from(B64_ALPHABET[(n >> 12 & 63) as usize]));
        if chunk.len() > 1 {
            out.push(char::from(B64_ALPHABET[(n >> 6 & 63) as usize]));
        }
        if chunk.len() > 2 {
            out.push(char::from(B64_ALPHABET[(n & 63) as usize]));
        }
    }
    out
}

/// 生成新会话令牌（明文形态，仅登录响应一次）。
pub fn new_session_token() -> String {
    let mut bytes = [0u8; TOKEN_RANDOM_BYTES];
    rand::thread_rng().fill_bytes(&mut bytes);
    let token = base64url_encode(&bytes);
    debug_assert_eq!(token.len(), TOKEN_ENCODED_LEN);
    token
}

/// 令牌 SHA-256 摘要（入库唯一形态）。
pub fn token_digest(token: &str) -> [u8; TOKEN_DIGEST_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher.finalize().into()
}

/// 登录名 SHA-256 摘要（login_attempts.login_name_hash 落库形态，
/// 防攻击者注入明文登录名）。
pub fn login_name_hash(login_name: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(login_name.as_bytes());
    hasher.finalize().into()
}

/// 新会话的到期时刻对。
pub fn expiry_pair(now: DateTime<Utc>, policy: &SessionPolicy) -> (DateTime<Utc>, DateTime<Utc>) {
    let ttl = chrono::Duration::seconds(i64::try_from(policy.ttl_seconds).unwrap_or(i64::MAX));
    let idle =
        chrono::Duration::seconds(i64::try_from(policy.idle_timeout_seconds).unwrap_or(i64::MAX));
    (now + ttl, now + idle)
}

/// 会话上限裁剪：活跃会话超过 max_per_user 时返回 issued_at 最早的一条
/// （撤销理由 SESSION_LIMIT_EXCEEDED 由调用方落审计语义）。
pub fn over_limit_victim<'a>(
    sessions: &'a [SessionRow],
    policy: &SessionPolicy,
) -> Option<&'a SessionRow> {
    if sessions.len() <= policy.max_per_user {
        return None;
    }
    sessions
        .iter()
        .filter(|s| s.revoked_at.is_none() && s.revoke_reason.is_none())
        .min_by_key(|s| s.issued_at)
}

/// 上限裁剪的撤销理由取用点（落库与审计共用同一字面量）。
pub const fn over_limit_reason() -> &'static str {
    REVOKE_SESSION_LIMIT_EXCEEDED
}

/// 读请求滑动续期的批量写合并判定：距上次续期写入超过粒度才再次写。
/// 只读请求不产生每请求一次的写事务（04:L371-L372）。
pub fn should_write_sliding_extension(
    last_written_at: DateTime<Utc>,
    now: DateTime<Utc>,
    granularity_seconds: u64,
) -> bool {
    let elapsed = now.signed_duration_since(last_written_at);
    elapsed.num_seconds() >= i64::try_from(granularity_seconds).unwrap_or(i64::MAX)
}

/// 会话活性判定：未撤销、绝对到期与空闲到期均未到。
pub fn is_session_live(row: &SessionRow, now: DateTime<Utc>) -> bool {
    row.revoked_at.is_none() && row.expires_at > now && row.idle_expires_at > now
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use ep_foundation::id::marker::{LegalEntity, UserAccount};
    use ep_foundation::id::Id;

    fn session_row(issued: DateTime<Utc>, id: u128) -> SessionRow {
        SessionRow {
            id: uuid::Uuid::from_u128(id),
            user_id: Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(1)),
            user_device_row_id: uuid::Uuid::from_u128(9),
            token_hash: vec![0; 32],
            active_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            issued_at: issued,
            expires_at: issued + Duration::hours(8),
            idle_expires_at: issued + Duration::minutes(30),
            last_seen_at: issued,
            revoked_at: None,
            revoke_reason: None,
            is_breakglass: false,
        }
    }

    #[test]
    fn token_is_43_base64url_chars_and_digest_is_32_bytes() {
        let t = new_session_token();
        assert_eq!(t.len(), TOKEN_ENCODED_LEN);
        assert!(
            t.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
            "base64url 字母表：{t}"
        );
        assert_eq!(token_digest(&t).len(), TOKEN_DIGEST_BYTES);
        assert_ne!(new_session_token(), new_session_token(), "随机不撞");
    }

    #[test]
    fn base64url_handles_padding_boundaries() {
        assert_eq!(base64url_encode(&[]), "");
        assert_eq!(base64url_encode(&[0xff]), "_w");
        assert_eq!(base64url_encode(&[0xff, 0xff]), "__8");
        assert_eq!(base64url_encode(&[0xff, 0xff, 0xff]), "____");
        assert_eq!(
            base64url_encode(&[0x00, 0x10, 0x83]),
            "ABCD",
            "三字节恰四位"
        );
        assert_eq!(
            base64url_encode(&[0x00, 0x10, 0x83, 0x00]),
            "ABCDAA",
            "四字节补位"
        );
    }

    #[test]
    fn expiry_pair_follows_policy() {
        let now = Utc::now();
        let (expires, idle) = expiry_pair(now, &SessionPolicy::default());
        assert_eq!((expires - now).num_hours(), 8);
        assert_eq!((idle - now).num_minutes(), 30);
    }

    #[test]
    fn over_limit_picks_the_earliest_unrevoked() {
        let policy = SessionPolicy::default();
        let now = Utc::now();
        let old = session_row(now - Duration::minutes(50), 1);
        let mid = session_row(now - Duration::minutes(20), 2);
        let new = session_row(now, 3);
        let extra = session_row(now + Duration::minutes(1), 4);
        assert!(over_limit_victim(&[old.clone(), mid.clone(), new.clone()], &policy).is_none());
        let rows = [mid.clone(), old.clone(), new.clone(), extra.clone()];
        let victim = over_limit_victim(&rows, &policy).expect("第 4 个会话超限");
        assert_eq!(victim.id, old.id, "issued_at 最早者被撤");
        assert_eq!(over_limit_reason(), "SESSION_LIMIT_EXCEEDED");
    }

    #[test]
    fn sliding_extension_writes_at_granularity() {
        let now = Utc::now();
        assert!(!should_write_sliding_extension(
            now - Duration::seconds(59),
            now,
            60
        ));
        assert!(should_write_sliding_extension(
            now - Duration::seconds(60),
            now,
            60
        ));
        assert!(should_write_sliding_extension(
            now - Duration::minutes(5),
            now,
            60
        ));
    }

    #[test]
    fn liveness_checks_both_expiries_and_revocation() {
        let now = Utc::now();
        let mut row = session_row(now - Duration::minutes(10), 1);
        assert!(is_session_live(&row, now));
        row.idle_expires_at = now - Duration::seconds(1);
        assert!(!is_session_live(&row, now), "空闲到期失活");
        let mut row = session_row(now - Duration::minutes(10), 1);
        row.expires_at = now - Duration::seconds(1);
        assert!(!is_session_live(&row, now), "绝对到期失活");
        let mut row = session_row(now, 1);
        row.revoked_at = Some(now);
        assert!(!is_session_live(&row, now), "撤销失活");
    }
}
