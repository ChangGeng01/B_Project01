//! 身份域策略参数。
//!
//! 默认值取 04 计划 §7 配置表：EP__AUTH__* 各段的启动取值；
//! U-B-14 临时取值处逐项注释标注（锁定 5/15/30 分钟、口令 12/3/90/5）。

/// 口令策略。U-B-14 临时取值：12 位、三类字符、90 天、历史 5 代。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PasswordPolicy {
    /// 最小长度（U-B-14 临时取值 12）。
    pub min_length: usize,
    /// 最少字符类别数：小写/大写/数字/符号四类中至少命中几类（U-B-14 取 3）。
    pub min_char_classes: usize,
    /// 口令最长有效天数，超期要求重置（U-B-14 取 90）。
    pub max_age_days: u32,
    /// 不得与最近 N 代历史口令重复（U-B-14 取 5）。
    pub history_size: usize,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            min_char_classes: 3,
            max_age_days: 90,
            history_size: 5,
        }
    }
}

/// Argon2id 参数。默认 memory 65536 KiB、iterations 3、parallelism 1，
/// 单次校验目标 ≤120ms；超出时下调 memory_kib 重测。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Argon2Params {
    pub memory_kib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory_kib: 65_536,
            iterations: 3,
            parallelism: 1,
        }
    }
}

/// 锁定策略。U-B-14 临时取值：窗口 15 分钟内失败 5 次，锁 30 分钟。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LockoutPolicy {
    /// 触发锁定的失败次数（U-B-14 取 5）。
    pub max_failures: u32,
    /// 失败计数窗口秒数（U-B-14 取 900 = 15 分钟）。
    pub window_seconds: u64,
    /// 锁定时长秒数（U-B-14 取 1800 = 30 分钟）。
    pub duration_seconds: u64,
}

impl Default for LockoutPolicy {
    fn default() -> Self {
        Self {
            max_failures: 5,
            window_seconds: 900,
            duration_seconds: 1_800,
        }
    }
}

/// 会话策略：TTL 8 小时、空闲 30 分钟滑动续期、单用户上限 3、
/// 读请求续期按 60 秒粒度批量写合并。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SessionPolicy {
    pub ttl_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_per_user: usize,
    pub sliding_write_granularity_seconds: u64,
}

impl Default for SessionPolicy {
    fn default() -> Self {
        Self {
            ttl_seconds: 28_800,
            idle_timeout_seconds: 1_800,
            max_per_user: 3,
            sliding_write_granularity_seconds: 60,
        }
    }
}

/// TOTP 参数：30 秒步长固定，skew 取 ±1 个窗口。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TotpPolicy {
    pub skew_steps: u32,
}

impl Default for TotpPolicy {
    fn default() -> Self {
        Self { skew_steps: 1 }
    }
}

/// WebAuthn 配置：RP_ID 与 ORIGINS 必填，缺失即自检失败。
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct WebauthnPolicy {
    pub rp_id: String,
    pub origins: Vec<String>,
}

impl WebauthnPolicy {
    /// 自检：两键任一为空即失败，理由文本供启动自检与端点 503 复用。
    pub fn selfcheck(&self) -> Result<(), String> {
        if self.rp_id.trim().is_empty() {
            return Err("EP__AUTH__WEBAUTHN__RP_ID 未配置".to_string());
        }
        if self.origins.is_empty() || self.origins.iter().any(|o| o.trim().is_empty()) {
            return Err("EP__AUTH__WEBAUTHN__ORIGINS 未配置或含空项".to_string());
        }
        Ok(())
    }
}

/// X509_CERT 第一因子的信任锚引用。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct X509Policy {
    /// 信任锚机密引用，默认 `secret://pki/client_ca#1`。
    pub trust_anchor_ref: String,
}

impl Default for X509Policy {
    fn default() -> Self {
        Self {
            trust_anchor_ref: "secret://pki/client_ca#1".to_string(),
        }
    }
}

/// 应急账号策略：单次启用 ≤8h；闲置 12 个月轮换。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BreakglassPolicy {
    pub max_session_seconds: u64,
    pub idle_rotation_days: u32,
}

impl Default for BreakglassPolicy {
    fn default() -> Self {
        Self {
            max_session_seconds: 28_800,
            idle_rotation_days: 365,
        }
    }
}

/// 身份域全量策略束。wiring 从 EP__AUTH__* 配置段映射构造。
#[derive(Clone, Debug, Default)]
pub struct IdentityPolicies {
    pub password: PasswordPolicy,
    pub argon2: Argon2Params,
    pub lockout: LockoutPolicy,
    pub session: SessionPolicy,
    pub totp: TotpPolicy,
    pub webauthn: WebauthnPolicy,
    pub x509: X509Policy,
    pub breakglass: BreakglassPolicy,
}

/// 六类高风险操作权限项编码（种子登记 V202610121120 的六行 SUBMIT 项）。
/// 强制 MFA 判据第三支：持有含任一该编码授权的角色即强制第二因子。
pub const HIGH_RISK_PERMISSION_ITEMS: [&str; 6] = [
    "platform.contract_effective",
    "platform.payment",
    "platform.invoice_issue",
    "platform.ledger_posting",
    "platform.period_close",
    "platform.sensitive_export",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_pin_the_plan_values() {
        let p = IdentityPolicies::default();
        assert_eq!(p.password.min_length, 12);
        assert_eq!(p.password.min_char_classes, 3);
        assert_eq!(p.password.max_age_days, 90);
        assert_eq!(p.password.history_size, 5);
        assert_eq!(p.argon2.memory_kib, 65_536);
        assert_eq!(p.argon2.iterations, 3);
        assert_eq!(p.argon2.parallelism, 1);
        assert_eq!(p.lockout.max_failures, 5);
        assert_eq!(p.lockout.window_seconds, 900);
        assert_eq!(p.lockout.duration_seconds, 1_800);
        assert_eq!(p.session.ttl_seconds, 28_800);
        assert_eq!(p.session.idle_timeout_seconds, 1_800);
        assert_eq!(p.session.max_per_user, 3);
        assert_eq!(p.session.sliding_write_granularity_seconds, 60);
        assert_eq!(p.totp.skew_steps, 1);
        assert_eq!(p.x509.trust_anchor_ref, "secret://pki/client_ca#1");
        assert_eq!(p.breakglass.max_session_seconds, 28_800);
        assert_eq!(p.breakglass.idle_rotation_days, 365);
    }

    #[test]
    fn webauthn_selfcheck_requires_rp_id_and_origins() {
        assert!(WebauthnPolicy::default().selfcheck().is_err());
        let half = WebauthnPolicy {
            rp_id: "ep.local".into(),
            origins: vec![],
        };
        assert!(half.selfcheck().is_err());
        let blank = WebauthnPolicy {
            rp_id: "ep.local".into(),
            origins: vec!["".into()],
        };
        assert!(blank.selfcheck().is_err(), "空项视同未配置");
        let ok = WebauthnPolicy {
            rp_id: "ep.local".into(),
            origins: vec!["https://ep.local".into()],
        };
        assert!(ok.selfcheck().is_ok());
    }

    #[test]
    fn high_risk_items_are_the_six_seed_codes() {
        assert_eq!(HIGH_RISK_PERMISSION_ITEMS.len(), 6);
        assert!(HIGH_RISK_PERMISSION_ITEMS.contains(&"platform.payment"));
        assert!(HIGH_RISK_PERMISSION_ITEMS.contains(&"platform.sensitive_export"));
    }
}
