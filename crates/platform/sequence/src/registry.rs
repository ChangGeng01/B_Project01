//! 类型码登记表。**先登记再实现**——基线第 11.1 节的明确要求。
//!
//! 本表与 `docs/data-dictionary.md` 第 5 节的登记表由
//! `xtask configdoc --check-doc-type-codes` 逐项比对，两侧必须逐项一致且各自无重复。

use crate::number::{SequenceError, TypeCode};

/// 取号的作用域类别。单据类与档案类共用同一张登记表、共用同一套取号机制，
/// 区别只在期间键：单据类按月分段，档案类固定 `000000`。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeKind {
    Document,
    Archive,
}

impl ScopeKind {
    /// 数据库侧的 `scope_kind` 取值，与 `ck_number_sequences_scope_kind` 的枚举一致。
    pub fn as_db_value(self) -> &'static str {
        // 这两个取值可以直接写字面量：门禁只把 2 至 4 位大写字母当类型码，
        // 而它们是 8 位与 7 位，落不进那个形状。crate 文档那条约束针对的是
        // 类型码本身，不是所有大写字面量——不要为它把可读的常量改成转义。
        match self {
            ScopeKind::Document => "DOCUMENT",
            ScopeKind::Archive => "ARCHIVE",
        }
    }
}

/// 已登记的类型码全集。
///
/// **当前为空，且这不是遗漏。** 阶段 1 至 3a 不引入任何单据与任何档案，
/// 一个类型码都没有可登记的；`docs/data-dictionary.md` 第 5 节同样是 0 行，
/// 两侧一致。业务阶段引入第一个单据类型时，**两处必须同批增加**——
/// 只改一边会被 `configdoc` 当场判不符。
pub const REGISTERED_TYPE_CODES: [&str; 0] = [];

/// 类型码登记表。
pub struct TypeCodeRegistry {
    codes: Vec<TypeCode>,
}

impl Default for TypeCodeRegistry {
    fn default() -> Self {
        Self::from_registered()
    }
}

impl TypeCodeRegistry {
    /// 由 [`REGISTERED_TYPE_CODES`] 构造。登记表里出现不合法的取值即为编码错误，
    /// 直接 panic 而不是静默跳过——静默跳过会让一个写错的类型码变成「未登记」，
    /// 那是把编码错误伪装成业务错误。
    pub fn from_registered() -> Self {
        Self {
            codes: REGISTERED_TYPE_CODES
                .iter()
                .map(|c| TypeCode::parse(c).expect("登记表中的类型码必须合法"))
                .collect(),
        }
    }

    /// 供测试与业务阶段自建登记表用。
    pub fn new(codes: Vec<TypeCode>) -> Self {
        Self { codes }
    }

    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    /// 校验类型码已登记。未登记返回 `PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED`
    /// 对应的错误，这是取号算法的第 1 步。
    pub fn ensure_registered(&self, code: &TypeCode) -> Result<(), SequenceError> {
        if self.codes.contains(code) {
            Ok(())
        } else {
            Err(SequenceError::TypeCodeNotRegistered {
                code: code.as_str().to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tc(chars: &[char]) -> TypeCode {
        TypeCode::from_chars(chars.iter().copied()).expect("夹具类型码应合法")
    }

    /// 空登记表不是「全部放行」，是「全部拒绝」。
    /// 这条守的是一个容易写反的地方：空集合的 `contains` 恒为假，
    /// 若哪天有人把它改成「空表即不校验」，本用例会红。
    #[test]
    fn empty_registry_rejects_everything() {
        let r = TypeCodeRegistry::from_registered();
        assert!(r.is_empty());
        let err = r
            .ensure_registered(&tc(&['S', 'O']))
            .expect_err("空表必须拒绝");
        assert!(matches!(err, SequenceError::TypeCodeNotRegistered { .. }));
        assert!(err.error_code().is_some(), "该错误必须有对外错误码");
    }

    #[test]
    fn registered_code_passes_and_others_do_not() {
        let r = TypeCodeRegistry::new(vec![tc(&['S', 'O'])]);
        assert!(r.ensure_registered(&tc(&['S', 'O'])).is_ok());
        assert!(r.ensure_registered(&tc(&['P', 'O'])).is_err());
        assert_eq!(r.len(), 1);
    }

    /// 与字典侧的两处计数必须同为 0——这是本阶段的既定状态，
    /// 改动其中一边而不改另一边会被 configdoc 判不符，本用例先在代码侧提醒。
    #[test]
    fn registered_table_is_empty_by_design() {
        assert_eq!(
            REGISTERED_TYPE_CODES.len(),
            0,
            "本阶段不引入任何单据与档案；新增类型码须与 docs/data-dictionary.md 第 5 节同批"
        );
    }

    #[test]
    fn scope_kind_db_values() {
        assert_eq!(ScopeKind::Document.as_db_value(), "DOCUMENT");
        assert_eq!(ScopeKind::Archive.as_db_value(), "ARCHIVE");
    }
}
