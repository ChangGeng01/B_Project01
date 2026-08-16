//! `ReconRegistry` —— 十五个校验项的注册表。
//!
//! # A-06 那段是签名速记，不是可编译代码
//!
//! 裁定给的是：
//!
//! ```text
//! pub struct ReconRegistry;
//! impl ReconRegistry { pub fn register(&mut self, check: std::sync::Arc<dyn ReconCheck>); }
//! ```
//!
//! 两处独立地证明它是草图：一是**单元结构体零字段，收下的 `Arc` 无处可存**，
//! 函数体只能把参数丢掉——那是一个永远成功的登记动作，正是本卷在清的恒真判据；
//! 二是**固有 `impl` 里写无函数体的方法声明本身不是合法 Rust**，
//! 只有 trait 定义允许省略函数体。
//!
//! 因此本模块只采信到「有一个吃 `Arc<dyn ReconCheck>` 的 `register(&mut self, …)`」
//! 这一层，存法与返回类型由本轮定，形状取自仓内两个同族先例各一半：
//! 存法与查重次序照 `SelfCheckRegistry`（先查后插），
//! 返回 `Result` 照 `ConfigItemApplierRegistry`。
//!
//! # 本模块**强制不了**的两件事，明写在这里
//!
//! 一、**十五项的名册。** 十五个实现体分属阶段 7、8、9b、11，本 crate 内一个都没有；
//! 卷内只找得到其中九个的具名码，另外六个只有中文名。所以本模块只提供
//! [`ReconRegistry::is_complete_roster`] 这个**按项数**的谓词，
//! 不写 `assert_eq!(registry.len(), 15)` 之类的假门禁——真名册断言的落点在
//! job-worker 的 wiring。
//!
//! 二、**`category` 与 `code` 的一致性。** 见 [`crate::ReconCheck::category`]。

use crate::check::ReconCheck;
use std::sync::Arc;

/// A-06 冻结的注册项总数：阶段 7 六个、阶段 8 两个、阶段 9b 四个、阶段 11 三个。
///
/// 这个数是**独立于注册表本身**的一个来源，正因如此它才有用：
/// 注册表只装得下已经注册进来的东西，拿它自己的内容当期望值，
/// 期望与实际必然相等、差集恒空——那样的覆盖面判定是恒真的。
pub const EXPECTED_REGISTERED_CHECK_COUNT: usize = 15;

/// 注册失败的原因。
///
/// 不走 `AppError`：错误码表冻结在 `PLATFORM` 段
/// （`code_shape_is_three_segment_upper` 逐字断言首段等于 `PLATFORM`），
/// 而对账的错误码按阶段 9 计划归 `LEDGER.` 段、本轮登记不了。
/// 照仓内二十余处的做法手写 `Display` 与 `Error`。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RegisterCheckError {
    /// `code()` 返回空串。空码进了注册表，未完成清单里就会出现一个报不出名字的项，
    /// 运维看到的是「有 1 个检查项未跑到底：」——后面什么都没有。
    EmptyCode,
    /// 同一 code 重复注册。
    DuplicateCode(&'static str),
}

impl std::fmt::Display for RegisterCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterCheckError::EmptyCode => f.write_str("对账校验项的 code 不得为空串"),
            RegisterCheckError::DuplicateCode(c) => {
                write!(f, "对账校验项 {c} 已注册过，重复注册是装配错误")
            }
        }
    }
}

impl std::error::Error for RegisterCheckError {}

/// 对账校验项注册表。
///
/// **注册顺序即执行顺序**，因此用保序的 `Vec`：执行器按此顺序分发、
/// 未完成清单也按此顺序累积，顺序稳定才可复现。
/// `HashMap` 给的是随机序，`BTreeMap` 给的是字典序——后者等于把执行次序
/// 悄悄换成了按 code 排，而两套命名风格并存时那个次序没有任何业务含义。
#[derive(Clone, Default)]
pub struct ReconRegistry {
    checks: Vec<Arc<dyn ReconCheck>>,
}

