//! TOTP 自实现（RustCrypto 最小面：hmac + sha1，不引 totp-rs）。
//!
//! RFC 6238：HMAC-SHA1、30 秒步长、6 位数字；skew ±1 个窗口。
//! 种子经 KmsBackend wrap/unwrap 落库，`secret_ref` 形态
//! `secret://kms/totp/<user_id>#<ver>`（04:L106-L110）。

use hmac::{Hmac, Mac};
use sha1::Sha1;

/// 步长固定 30 秒（RFC 6238 默认，计划未开配置面）。
pub const TOTP_STEP_SECS: u64 = 30;
/// 输出位数固定 6 位。
pub const TOTP_DIGITS: u32 = 6;

type HmacSha1 = Hmac<Sha1>;

/// 指定计数器值的 TOTP 码（计数器 = unix 秒 / 步长）。密钥形态非法返 None。
pub fn totp_at_step(secret: &[u8], counter: u64) -> Option<String> {
    let mut mac = HmacSha1::new_from_slice(secret).ok()?;
    mac.update(&counter.to_be_bytes());
    let bytes = mac.finalize().into_bytes();
    let offset = (bytes[19] & 0x0f) as usize;
    let mut code = u32::from(bytes[offset] & 0x7f) << 24
        | u32::from(bytes[offset + 1]) << 16
        | u32::from(bytes[offset + 2]) << 8
        | u32::from(bytes[offset + 3]);
    code %= 10u32.pow(TOTP_DIGITS);
    Some(format!("{code:0width$}", width = TOTP_DIGITS as usize))
}

/// 指定时刻的 TOTP 码。
pub fn totp_code(secret: &[u8], unix_secs: u64) -> Option<String> {
    totp_at_step(secret, unix_secs / TOTP_STEP_SECS)
}

/// 校验用户提交的码：当前窗口 ± skew_steps 内任一命中即通过。
/// 常量时间比较避免码值时序侧信道。
pub fn verify_totp(secret: &[u8], code: &str, unix_secs: u64, skew_steps: u32) -> bool {
    if code.len() != TOTP_DIGITS as usize || !code.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let current = unix_secs / TOTP_STEP_SECS;
    for i in 0..=u64::from(skew_steps) {
        for counter in [current + i, current.saturating_sub(i)] {
            let Some(want) = totp_at_step(secret, counter) else {
                return false;
            };
            if constant_time_eq(code.as_bytes(), want.as_bytes()) {
                return true;
            }
        }
    }
    false
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

/// TOTP 种子的 KMS 机密引用：`secret://kms/totp/<user_id>#<ver>`。
pub fn totp_secret_ref(user_id: uuid::Uuid, version: u32) -> String {
    format!("secret://kms/totp/{user_id}#{version}")
}

/// 解析 TOTP 机密引用，返回 (user_id, version)。
pub fn parse_totp_secret_ref(reference: &str) -> Option<(uuid::Uuid, u32)> {
    let rest = reference.strip_prefix("secret://kms/totp/")?;
    let (id, ver) = rest.split_once('#')?;
    Some((uuid::Uuid::parse_str(id).ok()?, ver.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 6238 附录 B 的 SHA-1 测试密钥（"12345678901234567890"）。
    const RFC_SECRET: &[u8] = b"12345678901234567890";

    #[test]
    fn rfc_vectors_six_digits() {
        // 向量出处 RFC 6238 附录 B（SHA-1 行），取 8 位值的后 6 位。
        assert_eq!(totp_code(RFC_SECRET, 59).as_deref(), Some("287082"));
        assert_eq!(
            totp_code(RFC_SECRET, 1_111_111_109).as_deref(),
            Some("081804")
        );
        assert_eq!(
            totp_code(RFC_SECRET, 1_234_567_890).as_deref(),
            Some("005924")
        );
        assert_eq!(
            totp_code(RFC_SECRET, 2_000_000_000).as_deref(),
            Some("279037")
        );
    }

    #[test]
    fn skew_of_one_step_accepts_neighbour_windows() {
        let now = 1_700_000_100;
        let current = totp_code(RFC_SECRET, now).expect("合法密钥");
        let prev = totp_code(RFC_SECRET, now - TOTP_STEP_SECS).expect("合法密钥");
        let two_ago = totp_code(RFC_SECRET, now - 2 * TOTP_STEP_SECS).expect("合法密钥");
        assert!(verify_totp(RFC_SECRET, &current, now, 1));
        assert!(verify_totp(RFC_SECRET, &prev, now, 1), "前一窗口命中");
        assert!(
            !verify_totp(RFC_SECRET, &two_ago, now, 1),
            "超出 ±1 窗口拒（两窗口前码与当前窗口的撞码概率忽略）"
        );
        assert!(verify_totp(RFC_SECRET, &two_ago, now, 2), "skew 2 放行");
    }

    #[test]
    fn malformed_codes_are_rejected_before_crypto() {
        assert!(!verify_totp(RFC_SECRET, "12345", 59, 1), "位数不足拒");
        assert!(!verify_totp(RFC_SECRET, "1234567", 59, 1), "位数超拒");
        assert!(!verify_totp(RFC_SECRET, "12a456", 59, 1), "非数字拒");
        assert!(!verify_totp(RFC_SECRET, "", 59, 1));
    }

    #[test]
    fn secret_ref_round_trips_and_rejects_foreign_shapes() {
        let id = uuid::Uuid::from_u128(0x42);
        let r = totp_secret_ref(id, 3);
        assert_eq!(
            r,
            "secret://kms/totp/00000000-0000-0000-0000-000000000042#3"
        );
        assert_eq!(parse_totp_secret_ref(&r), Some((id, 3)));
        assert_eq!(parse_totp_secret_ref("secret://kms/totp/zz#1"), None);
        assert_eq!(parse_totp_secret_ref("secret://pki/client_ca#1"), None);
        assert_eq!(parse_totp_secret_ref("secret://kms/totp/abc"), None);
    }
}
