//! 守卫表达式的值域与变量取数口径。
//!
//! # 数字为什么是 `rust_decimal::Decimal` 而不是 `f64`
//!
//! 基线第 3.5 节逐字把账面金额的 Rust 类型定为
//! 「`foundation::Money`，内含 `rust_decimal::Decimal`」。守卫表达式里最常见的一句
//! 就是 `vars.amount > 10000`——它比较的正是那个金额，只能与它同源。
//!
//! 用 `f64` 会得到一条**错了不会当场报错**的判据。算例：账面金额是 `numeric(18,2)`，
//! 整数位可到 16 位。`f64` 在 `[2^46, 2^47)` 区间上的间距是 `2^-6 = 0.015625`，
//! 已经大于一分钱。十进制 `70368744177664.01` 与 `70368744177664.02`
//! 的最近 `f64` 都是 `70368744177664.015625`——**两个差一分的金额按位完全相等**。
//! 于是守卫 `vars.amount > 70368744177664.01` 在金额确为 `...664.02` 时返回 `false`，
//! 一笔确实更大的金额被判成不大于，无异常、无日志，流程照走另一条分支。
//!
//! 要命的是小额比较**不会错**：手写用例里的一万、十万、一百万一律通过，
//! 只有大额单据触发。同类教训本卷已在审计的 JCS 上吃过一次（附录辛-2）。
//!
//! # 腐化点在本 crate 的上游，因此本 crate 拒绝接触 JSON
//!
//! `variables` 在库里是 `jsonb`，PostgreSQL 用 numeric 精确存；
//! 而 `serde_json` 未开 `arbitrary_precision` 时，**在 `from_str` 返回之前**
//! 就已经把带小数点的字面量变成了 `f64`——求值器再怎么写也救不回来。
//!
//! 所以这条不写成纪律，写成类型事实：本 crate 的 `Cargo.toml` 里没有 `serde`
//! 也没有 `serde_json`，[`GuardValue`] 在类型上无法从 `serde_json::Value` 构造。
//! 适配层只能走 `variables->>'k'` 取十进制**文本**再进 [`GuardValue::number`]。
//! **但这一条没有机检承接**——`archcheck` 只判层位与环，不按 crate 比对依赖清单，
//! 一次 `cargo add serde_json` 就能把它推翻。已登记入 crate 文档的未覆盖段。

use rust_decimal::Decimal;
use std::collections::BTreeMap;
use std::str::FromStr;

/// 一个守卫表达式的值。
///
/// **没有 `List` 变体。** 集合只作为 `in` 右侧的字面量列表存在于语法树上，
/// 变量里取不出它、三个白名单函数都不返回它、六种比较遇到它是类型错——
/// 放进值域就是一个永远取不到的变体，并让每一处 `match` 多一条走不到的臂。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GuardValue {
    /// 空。**键不存在与键存在但取值为空合并成同一个值**——
    /// 变量没有 schema，本 crate 无从区分二者，硬要区分就是编一个自己不掌握的事实。
    Null,
    Bool(bool),
    Number(Decimal),
    Text(String),
}

impl GuardValue {
    /// 从十进制**文本**构造一个数。这是数进入本 crate 的唯一入口。
    ///
    /// **指数形态一律拒**，即便 `Decimal::from_str` 认得 `1e10`。
    /// 拒它不是为了严格好看：`numeric(18,2)` 的文本输出从不带指数，
    /// 一个带 `e` 的取值进到这里，本身就是**上游走过 `f64`** 的证据
    /// （`f64` 的 `to_string` 在量级够大时会给出指数形态）。
    /// 与其把它当成一个能精确解析的数收下，不如让它在这里响一声——
    /// 这是本模块唯一一处能察觉上游腐化的地方。
    pub fn number(decimal_text: &str) -> Option<Self> {
        if decimal_text.contains(['e', 'E']) {
            return None;
        }
        Decimal::from_str(decimal_text).ok().map(GuardValue::Number)
    }

    pub fn text(s: impl Into<String>) -> Self {
        GuardValue::Text(s.into())
    }

    /// 类型名。**用于错误文案——错误里只报类型，不报取值**，见 [`crate::expr`] 的纪律说明。
    pub fn type_name(&self) -> &'static str {
        match self {
            GuardValue::Null => "空",
            GuardValue::Bool(_) => "布尔",
            GuardValue::Number(_) => "数字",
            GuardValue::Text(_) => "文本",
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, GuardValue::Null)
    }
}

/// 单个文本变量的字节上限。
///
/// 取值不来自计划——计划对 `variables` 只有 `jsonb not null default '{}'` 一句，
/// 对其中单个字符串的长度一个字都没有。设它是因为 `len(vars.t)` 只计 2 步，
/// 而 `chars().count()` 要走完整个字符串，且这发生在持 `FOR UPDATE` 行锁的
/// 单步事务内（计划第 3.4.8 节），与基线「事务内禁止……长时计算」直接相关。
///
/// **它管不住变量的个数。** 见 [`VarLookup`] 的文档——那一条是靠取数形状解决的，
/// 不是靠这个常量。
pub const MAX_VAR_TEXT_BYTES: usize = 4096;