impl ReconRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 按 check 自报的 `code()` 注册。
    ///
    /// **先查后插。** 不用「先插入再看有没有顶掉旧值」的写法——
    /// 那种写法在返错之前旧值已经没了，调用方一旦忽略 `Err`，
    /// 就是静默把一个校验项换成了另一个。
    pub fn register(&mut self, check: Arc<dyn ReconCheck>) -> Result<(), RegisterCheckError> {
        let code = check.code();
        if code.is_empty() {
            return Err(RegisterCheckError::EmptyCode);
        }
        if let Some(existing) = self.checks.iter().find(|c| c.code() == code) {
            return Err(RegisterCheckError::DuplicateCode(existing.code()));
        }
        self.checks.push(check);
        Ok(())
    }

    /// 注册顺序的全部 code。
    pub fn codes(&self) -> Vec<&'static str> {
        self.checks.iter().map(|c| c.code()).collect()
    }

    /// 注册顺序中 `blocks_period_close()` 为真的 code——
    /// 即阶段 9 计划第 9.4.7 节「构成关账前强制校验的范围」的那些。
    pub fn blocking_codes(&self) -> Vec<&'static str> {
        self.checks
            .iter()
            .filter(|c| c.blocks_period_close())
            .map(|c| c.code())
            .collect()
    }

    pub fn lookup(&self, code: &str) -> Option<&dyn ReconCheck> {
        self.checks
            .iter()
            .find(|c| c.code() == code)
            .map(std::convert::AsRef::as_ref)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn ReconCheck>> {
        self.checks.iter()
    }

    pub fn len(&self) -> usize {
        self.checks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }

    /// 项数是否已达 A-06 冻结的十五项。
    ///
    /// **这个谓词判的是项数，不是名册。** 名册在本 crate 内没有被测对象，
    /// 见模块文档。
    pub fn is_complete_roster(&self) -> bool {
        self.len() == EXPECTED_REGISTERED_CHECK_COUNT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::BatchOutcome;
    use crate::model::{BatchWindow, ReconCategory};
    use ep_foundation::error::AppError;
    use ep_foundation::id::marker::{AccountingPeriod, LegalEntity};
    use ep_foundation::id::Id;
    use ep_foundation::port::tx::SnapshotCtx;

    struct Stub {
        code: &'static str,
        blocking: bool,
        batch_size: u32,
    }

    impl Stub {
        fn one(code: &'static str) -> Arc<dyn ReconCheck> {
            Arc::new(Stub {
                code,
                blocking: true,
                batch_size: 1,
            })
        }
        fn with(code: &'static str, blocking: bool, batch_size: u32) -> Arc<dyn ReconCheck> {
            Arc::new(Stub {
                code,
                blocking,
                batch_size,
            })
        }
    }

    #[async_trait::async_trait]
    impl ReconCheck for Stub {
        fn code(&self) -> &'static str {
            self.code
        }
        fn category(&self) -> ReconCategory {
            ReconCategory::Invariant
        }
        fn blocks_period_close(&self) -> bool {
            self.blocking
        }
        fn batch_size(&self) -> u32 {
            self.batch_size
        }
        async fn run_batch(
            &self,
            _snapshot: &dyn SnapshotCtx,
            _legal_entity_id: Id<LegalEntity>,
            _accounting_period_id: Id<AccountingPeriod>,
            _batch: BatchWindow,
        ) -> Result<BatchOutcome, AppError> {
            Ok(BatchOutcome {
                discrepancies: Vec::new(),
                has_more: false,
            })
        }
    }

    /// 重复 code 拒收，且**保留先注册的那一个**。
    ///
    /// 后半句是要害：一个「先插入再看有没有顶掉旧值」的实现同样返 `Err`，
    /// 但库里已经换成第二个了。用两个 `batch_size` 不同的 stub 把这一点钉住。
    #[test]
    fn a_duplicate_code_is_refused_and_the_first_one_survives() {
        let mut r = ReconRegistry::new();
        assert_eq!(r.register(Stub::with("R-X", true, 7)), Ok(()));
        assert_eq!(
            r.register(Stub::with("R-X", true, 11)),
            Err(RegisterCheckError::DuplicateCode("R-X"))
        );
        assert_eq!(r.len(), 1);
        assert_eq!(
            r.lookup("R-X").expect("先注册的那个应还在").batch_size(),
            7,
            "重复注册把先来的顶掉了——先插后查的写法就是这个结果"
        );
    }

    #[test]
    fn an_empty_code_is_refused() {
        let mut r = ReconRegistry::new();
        assert_eq!(
            r.register(Stub::one("")),
            Err(RegisterCheckError::EmptyCode)
        );
        assert!(r.is_empty());
    }

    /// 注册顺序即执行顺序。三个码刻意选成字典序与注册序不同：
    /// `COSTING_COST_VS_LEDGER` < `R-PROC-01` < `R-PROC-03`。
    /// `BTreeMap` 存法会给出字典序，`HashMap` 给随机序，两者都会让这条红。
    #[test]
    fn registration_order_is_preserved() {
        let mut r = ReconRegistry::new();
        for c in ["R-PROC-03", "COSTING_COST_VS_LEDGER", "R-PROC-01"] {
            r.register(Stub::one(c)).expect("应能注册");
        }
        assert_eq!(
            r.codes(),
            vec!["R-PROC-03", "COSTING_COST_VS_LEDGER", "R-PROC-01"]
        );
    }

    /// `blocking_codes` 是 `blocks_period_close` 的忠实投影，不是 `codes` 的别名。
    ///
    /// 今天十五个真实实现全部返真，所以在真实取值面上两者恰好相等——
    /// 正因如此这条断言只能用 stub 写，而且**必须写**：
    /// 没有它，一个 `blocking_codes() = codes()` 的偷懒实现永远不会被发现，
    /// 直到某天真有一个校验项返假。
    #[test]
    fn blocking_codes_is_a_faithful_projection_not_an_alias() {
        let mut r = ReconRegistry::new();
        r.register(Stub::with("A", true, 1)).expect("应能注册");
        r.register(Stub::with("B", false, 1)).expect("应能注册");
        r.register(Stub::with("C", true, 1)).expect("应能注册");
        assert_eq!(r.codes(), vec!["A", "B", "C"]);
        assert_eq!(r.blocking_codes(), vec!["A", "C"]);
    }

    /// 空注册表不是「全部放行」。
    /// 空集合的差集恒空、`contains` 恒假——本卷已在单据类型码登记表上点过这个陷阱。
    #[test]
    fn an_empty_registry_is_not_a_complete_roster() {
        let r = ReconRegistry::new();
        assert!(!r.is_complete_roster());
        assert!(r.blocking_codes().is_empty());
    }

    /// 项数谓词的两侧都要能取到，否则它是个写死的常量。
    #[test]
    fn the_roster_predicate_flips_at_exactly_fifteen() {
        let codes: [&'static str; 16] = [
            "C00", "C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09", "C10", "C11",
            "C12", "C13", "C14", "C15",
        ];
        let mut r = ReconRegistry::new();
        for c in &codes[..EXPECTED_REGISTERED_CHECK_COUNT - 1] {
            r.register(Stub::one(c)).expect("应能注册");
        }
        assert!(!r.is_complete_roster(), "差一项时不该判成齐备");
        r.register(Stub::one(codes[EXPECTED_REGISTERED_CHECK_COUNT - 1]))
            .expect("应能注册");
        assert!(r.is_complete_roster());
        // 多一项同样不齐备——A-06 冻结的是「恰十五个」，不是「至少十五个」。
        r.register(Stub::one(codes[EXPECTED_REGISTERED_CHECK_COUNT]))
            .expect("应能注册");
        assert!(!r.is_complete_roster(), "十六项不是齐备，是多了一项");
    }

    #[test]
    fn lookup_misses_return_none() {
        let mut r = ReconRegistry::new();
        r.register(Stub::one("A")).expect("应能注册");
        assert!(r.lookup("B").is_none());
    }
}
