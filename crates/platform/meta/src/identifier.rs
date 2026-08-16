//! 自定义对象码，以及由它派生出来的那一串数据库标识符的长度约束。
//!
//! 阶段 13 计划第 3.2 节逐字：对象码「**同时是 `ext` 下的物理表名**」，
//! `physical_table_name` 「固定为 `ext.` 加 code」。也就是说这个取值**直接进 DDL**，
//! 而且不止进一处——第 3.3 节的生成模板从同一个 code 派生出主键、CHECK、
//! RLS 策略、两条索引，多对多关系还会派生出关联表与它的唯一索引。
//!
//! # 两件事必须在这里挡住，都是「错了不会当场报错」的那一类
//!
//! **一、字符集。** code 未经校验就拼进 `create table ext.<code>`，
//! 一个带引号或分号的取值就是 DDL 注入，而执行它的是 `ep_migrator`——
//! 基线第 3.1 节给这个角色的权限是「迁移 DDL 与自定义对象在线 DDL」。
//!
//! **二、长度。** PostgreSQL 的 `NAMEDATALEN` 是 64，标识符上限 63 字节；
//! **超长不报错，是截断，只发一条 NOTICE**。于是 `platform_meta` 里记下的名字
//! 与库里真实存在的名字不是同一个——日后按记录的名字去 drop 或重建，
//! 命中的是空集，而调用方拿到的是「成功」。
//! 这比建表当场失败难查得多：失败会有人看见，截断没有。
//!
//! # 本模块没有覆盖的一件事，明写在这里
//!
//! 生成模板里的 `ext.<code>` 是**不加引号**的，因此一个等于 PostgreSQL 保留字的
//! code（`order`、`user`、`table` 之类）会让建表语句语法出错。
//! **本模块不判保留字。** 判它需要一份完整的保留字表，而一份不完整的黑名单
//! 正是本卷反复禁止的形态——它会让「查过了」这件事变成假的。
//! 该项已登记入裁定文件附录辛，处置有两条路（模板改为加引号，或引入完整保留字表），
//! 由那一处定，不在这里含糊过去。

/// 对象码校验失败的原因。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CodeError {
    Empty,
    /// 首字符不是小写字母。数字或下划线开头的标识符在未加引号时不合法。
    BadFirstChar {
        found: char,
    },
    /// 含不允许的字符。把违规字符点出来，否则调用方只知道「不合法」。
    IllegalChar {
        found: char,
    },
    /// 超长。带上是被哪一条派生名顶到上限的，否则「为什么是 24」没人说得清。
    TooLong {
        len: usize,
        max: usize,
        binding_derivation: &'static str,
    },
}

impl std::fmt::Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeError::Empty => f.write_str("对象码不得为空"),
            CodeError::BadFirstChar { found } => {
                write!(f, "对象码首字符 {found:?} 不合法，必须是小写字母 a 至 z")
            }
            CodeError::IllegalChar { found } => write!(
                f,
                "对象码含不允许的字符 {found:?}；只允许小写字母、数字与下划线"
            ),
            CodeError::TooLong {
                len,
                max,
                binding_derivation,
            } => write!(
                f,
                "对象码长度 {len} 超过上限 {max}；上限由派生名 {binding_derivation} 顶到，\
                 该名超过 63 字节时 PostgreSQL 会静默截断"
            ),
        }
    }
}

impl std::error::Error for CodeError {}

/// PostgreSQL 标识符的字节上限。`NAMEDATALEN` 为 64，可用 63。
pub const PG_IDENTIFIER_LIMIT: usize = 63;

/// 对象码长度上限。
///
/// **这个数是算出来的，不是拍的。** 阶段 13 计划第 3.3 节的生成模板会从一个 code
/// 派生出多条名字，取其中最长的那条反推：
///
/// | 派生名 | 长度 |
/// |---|---|
/// | `ix_<code>_legal_entity_id_created_at` | n + 30 |
/// | `ux_<code>_legal_entity_id_doc_no` | n + 26 |
/// | `ck_<code>_status` | n + 10 |
/// | `rls_<code>_le` | n + 7 |
/// | `ux_<a>_<b>_links_pair`（多对多，两个码） | n₁ + n₂ + 15 |
///
/// 单对象一路最紧的是 `ix_…created_at`，给出 n ≤ 33。
/// **但多对多那一路更紧**：两个码都取上限时 2n + 15 ≤ 63，给出 n ≤ 24。
/// 取小者 24——n=24 时多对多派生名恰好 63 字节，单对象派生名 54 字节。
///
/// 漏掉多对多那一路会得出 33，而 33 在两个自定义对象建多对多关系时
/// 派生出 81 字节的索引名，被截断到 63。**那正是本模块要挡的那种错。**
pub const MAX_OBJECT_CODE_LEN: usize = 24;

/// 顶到上限的那条派生名，随错误一起报出去。
pub const BINDING_DERIVATION: &str = "ux_<a>_<b>_links_pair";

/// 一个已校验的自定义对象码。
///
/// 只能经 [`ObjectCode::parse`] 得到——**拿不到它就拼不出建表语句**。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ObjectCode(String);

