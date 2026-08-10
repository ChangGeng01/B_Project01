//! 密级。
//!
//! 取值码与序列化形态按技术基线第 4 节公共列 `security_level smallint` 与
//! 阶段 1 计划的冻结类型表，四级取 10、20、30、40 并序列化为数字，不序列化为变体名。
//! 未知取值反序列化失败，不静默降级。

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum SecurityLevel {
    Public = 10,
    Internal = 20,
    Confidential = 30,
    Secret = 40,
}

impl SecurityLevel {
    pub const ALL: [SecurityLevel; 4] = [
        SecurityLevel::Public,
        SecurityLevel::Internal,
        SecurityLevel::Confidential,
        SecurityLevel::Secret,
    ];

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub const fn from_code(code: u8) -> Option<SecurityLevel> {
        match code {
            10 => Some(SecurityLevel::Public),
            20 => Some(SecurityLevel::Internal),
            30 => Some(SecurityLevel::Confidential),
            40 => Some(SecurityLevel::Secret),
            _ => None,
        }
    }
}

impl Serialize for SecurityLevel {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(self.code())
    }
}

impl<'de> Deserialize<'de> for SecurityLevel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let code = u8::deserialize(d)?;
        SecurityLevel::from_code(code).ok_or_else(|| {
            serde::de::Error::custom(format!("密级取值码只允许 10、20、30、40，收到 {code}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_match_the_common_column() {
        assert_eq!(SecurityLevel::ALL.map(SecurityLevel::code), [10, 20, 30, 40]);
    }

    #[test]
    fn serializes_as_number_not_variant_name() {
        let json = serde_json::to_string(&SecurityLevel::Confidential).expect("可序列化");
        assert_eq!(json, "30", "公共列是 smallint，序列化为变体名会与库内取值分叉");
    }

    #[test]
    fn unknown_code_fails_loudly() {
        assert!(serde_json::from_str::<SecurityLevel>("25").is_err(), "未知取值不得静默降级");
        assert_eq!(serde_json::from_str::<SecurityLevel>("40").expect("合法"), SecurityLevel::Secret);
    }

    #[test]
    fn ordering_follows_the_codes() {
        assert!(SecurityLevel::Public < SecurityLevel::Secret);
    }
}
