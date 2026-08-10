//! 机密的两个承载类型。
//!
//! 配置里只写引用，不写机密本身；解引用的结果包在 [`SecretString`] 里。
//! [`SecretString`] 故意不实现 Debug 与 Display——实现了就一定会有人
//! 在排障时把它打进日志，而日志是可轮转可外发的。

use std::fmt;

use serde::Deserialize;

/// `secret://<domain>/<name>#<version>` 形态的机密引用。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SecretRef(String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SecretRefError(String);

impl fmt::Display for SecretRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "机密引用格式非法：{}", self.0)
    }
}

impl std::error::Error for SecretRefError {}

impl SecretRef {
    pub fn parse(raw: &str) -> Result<SecretRef, SecretRefError> {
        let Some(rest) = raw.strip_prefix("secret://") else {
            return Err(SecretRefError(format!("{raw} 缺 secret:// 前缀"))); // 明文口令写进配置的唯一防线
        };
        let Some((path, version)) = rest.split_once('#') else {
            return Err(SecretRefError(format!("{raw} 缺 # 版本段")));
        };
        if path.is_empty() || version.is_empty() || !path.contains('/') {
            return Err(SecretRefError(format!("{raw} 的域、名或版本为空")));
        }
        Ok(SecretRef(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SecretRef {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        SecretRef::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 解引用之后的机密取值。不实现 Debug、Display 与 Serialize。
#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 唯一的取出口。调用点应尽量靠近使用处，不做长期持有。
    pub fn expose(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_formed_reference_is_accepted() {
        assert_eq!(SecretRef::parse("secret://db/app_rw#1").unwrap().as_str(), "secret://db/app_rw#1");
    }

    // 负样例断言的是「配置里不得出现明文机密」这条规则本身：
    // 任何不带 secret:// 的取值都不得被接受为机密引用。
    #[test]
    fn plaintext_password_is_rejected() {
        assert!(SecretRef::parse("hunter2").is_err());
        assert!(SecretRef::parse("secret://db/app_rw").is_err(), "缺版本段");
        assert!(SecretRef::parse("secret://app_rw#1").is_err(), "缺域");
        assert!(SecretRef::parse("secret://db/app_rw#").is_err(), "版本为空");
    }
}
