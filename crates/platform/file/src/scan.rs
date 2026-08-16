//! 恶意内容检查结论的汇总。
//!
//! 计划第 3.4.7 节那张表逐字：
//! `SCANNING → COMMITTED` 的触发是「**全部检查器返回 PASS 或 SKIPPED**」，
//! `SCANNING → REJECTED` 的触发是「**任一检查器返回 REJECT**」。
//!
//! # 一处必须点破的地方：这里的 `SKIPPED` 是当成通过的
//!
//! 本卷在别处反复立的纪律是「**未覆盖不等于通过**」——门禁读不到被测对象时
//! 要报未覆盖而不是判过。**这一处是全卷少见的反例，而且它是规格自己定的**：
//! 检查器不可用时返回 `SKIPPED`，上传照常放行。
//!
//! 这不是疏漏，是一个权衡：把杀毒服务的可用性变成上传功能的前置条件，
//! 会让一次杀毒服务重启把全公司的附件上传打停。规格选了另一侧。
//!
//! 但「当成通过」不等于「当作没发生」。本模块因此**把跳过的检查器名字带出去**
//! （[`ScanOutcome::skipped`]），让调用方能把它写进附件版本记录与审计——
//! 一份从未被扫过的附件与一份扫过并通过的附件，在证据上不是同一回事。
//! **这一条是本实现自加的，计划没写；不加的话「跳过」在库里不留痕，
//! 日后追查一份带毒附件时无从知道当时到底扫没扫。**

/// 单个检查器的结论。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CheckVerdict {
    Pass,
    /// 检查器不可用或不适用于该类型。
    Skipped,
    Reject,
}

impl CheckVerdict {
    pub fn as_db_value(self) -> &'static str {
        match self {
            CheckVerdict::Pass => "PASS",
            CheckVerdict::Skipped => "SKIPPED",
            CheckVerdict::Reject => "REJECT",
        }
    }
}

/// 汇总结论。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScanOutcome {
    /// 是否放行。
    pub admitted: bool,
    /// 被拒时是哪几个检查器拒的。**列全而不是只报第一个**：
    /// 一份文件同时触发两个检查器时，只报一个会让人以为改掉那一处就能过。
    pub rejected_by: Vec<String>,
    /// 跳过了哪几个检查器。放行时也要带出去，见模块文档。
    pub skipped: Vec<String>,
}

/// 汇总一组检查器的结论。
///
/// 入参是 `(检查器名, 结论)`。**空集合判为不放行**——
/// 一个检查器都没跑不等于都通过，那是「没有检查器被注册」这种配置错误，
/// 放行它等于让一次配置失误静默关掉整条恶意内容检查。
/// 这一点与模块文档里说的 `SKIPPED` 不同：`SKIPPED` 是检查器跑了并明确表态跳过，
/// 空集合是根本没有检查器——前者有人负责，后者没有。
pub fn summarize(verdicts: &[(String, CheckVerdict)]) -> ScanOutcome {
    let rejected_by: Vec<String> = verdicts
        .iter()
        .filter(|(_, v)| *v == CheckVerdict::Reject)
        .map(|(n, _)| n.clone())
        .collect();
    let skipped: Vec<String> = verdicts
        .iter()
        .filter(|(_, v)| *v == CheckVerdict::Skipped)
        .map(|(n, _)| n.clone())
        .collect();
    ScanOutcome {
        admitted: !verdicts.is_empty() && rejected_by.is_empty(),
        rejected_by,
        skipped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(pairs: &[(&str, CheckVerdict)]) -> Vec<(String, CheckVerdict)> {
        pairs.iter().map(|(n, x)| ((*n).to_string(), *x)).collect()
    }

    #[test]
    fn all_pass_admits() {
        let o = summarize(&v(&[
            ("clamd", CheckVerdict::Pass),
            ("type", CheckVerdict::Pass),
        ]));
        assert!(o.admitted);
        assert!(o.rejected_by.is_empty());
        assert!(o.skipped.is_empty());
    }

    /// SKIPPED 当成通过——这是规格自己的取舍，见模块文档。
    /// 但跳过的检查器名字必须被带出去，否则「跳过」在库里不留痕。
    #[test]
    fn skipped_admits_but_is_never_silent() {
        let o = summarize(&v(&[
            ("clamd", CheckVerdict::Skipped),
            ("type", CheckVerdict::Pass),
        ]));
        assert!(o.admitted, "SKIPPED 按规格当成通过");
        assert_eq!(
            o.skipped,
            vec!["clamd".to_string()],
            "跳过了谁必须带出去，供写入版本记录与审计"
        );
    }

    #[test]
    fn any_reject_refuses() {
        let o = summarize(&v(&[
            ("clamd", CheckVerdict::Reject),
            ("type", CheckVerdict::Pass),
        ]));
        assert!(!o.admitted);
        assert_eq!(o.rejected_by, vec!["clamd".to_string()]);
    }

    /// 多个检查器同时拒时要列全，不能只报第一个——
    /// 只报一个会让人以为改掉那一处就能过。
    #[test]
    fn all_rejecting_checkers_are_listed() {
        let o = summarize(&v(&[
            ("clamd", CheckVerdict::Reject),
            ("type", CheckVerdict::Reject),
            ("size", CheckVerdict::Pass),
        ]));
        assert_eq!(o.rejected_by.len(), 2);
    }

    /// 空集合不放行。一个检查器都没跑 ≠ 都通过——那是配置错误，
    /// 放行它等于让一次配置失误静默关掉整条恶意内容检查。
    /// 这与 SKIPPED 不同：SKIPPED 是跑了并明确表态，空集合是根本没有检查器。
    #[test]
    fn no_checker_at_all_is_not_an_implicit_pass() {
        let o = summarize(&[]);
        assert!(!o.admitted, "空检查器集合必须判不放行");
    }

    #[test]
    fn verdict_db_values() {
        assert_eq!(CheckVerdict::Pass.as_db_value(), "PASS");
        assert_eq!(CheckVerdict::Skipped.as_db_value(), "SKIPPED");
        assert_eq!(CheckVerdict::Reject.as_db_value(), "REJECT");
    }
}
