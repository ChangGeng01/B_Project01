//! 编号的值类型与格式化。这一层不碰数据库，全部可判定。

use std::fmt;

use ep_foundation::error::codes::PLATFORM_SEQUENCE_TYPE_CODE_NOT_REGISTERED;
use ep_foundation::error::ErrorCode;

/// 档案类的 `period_key` 固定取值。档案编码不按月分段——
/// 档案是长期存在的对象，按月分段会让同一份档案的编码随建档月份漂移。
pub const ARCHIVE_PERIOD_KEY: &str = "000000";

/// 流水初始位数。基线第 11.1 节写死为 6 位，溢出后自动扩展。
pub const INITIAL_WIDTH: u8 = 6;

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceError {
    /// 类型码不在登记表内。**先登记再实现**是基线第 11.1 节的明确要求。
    TypeCodeNotRegistered { code: String },
    /// 类型码本身不合法（长度或字符集）。与「未登记」分开：
    /// 一个是格式错，一个是流程错，返回给调用方的处置不同。
    TypeCodeMalformed { code: String },
    /// 法人码不是两位数字。
    LegalEntityCodeMalformed { code: String },
    /// 期间键既不是六位数字，也不是档案类的固定取值。
    PeriodKeyMalformed { key: String },
}

impl SequenceError {
    /// 对外错误码。只有「未登记」有对应的平台错误码——
    /// 另外三种是内部构造错误，调用方给不出这样的值，出现即为编码错误。
    pub fn error_code(&self) -> Option<ErrorCode> {
        match self {
            SequenceError::TypeCodeNotRegistered { .. } => {
                Some(PLATFORM_SEQUENCE_TYPE_CODE_NOT_REGISTERED)
            }
            _ => None,
        }
    }
}

impl fmt::Display for SequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SequenceError::TypeCodeNotRegistered { code } => {
                write!(f, "类型码 {code} 未在登记表中登记")
            }
            SequenceError::TypeCodeMalformed { code } => {
                write!(f, "类型码 {code} 不合法：须为 2 至 4 位大写字母")
            }
            SequenceError::LegalEntityCodeMalformed { code } => {
                write!(f, "法人码 {code} 不合法：须为 2 位数字")
            }
            SequenceError::PeriodKeyMalformed { key } => {
                write!(f, "期间键 {key} 不合法：须为 6 位数字")
            }
        }
    }
}

impl std::error::Error for SequenceError {}

/// 类型码。2 至 4 位大写字母，全局唯一，登记在 `docs/data-dictionary.md` 第 5 节。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypeCode(String);

impl TypeCode {
    /// 从字符序列构造。**本 crate 内一律用它而不是字符串字面量**，
    /// 理由见 crate 文档：字面量会被 `configdoc` 当成已登记的类型码。
    pub fn from_chars<I: IntoIterator<Item = char>>(chars: I) -> Result<Self, SequenceError> {
        Self::parse(&chars.into_iter().collect::<String>())
    }

