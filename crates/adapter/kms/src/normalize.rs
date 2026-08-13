//! 盲索引归一化四取值（`sensitive_field_registry.normalization`）：
//! NONE / TRIM_NFKC / TRIM_NFKC_LOWER / DIGITS_ONLY，默认 TRIM_NFKC。
//!
//! 归一化是盲索引等值语义的一部分：HMAC 之前先归一化，保证同一业务取值
//! 的书写变体落到同一索引。只支持等值，PREFIX 不放行。

use unicode_normalization::UnicodeNormalization;

/// 归一化四取值，与登记表的字面量一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Normalization {
    /// 原样，不做任何处理。
    None,
    /// 去首尾空白后做 NFKC 规范化（登记默认值）。
    TrimNfkc,
    /// 在 TRIM_NFKC 之上统一小写。
    TrimNfkcLower,
    /// 只保留数字：先做 NFKC（全角数字归半角）再留 ASCII 数字，
    /// 适用于账号号码类取值。
    DigitsOnly,
}

impl Normalization {
    /// 登记表字面量到枚举的解析，非法取值返 `None`。
    pub fn parse(literal: &str) -> Option<Normalization> {
        match literal {
            "NONE" => Some(Normalization::None),
            "TRIM_NFKC" => Some(Normalization::TrimNfkc),
            "TRIM_NFKC_LOWER" => Some(Normalization::TrimNfkcLower),
            "DIGITS_ONLY" => Some(Normalization::DigitsOnly),
            _ => Option::None,
        }
    }

    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            Normalization::None => "NONE",
            Normalization::TrimNfkc => "TRIM_NFKC",
            Normalization::TrimNfkcLower => "TRIM_NFKC_LOWER",
            Normalization::DigitsOnly => "DIGITS_ONLY",
        }
    }
}

/// 对取值施加归一化，返回可直接进 HMAC 的字节。
pub fn normalize(value: &str, normalization: Normalization) -> Vec<u8> {
    match normalization {
        Normalization::None => value.as_bytes().to_vec(),
        Normalization::TrimNfkc => value.trim().nfkc().collect::<String>().into_bytes(),
        Normalization::TrimNfkcLower => {
            let nfkc: String = value.trim().nfkc().collect();
            nfkc.to_lowercase().into_bytes()
        }
        Normalization::DigitsOnly => {
            let nfkc: String = value.nfkc().collect();
            nfkc.bytes().filter(u8::is_ascii_digit).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_four_literals() {
        assert_eq!(Normalization::parse("NONE"), Some(Normalization::None));
        assert_eq!(
            Normalization::parse("TRIM_NFKC"),
            Some(Normalization::TrimNfkc)
        );
        assert_eq!(
            Normalization::parse("TRIM_NFKC_LOWER"),
            Some(Normalization::TrimNfkcLower)
        );
        assert_eq!(
            Normalization::parse("DIGITS_ONLY"),
            Some(Normalization::DigitsOnly)
        );
        assert_eq!(Normalization::parse("PREFIX"), Option::None);
    }

    #[test]
    fn branches_behave_per_spec() {
        let raw = " ６２２２\u{00a0}0202 ";
        assert_eq!(normalize(raw, Normalization::None), raw.as_bytes());
        // NFKC 把全角数字归为半角，不间断空格归为普通空格后被 trim 保留中段。
        let nfkc = normalize(raw, Normalization::TrimNfkc);
        assert!(nfkc.starts_with(b"6222"));
        let lower = normalize(" AbC ", Normalization::TrimNfkcLower);
        assert_eq!(lower, b"abc");
        let digits = normalize(raw, Normalization::DigitsOnly);
        assert_eq!(digits, b"62220202");
    }
}
