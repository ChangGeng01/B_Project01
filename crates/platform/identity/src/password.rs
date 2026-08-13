//! 口令域：Argon2id 哈希与校验、口令策略、历史不重复。
//!
//! PHC 串存 `user_credentials.verifier`；参数可配（默认 memory 65536 KiB、
//! iterations 3、parallelism 1，单次校验目标 ≤120ms）。未知用户由登录用例
//! 调 [`PasswordService::burn_dummy_cost`] 执行同一参数的固定成本校验，
//! 使「用户不存在」与「口令错误」的响应时间同分布（04 §4.3 步 2）。

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_CREDENTIAL_INVALID, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;

use crate::config::{Argon2Params, PasswordPolicy};

/// 口令哈希与校验服务。构造时按配置参数建 Argon2id 实例，并预生成
/// 一个同参数的伪 PHC 串供未知用户的固定成本校验使用。
pub struct PasswordService {
    argon2: Argon2<'static>,
    dummy_phc: String,
}

impl PasswordService {
    /// 构造失败仅可能于参数越界（如 memory 低于 8×parallelism²），
    /// 按内部错误上抛，不带病运行。
    pub fn new(params: Argon2Params) -> Result<Self, AppError> {
        let argon2 = build_argon2(params)?;
        // 伪 PHC 串与真实口令同参数，成本分布一致；内容为随机值，
        // 不作为任何账号的凭据落库。
        let dummy_phc = hash_with(&argon2, "__ep_dummy_phc_material__")?;
        Ok(Self { argon2, dummy_phc })
    }

    /// 哈希新口令，输出 PHC 串。
    pub fn hash(&self, password: &str) -> Result<String, AppError> {
        hash_with(&self.argon2, password)
    }

    /// 校验口令。PHC 串非法或口令不符一律返 false，不区分二者。
    pub fn verify(&self, phc: &str, password: &str) -> bool {
        match PasswordHash::new(phc) {
            Ok(h) => self.argon2.verify_password(password.as_bytes(), &h).is_ok(),
            Err(_) => false,
        }
    }

    /// 未知用户的固定成本伪校验：对伪 PHC 串执行一次真实参数校验，
    /// 结果弃置，仅消耗与真实校验同分布的时间。
    pub fn burn_dummy_cost(&self) {
        let _ = self.verify(&self.dummy_phc, "__ep_dummy_probe__");
    }

    /// 口令有效期：凭据创建时刻距今超过 max_age_days 即过期。
    pub fn is_expired(
        created_at: chrono::DateTime<chrono::Utc>,
        now: chrono::DateTime<chrono::Utc>,
        policy: &PasswordPolicy,
    ) -> bool {
        let age = now.signed_duration_since(created_at);
        age.num_seconds() > i64::from(policy.max_age_days) * 86_400
    }

    /// 历史不重复：新口令不得与最近 history_size 代任一 verifier 相同。
    pub fn matches_history(&self, password: &str, verifiers: &[String]) -> bool {
        verifiers.iter().any(|v| self.verify(v, password))
    }
}

fn build_argon2(params: Argon2Params) -> Result<Argon2<'static>, AppError> {
    let p = Params::new(
        params.memory_kib,
        params.iterations,
        params.parallelism,
        None,
    )
    .map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            format!("Argon2id 参数非法：{e}"),
        )
    })?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, p))
}

fn hash_with(argon2: &Argon2<'_>, password: &str) -> Result<String, AppError> {
    use argon2::password_hash::rand_core::OsRng;
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, format!("口令哈希失败：{e}")))
}