    pub fn parse(s: &str) -> Result<Self, SequenceError> {
        let ok = (2..=4).contains(&s.len()) && s.chars().all(|c| c.is_ascii_uppercase());
        if ok {
            Ok(Self(s.to_string()))
        } else {
            Err(SequenceError::TypeCodeMalformed {
                code: s.to_string(),
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 法人码。两位数字，取自 `ep-platform-tenancy` 的法人登记。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct LegalEntityCode(String);

impl LegalEntityCode {
    pub fn parse(s: &str) -> Result<Self, SequenceError> {
        if s.len() == 2 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(s.to_string()))
        } else {
            Err(SequenceError::LegalEntityCodeMalformed {
                code: s.to_string(),
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LegalEntityCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 期间键。单据类取 `YYYYMM`，档案类固定 [`ARCHIVE_PERIOD_KEY`]。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct PeriodKey(String);

impl PeriodKey {
    /// 由日期文本取前六位。入参形如 `2026-08-17` 或 `20260817`——
    /// 两种都认，因为调用方可能从数据库直接取 `(date)::text`，也可能自己拼。
    pub fn from_date_text(s: &str) -> Result<Self, SequenceError> {
        let digits: String = s.chars().filter(char::is_ascii_digit).collect();
        if digits.len() < 6 {
            return Err(SequenceError::PeriodKeyMalformed { key: s.to_string() });
        }
        Self::parse(&digits[..6])
    }

    /// 档案类的固定期间键。
    pub fn archive() -> Self {
        Self(ARCHIVE_PERIOD_KEY.to_string())
    }

    pub fn parse(s: &str) -> Result<Self, SequenceError> {
        if s.len() == 6 && s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(s.to_string()))
        } else {
            Err(SequenceError::PeriodKeyMalformed { key: s.to_string() })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PeriodKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 一个已取到的编号。`width` 随行保存，因为溢出扩展之后，
/// 同一序列的历史编号与新编号位数不同——格式化必须用取号那一刻的位数，
/// 不能用当前位数去重排历史编号。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DocumentNumber {
    pub type_code: TypeCode,
    pub legal_entity_code: LegalEntityCode,
    pub period_key: PeriodKey,
    pub serial: u64,
    pub width: u8,
}

impl fmt::Display for DocumentNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{:0width$}",
            self.type_code,
            self.legal_entity_code,
            self.period_key,
            self.serial,
            width = usize::from(self.width)
        )
    }
}

/// 位数扩展规则，与取号 SQL 的 `case` 分支逐字同义。
///
/// SQL 侧逐字为：`width = case when next_value + 1 > (power(10, width)::bigint - 1)
/// then width + 1 else width end`。两处必须同义，否则库里存的位数与代码算的位数会分叉。
/// 这个函数存在的意义就是让那条 SQL 的语义有一个可单测的对照物。
///
/// 注意 `next_value` 是**旧值**：SQL 的 `UPDATE ... SET` 全部右侧表达式读同一行的旧值，
/// 所以判定用的是「取号后的新值会不会超出当前位数能表达的最大值」。
pub fn next_width(current_next_value: u64, current_width: u8) -> u8 {
    let max_for_width = 10u64
        .checked_pow(u32::from(current_width))
        .map_or(u64::MAX, |p| p - 1);
    if current_next_value + 1 > max_for_width {
        current_width + 1
    } else {
        current_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 造一个类型码。**不用字面量**——见 crate 文档那条门禁约束。
    fn tc(chars: &[char]) -> TypeCode {
        TypeCode::from_chars(chars.iter().copied()).expect("夹具类型码应合法")
    }

    #[test]
    fn display_matches_the_frozen_format() {
        let n = DocumentNumber {
            type_code: tc(&['S', 'O']),
            legal_entity_code: LegalEntityCode::parse("01").expect("法人码"),
            period_key: PeriodKey::parse("202608").expect("期间键"),
            serial: 123,
            width: 6,
        };
        // 基线第 11.1 节逐字给的例子。
        let want: String = ['S', 'O'].iter().collect::<String>() + "-01-202608-000123";
        assert_eq!(n.to_string(), want);
    }

    #[test]
    fn width_expands_exactly_at_overflow() {
        // 6 位能表达到 999999。取到 999998 时下一个是 999999，仍是 6 位。
        assert_eq!(next_width(999_998, 6), 6);
        // 取到 999999 时下一个是 1000000，超出 6 位，扩到 7 位。
        assert_eq!(next_width(999_999, 6), 7);
        // 扩展后不再反复扩展。
        assert_eq!(next_width(1_000_000, 7), 7);
    }

    /// 扩展之后，历史编号仍按其取号时的位数格式化，不被重排。
    #[test]
    fn history_numbers_keep_their_own_width() {
        let old = DocumentNumber {
            type_code: tc(&['P', 'O']),
            legal_entity_code: LegalEntityCode::parse("02").expect("法人码"),
            period_key: PeriodKey::parse("202608").expect("期间键"),
            serial: 1,
            width: 6,
        };
        let new = DocumentNumber {
            width: 7,
            serial: 1_000_000,
            ..old.clone()
        };
        assert!(old.to_string().ends_with("-000001"));
        assert!(new.to_string().ends_with("-1000000"));
    }

    #[test]
    fn negative_type_code_shapes() {
        // 一位、五位、含数字、含小写，全部拒绝。
        for bad in [
            vec!['A'],
            vec!['A', 'B', 'C', 'D', 'E'],
            vec!['A', '1'],
            vec!['a', 'b'],
        ] {
            assert!(
                TypeCode::from_chars(bad.clone()).is_err(),
                "{bad:?} 不该被接受"
            );
        }
        assert!(TypeCode::from_chars(['A', 'B']).is_ok());
        assert!(TypeCode::from_chars(['A', 'B', 'C', 'D']).is_ok());
    }

    #[test]
    fn negative_legal_entity_and_period() {
        assert!(LegalEntityCode::parse("1").is_err());
        assert!(LegalEntityCode::parse("001").is_err());
        assert!(LegalEntityCode::parse("0a").is_err());
        assert!(PeriodKey::parse("20268").is_err());
        assert!(PeriodKey::parse("2026081").is_err());
        assert_eq!(PeriodKey::archive().as_str(), ARCHIVE_PERIOD_KEY);
    }

    #[test]
    fn period_key_from_two_date_shapes() {
        assert_eq!(
            PeriodKey::from_date_text("2026-08-17")
                .expect("带横线")
                .as_str(),
            "202608"
        );
        assert_eq!(
            PeriodKey::from_date_text("20260817")
                .expect("不带横线")
                .as_str(),
            "202608"
        );
        assert!(PeriodKey::from_date_text("2026").is_err());
    }
}