impl ObjectCode {
    pub fn parse(s: &str) -> Result<Self, CodeError> {
        let first = s.chars().next().ok_or(CodeError::Empty)?;
        if !first.is_ascii_lowercase() {
            return Err(CodeError::BadFirstChar { found: first });
        }
        if s.len() > MAX_OBJECT_CODE_LEN {
            return Err(CodeError::TooLong {
                len: s.len(),
                max: MAX_OBJECT_CODE_LEN,
                binding_derivation: BINDING_DERIVATION,
            });
        }
        if let Some(bad) = s
            .chars()
            .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_'))
        {
            return Err(CodeError::IllegalChar { found: bad });
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// `physical_table_name`。阶段 13 计划第 3.2 节逐字「固定为 `ext.` 加 code」。
    pub fn physical_table_name(&self) -> String {
        format!("{}.{}", super::custom::CUSTOM_OBJECT_SCHEMA, self.0)
    }
}

/// 多对多关联表的唯一索引名。多对多是长度上限的约束方，
/// 单列出来是为了让「上限 24 到底够不够」有一个可断言的被测对象。
pub fn link_pair_index_name(a: &ObjectCode, b: &ObjectCode) -> String {
    format!("ux_{}_{}_links_pair", a.as_str(), b.as_str())
}

/// 单对象最长的那条派生名。
pub fn created_at_index_name(code: &ObjectCode) -> String {
    format!("ix_{}_legal_entity_id_created_at", code.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(s: &str) -> ObjectCode {
        ObjectCode::parse(s).expect("夹具对象码应合法")
    }

    #[test]
    fn real_world_codes_pass() {
        assert!(ObjectCode::parse("equipment_ledger").is_ok());
        assert!(ObjectCode::parse("wo2").is_ok());
        assert_eq!(code("asset_card").physical_table_name(), "ext.asset_card");
    }

    /// 字符集白名单：code 直接进 `create table ext.<code>`，
    /// 一个带引号或分号的取值就是 DDL 注入，而执行它的是 `ep_migrator`。
    #[test]
    fn injection_shaped_codes_are_refused() {
        for bad in [
            "a;drop table finance.invoices",
            "a\"b",
            "a'b",
            "a b",
            "a-b",
            "a.b",
            "a\\b",
            "a\0b",
            "设备台账",
            "AssetCard",
        ] {
            assert!(ObjectCode::parse(bad).is_err(), "{bad:?} 必须被拒绝");
        }
    }

    #[test]
    fn first_char_must_be_a_lowercase_letter() {
        assert!(matches!(
            ObjectCode::parse("1abc"),
            Err(CodeError::BadFirstChar { .. })
        ));
        assert!(matches!(
            ObjectCode::parse("_abc"),
            Err(CodeError::BadFirstChar { .. })
        ));
        assert!(matches!(ObjectCode::parse(""), Err(CodeError::Empty)));
    }

    /// 上限恰好等于 24 应通过——上限是「不得超过」。
    #[test]
    fn exactly_at_the_length_limit_passes() {
        let at = "a".repeat(MAX_OBJECT_CODE_LEN);
        assert!(ObjectCode::parse(&at).is_ok());
        let over = "a".repeat(MAX_OBJECT_CODE_LEN + 1);
        assert!(matches!(
            ObjectCode::parse(&over),
            Err(CodeError::TooLong { .. })
        ));
    }

    /// 上限的正当性检验：两个都取满上限的码建多对多，
    /// 派生索引名必须仍在 63 字节内。
    ///
    /// **这一条是上限那个数字唯一的依据**。漏掉多对多这一路会算出 33，
    /// 而 33 在这里派生出 81 字节的名字——PostgreSQL 不报错，截断到 63，
    /// 于是 `platform_meta` 记的名字与库里的名字不是同一个，
    /// 日后按记录的名字去 drop 命中空集，调用方拿到的却是「成功」。
    #[test]
    fn two_max_length_codes_still_fit_the_link_index_name() {
        let a = code(&"a".repeat(MAX_OBJECT_CODE_LEN));
        let b = code(&"b".repeat(MAX_OBJECT_CODE_LEN));
        let name = link_pair_index_name(&a, &b);
        assert_eq!(
            name.len(),
            63,
            "两码取满时应恰好顶到 63，实为 {}",
            name.len()
        );
        assert!(name.len() <= PG_IDENTIFIER_LIMIT);
    }

    /// 上限不能定得更松：25 就会溢出。
    #[test]
    fn the_limit_is_tight_not_arbitrary() {
        let over = 2 * (MAX_OBJECT_CODE_LEN + 1) + "ux__links_pair".len() + 1;
        assert!(
            over > PG_IDENTIFIER_LIMIT,
            "若上限放宽一位即溢出，说明 24 是紧的；实算 {over}"
        );
    }

    /// 单对象一路在上限下留有余量——顺带证明约束方是多对多而不是它。
    #[test]
    fn single_object_derivations_are_not_the_binding_constraint() {
        let c = code(&"a".repeat(MAX_OBJECT_CODE_LEN));
        let n = created_at_index_name(&c);
        assert_eq!(n.len(), 54);
        assert!(n.len() < PG_IDENTIFIER_LIMIT);
    }

    /// 超长错误要说清是被哪条派生名顶到的，否则「为什么是 24」没人说得清。
    #[test]
    fn too_long_names_the_binding_derivation() {
        let msg = ObjectCode::parse(&"a".repeat(99))
            .expect_err("应拒绝")
            .to_string();
        assert!(msg.contains(BINDING_DERIVATION), "实为 {msg}");
        assert!(msg.contains("静默截断"), "实为 {msg}");
    }
}