/// 口令策略校验：长度与字符类别。U-B-14 临时取值经 [`PasswordPolicy`] 注入。
pub fn check_policy(password: &str, policy: &PasswordPolicy) -> Result<(), AppError> {
    if password.len() < policy.min_length {
        return Err(AppError::new(
            PLATFORM_AUTHN_CREDENTIAL_INVALID,
            format!("口令长度不得低于 {}", policy.min_length),
        ));
    }
    if count_char_classes(password) < policy.min_char_classes {
        return Err(AppError::new(
            PLATFORM_AUTHN_CREDENTIAL_INVALID,
            format!(
                "口令字符类别不得少于 {} 类（小写/大写/数字/符号）",
                policy.min_char_classes
            ),
        ));
    }
    Ok(())
}

/// 四类字符命中计数：小写、大写、数字、其余（符号与非 ASCII）。
fn count_char_classes(password: &str) -> usize {
    let mut lower = false;
    let mut upper = false;
    let mut digit = false;
    let mut other = false;
    for c in password.chars() {
        if c.is_ascii_lowercase() {
            lower = true;
        } else if c.is_ascii_uppercase() {
            upper = true;
        } else if c.is_ascii_digit() {
            digit = true;
        } else {
            other = true;
        }
    }
    usize::from(lower) + usize::from(upper) + usize::from(digit) + usize::from(other)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试用最小参数：dev 档未优化，默认 64MiB 内存参数过重。
    fn svc() -> PasswordService {
        PasswordService::new(Argon2Params {
            memory_kib: 8,
            iterations: 1,
            parallelism: 1,
        })
        .expect("参数合法")
    }

    #[test]
    fn hash_produces_phc_argon2id_and_verify_round_trips() {
        let s = svc();
        let phc = s.hash("S3cure-pass-WORD").expect("哈希成功");
        assert!(
            phc.starts_with("$argon2id$"),
            "PHC 串算法段固定 argon2id：{phc}"
        );
        assert!(s.verify(&phc, "S3cure-pass-WORD"));
        assert!(!s.verify(&phc, "wrong"));
        assert!(!s.verify("not-a-phc", "x"), "非法 PHC 与错口令同一 false");
    }

    #[test]
    fn dummy_cost_burns_without_error() {
        let s = svc();
        s.burn_dummy_cost();
        assert!(!s.verify(&"x".repeat(64), "y"));
    }

    #[test]
    fn policy_rejects_short_and_single_class_passwords() {
        let policy = PasswordPolicy::default();
        assert!(check_policy("Ab1!Ab1!Ab1!", &policy).is_ok(), "12 位四类过");
        assert!(check_policy("Ab1!Ab1!Ab1", &policy).is_err(), "11 位拒");
        assert!(check_policy("abcdefghijkl", &policy).is_err(), "单类拒");
        assert!(check_policy("abcdefghij12", &policy).is_err(), "两类拒");
        assert!(check_policy("Abcdefghij12", &policy).is_ok(), "三类过");
        assert!(
            check_policy("口令Ab12口令Ab12", &policy).is_ok(),
            "非 ASCII 计符号类"
        );
        assert!(
            check_policy("口令口令口令口令", &policy).is_err(),
            "单一符号类不足三类"
        );
    }

    #[test]
    fn history_match_detects_reuse() {
        let s = svc();
        let old = s.hash("Old-1-pass-X").expect("哈希");
        assert!(s.matches_history("Old-1-pass-X", std::slice::from_ref(&old)));
        assert!(!s.matches_history("New-2-pass-Y", &[old]));
        assert!(!s.matches_history("anything", &[]));
    }

    #[test]
    fn password_expiry_honours_max_age_days() {
        use chrono::{Duration, Utc};
        let policy = PasswordPolicy::default();
        let now = Utc::now();
        let fresh = now - Duration::days(89);
        let stale = now - Duration::days(91);
        assert!(!PasswordService::is_expired(fresh, now, &policy));
        assert!(PasswordService::is_expired(stale, now, &policy));
    }

    #[test]
    fn invalid_argon2_params_are_rejected() {
        let err = PasswordService::new(Argon2Params {
            memory_kib: 4,
            iterations: 0,
            parallelism: 1,
        });
        assert!(err.is_err(), "iterations 为 0 非法");
    }
}