/// 变量取数口径。
///
/// # 为什么是按名取数而不是先构造一整张表
///
/// 把入参写成 `BTreeMap<String, GuardValue>` 会强迫调用方**先把整份
/// `variables` 转一遍**——求值器没告诉它哪些键会被引用。而 `variables`
/// 全卷没有大小上限：一份两万五千键、每键四千字节的 `variables`
/// 能通过本模块的每一条上限（源文本二十字节、求值两步、每个文本都不超限），
/// 而构造开销上百兆，全部发生在持锁事务内。
///
/// 按名取数还顺带消掉一处**非确定性**：先构造整张表时，一个与本守卫毫不相干的键
/// 若十进制文本解析失败，整次求值就失败，而具体是哪一个取决于遍历次序。
///
/// 调用方需要知道该取哪些键时，用 [`crate::expr::Guard::referenced_vars`]。
pub trait VarLookup {
    /// 取一个变量。**返回 `None` 与返回 `Some(GuardValue::Null)` 等价**——
    /// 见 [`GuardValue::Null`] 的文档，本 crate 不区分二者。
    fn get(&self, name: &str) -> Option<GuardValue>;
}

impl VarLookup for BTreeMap<String, GuardValue> {
    fn get(&self, name: &str) -> Option<GuardValue> {
        BTreeMap::get(self, name).cloned()
    }
}

/// 空变量集。用于只引用 `instance.state` 的守卫。
pub struct NoVars;

impl VarLookup for NoVars {
    fn get(&self, _name: &str) -> Option<GuardValue> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 本模块存在的理由，钉成一条用例：十进制文本进来是精确的。
    /// 两个差一分的大额金额必须不相等——它们的最近 `f64` 是同一个数。
    #[test]
    fn two_amounts_one_cent_apart_are_never_equal() {
        let a = GuardValue::number("70368744177664.01").expect("应能解析");
        let b = GuardValue::number("70368744177664.02").expect("应能解析");
        assert_ne!(a, b, "差一分的两笔金额不得相等；用 f64 时它们按位相同");
        let (GuardValue::Number(x), GuardValue::Number(y)) = (&a, &b) else {
            panic!("应为数字");
        };
        assert!(x < y, "更大的金额必须更大");

        // 同一算例在 f64 下的样子，钉住「为什么不能用 f64」不被当成口味问题。
        let fa = 70368744177664.01_f64;
        let fb = 70368744177664.02_f64;
        assert_eq!(fa, fb, "f64 承载下两者相等——这正是本模块要避开的");
    }

    /// 第二个算例：跨整数关口的舍入。
    #[test]
    fn a_large_amount_does_not_round_across_the_threshold() {
        // 不给 GuardValue 派生 PartialOrd——跨类型的次序没有含义，
        // 派生它等于给「文本与数字谁大」发明一个口径。要比就把数取出来比。
        let (Some(GuardValue::Number(v)), Some(GuardValue::Number(t))) = (
            GuardValue::number("9999999999999999.99"),
            GuardValue::number("10000000000000000.00"),
        ) else {
            panic!("夹具应为数字");
        };
        assert!(v < t, "9999999999999999.99 必须小于 1e16");
        assert_eq!(
            9999999999999999.99_f64, 10000000000000000.00_f64,
            "f64 承载下前者被舍到后者，`>=` 会判成成立"
        );
    }

    /// 相等按**值**不按表示：`1.00` 与 `1` 是同一个数。
    /// 写成按表示相等的话，`vars.qty in [1, 2, 3]` 在 qty 为 `1.00` 时静默漏判。
    #[test]
    fn equality_is_by_value_not_by_representation() {
        assert_eq!(
            GuardValue::number("1.00"),
            GuardValue::number("1"),
            "1.00 与 1 必须相等"
        );
        assert_eq!(
            GuardValue::number("1.000000"),
            GuardValue::number("1.0"),
            "尾随零不改变取值"
        );
    }

    /// 非十进制文本一律拒，不做「尽力而为」的解析。
    ///
    /// `1e10` 单独说一句：`Decimal::from_str` 其实认得它，且解析结果是精确的。
    /// 这里仍然拒，因为 `numeric(18,2)` 的文本输出从不带指数——
    /// 一个带 `e` 的取值进到这里就是上游走过 `f64` 的证据，
    /// 这是本模块唯一一处能察觉那件事的地方。
    #[test]
    fn non_decimal_text_is_refused() {
        for bad in ["", "abc", "1.2.3", "1e10", "1E10", "0x10", "１２３", " 1"] {
            assert!(
                GuardValue::number(bad).is_none(),
                "{bad:?} 不是十进制文本，应拒绝"
            );
        }
    }

    /// 键不存在与取值为空是同一件事——本 crate 不区分，也不假装能区分。
    #[test]
    fn missing_key_and_null_value_are_the_same() {
        let empty = BTreeMap::new();
        assert!(VarLookup::get(&empty, "x").is_none());
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), GuardValue::Null);
        assert!(VarLookup::get(&m, "x").expect("键在").is_null());
    }

    #[test]
    fn type_names_are_distinct() {
        let names = [
            GuardValue::Null.type_name(),
            GuardValue::Bool(true).type_name(),
            GuardValue::number("1").expect("应能解析").type_name(),
            GuardValue::text("a").type_name(),
        ];
        let mut sorted = names;
        sorted.sort_unstable();
        let mut deduped = sorted.to_vec();
        deduped.dedup();
        assert_eq!(deduped.len(), 4, "四个类型名必须互异，否则错误文案分不清");
    }
}
