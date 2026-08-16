//! `ReconExecutor` 契约，与一次运行的汇总语义。
//!
//! # 本 crate 不提供执行器的实现体
//!
//! `run` 要读三张表、要开事务、要逐法人遍历，按 A-06 归阶段 9a 的落库交付。
//! 这里只给签名，**不留 `todo!()`、不留占位实现**——一个会 panic 的占位
//! 比没有实现更危险，因为它编译得过、也注册得进去。
//!
//! 但汇总语义不能跟着一起推走。若不把它提炼成一个不读库的纯函数
//! （[`summarize_run`]），四个 status 的判定条件就只能是文档里的散文，
//! 没有任何被测对象——那正是本轮要避免的形态。
//!
//! # UNFINISHED 与 FAILED 的分界是本实现自定的，卷内没有出处
//!
//! 规格第 10.2 章把五类终止成因——单批执行时限触发终止、单查询内存或临时空间
//! 上限触发终止、执行进程异常退出、连接被回收、快照失效——**全部**归入「未完成」，
//! 一次关账的几种结束方式里根本没有「失败」这一种。
//! 全卷 `FAILED` 只出现在 A-06 那一行 CHECK 定义里，再无第二处给它产生条件；
//! 阶段 14 的降级 kind 里也只有 `RECON_RUN_UNFINISHED`，没有对应 `FAILED` 的一项。
//!
//! 本轮给出的分界是「**能不能归因到某个 check**」：
//! `Unfinished` = 至少有一个阻断性校验项没产生结论，且这件事落得到具体的 code 上；
//! `Failed` = 一个结论也没有、一个归因也给不出。
//! 这条线切在**输出的可用性**上而不是成因上，因此不违逆规格对五类成因的归属。
//! 它使四个取值全部可达，且两者的处置路径确实不同：前者有名单可查、可按项重跑，
//! 后者只能查运行本身。**该分界与 `FAILED` 的降级承接方一并登记为待裁定。**

use crate::model::{ReconRunKind, ReconRunOutcome, ReconRunStatus};
use crate::ReconRun;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{AccountingPeriod, LegalEntity};
use ep_foundation::id::Id;

/// 对账执行器。签名逐字取自 A-06，**不加参数**。
///
/// 逐法人遍历在 `run` 的**调用方**（job-worker 的调度），不在 `run` 内部：
/// A-06 与阶段 9 计划都说「执行器按基线第 3.8 节逐法人遍历」，
/// 而同一份裁定给的 `run` 签名是单法人单期间，两者只能这样调和。
///
/// `run_kind` 在本轮**只是留痕**，对「跑哪些 check」没有区分作用：
/// `ReconCheck` 的选择维度只有 `blocks_period_close()` 一个而它今天恒真。
/// **不要按 `run_kind` 写分支**——今天写出来的一定是恒真或不可达。
#[async_trait::async_trait]
pub trait ReconExecutor: Send + Sync {
    async fn run(
        &self,
        run_kind: ReconRunKind,
        legal_entity_id: Id<LegalEntity>,
        accounting_period_id: Id<AccountingPeriod>,
    ) -> Result<ReconRunOutcome, AppError>;
}

/// 对象安全断言。装配侧按 `Arc<dyn ReconExecutor>` 注入。编译期，夹具伪装不了。
const _: fn(std::sync::Arc<dyn ReconExecutor>) = |_| {};

/// 一次运行攒下的观测。执行器边跑边填，跑完交给 [`summarize_run`]。
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RunTally {
    /// 运行开始那一刻注册表里的阻断性校验项。**期望集**。
    pub expected_codes: Vec<String>,
    /// 跑到 `has_more == false` 的项。**产生了结论**的那些。
    pub concluded_codes: Vec<String>,
    /// 没产生结论的项——报错的、被切断的。
    pub inconclusive_codes: Vec<String>,
    /// 本次运行检出的差异条数。
    pub discrepancy_count: u32,
    /// 运行在任何一批派发出去之前就断了（建快照失败、法人或期间解析失败、
    /// 连接在第一批之前被回收）。此时「哪一项没跑」无从归因。
    pub aborted_before_any_check: bool,
}

/// 把一次运行的观测汇总成结果。
///
/// 四个分支**有序且互斥**，每一个都可达：
///
/// 一、`Failed`：期望集为空。注册表里一个阻断性校验项都没有——
///    这不是「没有差异」，是「这次运行根本判不出任何东西」。
///    **它今天就会发生**：阶段 9a 交付本体时，十五项里有十一项还不存在。
///
/// 二、`Failed`：在任何一批之前就断了，且什么结论和归因都没有。
///
/// 三、`Unfinished`：有归因得到的未完成项，或期望集没被结论集覆盖。
///    未完成清单由构造保证非空。
///
/// 四、`Completed`：期望集非空、无未完成项、且期望集⊆结论集。
///    **三条缺一不可**——只判「没有报错」就是恒真判据。
///
/// `Running` **永不由本函数产出**，见 [`ReconRunOutcome::running`]。
pub fn summarize_run(tally: &RunTally, run_id: Id<ReconRun>) -> ReconRunOutcome {
    let missing: Vec<String> = tally
        .expected_codes
        .iter()
        .filter(|c| !tally.concluded_codes.contains(c))
        .cloned()
        .collect();

    let mut unfinished: Vec<String> = tally.inconclusive_codes.clone();
    for m in missing {
        if !unfinished.contains(&m) {
            unfinished.push(m);
        }
    }

    let executed = tally.concluded_codes.clone();

    // 两条 `Failed` 的次序不能颠倒。**先判「一批都没派发出去」**：
    // 那一刻期望集通常非空，若先算未完成清单，期望集会整个落进未完成，
    // 这条分支就永远走不到——一个列在取值域里却取不到的取值，正是本卷在清的乙类。
    // 落进未完成也是误报：那十五项不是「没跑到底」，是运行本身没起来，
    // 运维要查的是快照与连接，不是逐项重跑。
    let aborted_with_nothing_to_attribute = tally.aborted_before_any_check
        && tally.concluded_codes.is_empty()
        && tally.inconclusive_codes.is_empty();
    if aborted_with_nothing_to_attribute || tally.expected_codes.is_empty() {
        return ReconRunOutcome {
            run_id,
            status: ReconRunStatus::Failed,
            discrepancy_count: tally.discrepancy_count,
            executed_check_codes: executed,
            unfinished_check_codes: Vec::new(),
        };
    }

    let status = if unfinished.is_empty() {
        ReconRunStatus::Completed
    } else {
        ReconRunStatus::Unfinished
    };

    ReconRunOutcome {
        run_id,
        status,
        discrepancy_count: tally.discrepancy_count,
        executed_check_codes: executed,
        unfinished_check_codes: unfinished,
    }
}

/// 校验一个 [`ReconRunOutcome`] 自身是否自洽。
///
/// [`summarize_run`] 由构造保证这些性质，但 `ReconRunOutcome` 的字段是 `pub`
/// （与本 crate 其余值类型一致），所以谁都能手搓一个坏值出来。
/// 最要紧的坏值是 `{ status: Completed, unfinished_check_codes: ["X"] }`——
/// 关账闸门只在 `Unfinished` 那一臂读未完成清单，于是这条证据被静默丢掉、
/// 期间照常关掉。这个函数是那一处的第二道锁。
pub fn validate_run_outcome(o: &ReconRunOutcome) -> Result<(), &'static str> {
    if o.status == ReconRunStatus::Unfinished && o.unfinished_check_codes.is_empty() {
        return Err("UNFINISHED 必须列出没跑到底的检查项，否则运维无从下手");
    }
    if o.status != ReconRunStatus::Unfinished && !o.unfinished_check_codes.is_empty() {
        return Err("只有 UNFINISHED 才带未完成清单；别的状态带着它意味着证据会被闸门丢掉");
    }
    if o.status == ReconRunStatus::Completed && o.executed_check_codes.is_empty() {
        return Err("COMPLETED 必须有跑过的检查项；一项都没跑过不是「通过」");
    }
    if o.status == ReconRunStatus::Running
        && !(o.executed_check_codes.is_empty()
            && o.unfinished_check_codes.is_empty()
            && o.discrepancy_count == 0)
    {
        return Err("RUNNING 是运行开始那一刻的状态，三项统计都该是空的");
    }
    if let Some(dup) = o
        .executed_check_codes
        .iter()
        .find(|c| o.unfinished_check_codes.contains(c))
    {
        let _ = dup;
        return Err("同一个检查项不能既跑完了又没跑到底");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rid() -> Id<ReconRun> {
        Id::from_uuid(uuid::Uuid::from_u128(1))
    }

    fn v(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_string()).collect()
    }

    /// 四个 status 各要有一组观测能取到它——**除了 `Running`**，
    /// 它按设计不由本函数产出。一个取不到的取值是本卷在清的乙类缺陷。
    #[test]
    fn every_status_summarize_can_produce_is_reachable() {
        // Failed 分支一：注册表空。
        let empty = RunTally::default();
        assert_eq!(summarize_run(&empty, rid()).status, ReconRunStatus::Failed);

        // Failed 分支二：期望集非空，但一批都没派发出去就断了。
        // 这一条**必须与分支一分开测**：若两条分支的判定次序写反，
        // 期望集会整个落进未完成清单，本分支就永远走不到。
        let aborted = RunTally {
            expected_codes: v(&["A"]),
            aborted_before_any_check: true,
            ..RunTally::default()
        };
        let o = summarize_run(&aborted, rid());
        assert_eq!(
            o.status,
            ReconRunStatus::Failed,
            "运行本身没起来，不该报成「这些项没跑到底」"
        );
        assert!(
            o.unfinished_check_codes.is_empty(),
            "无从归因时不得把期望集整个当成未完成项"
        );

        let unfinished = RunTally {
            expected_codes: v(&["A", "B"]),
            concluded_codes: v(&["A"]),
            ..RunTally::default()
        };
        assert_eq!(
            summarize_run(&unfinished, rid()).status,
            ReconRunStatus::Unfinished
        );

        let completed = RunTally {
            expected_codes: v(&["A", "B"]),
            concluded_codes: v(&["A", "B"]),
            ..RunTally::default()
        };
        assert_eq!(
            summarize_run(&completed, rid()).status,
            ReconRunStatus::Completed
        );
    }

    /// 本模块最要紧的一条：**注册表为空时不得判成通过**。
    ///
    /// 这不是假想的边界——阶段 9a 交付本体那一刻，十五项里有十一项还不存在，
    /// 一次 PERIOD_CLOSE 运行会跑完、无差异。若判成 `Completed`，
    /// 关账闸门放行，那个期间就被永久关掉了，而十一项校验从没跑过。
    #[test]
    fn an_empty_expectation_set_is_never_a_pass() {
        let o = summarize_run(&RunTally::default(), rid());
        assert_eq!(o.status, ReconRunStatus::Failed);
        assert!(o.executed_check_codes.is_empty());
        assert_ne!(
            o.status,
            ReconRunStatus::Completed,
            "「一项都没跑」不是「跑完且无差异」"
        );
    }

    /// 期望集没被结论集覆盖即 UNFINISHED，且缺的那些要列出名字。
    /// 只判「有没有报错」的实现会在这里判成 COMPLETED。
    #[test]
    fn checks_that_never_reported_a_conclusion_are_named() {
        let o = summarize_run(
            &RunTally {
                expected_codes: v(&["A", "B", "C"]),
                concluded_codes: v(&["A"]),
                inconclusive_codes: v(&["B"]),
                ..RunTally::default()
            },
            rid(),
        );
        assert_eq!(o.status, ReconRunStatus::Unfinished);
        assert_eq!(o.unfinished_check_codes, v(&["B", "C"]), "C 从未产生结论");
        assert_eq!(o.executed_check_codes, v(&["A"]));
    }

    /// 归因项与「从未产生结论」的项去重合并，不重复列名。
    #[test]
    fn the_unfinished_list_is_deduplicated() {
        let o = summarize_run(
            &RunTally {
                expected_codes: v(&["A"]),
                inconclusive_codes: v(&["A"]),
                ..RunTally::default()
            },
            rid(),
        );
        assert_eq!(o.unfinished_check_codes, v(&["A"]));
    }

    /// `Failed` 与 `Completed` 都不得带未完成清单——
    /// 带着它意味着那份证据会被关账闸门丢掉（闸门只在 UNFINISHED 那一臂读它）。
    #[test]
    fn only_unfinished_carries_the_unfinished_list() {
        let failed = summarize_run(&RunTally::default(), rid());
        assert!(failed.unfinished_check_codes.is_empty());
        assert_eq!(validate_run_outcome(&failed), Ok(()));

        let completed = summarize_run(
            &RunTally {
                expected_codes: v(&["A"]),
                concluded_codes: v(&["A"]),
                ..RunTally::default()
            },
            rid(),
        );
        assert!(completed.unfinished_check_codes.is_empty());
        assert_eq!(validate_run_outcome(&completed), Ok(()));
    }

    /// `summarize_run` 在任何输入下都不产出 `Running`。
    /// 这条钉住两个生产者的边界：`Running` 只能由 [`ReconRunOutcome::running`] 给。
    #[test]
    fn summarize_never_produces_running() {
        let inputs = [
            RunTally::default(),
            RunTally {
                expected_codes: v(&["A"]),
                ..RunTally::default()
            },
            RunTally {
                expected_codes: v(&["A"]),
                concluded_codes: v(&["A"]),
                aborted_before_any_check: true,
                discrepancy_count: 3,
                ..RunTally::default()
            },
        ];
        for t in inputs {
            assert_ne!(summarize_run(&t, rid()).status, ReconRunStatus::Running);
        }
    }

    /// 手搓的坏值要被 `validate_run_outcome` 拦住。
    /// 头一个是最要紧的：`Completed` 配非空未完成清单——今天类型上构造得出。
    #[test]
    fn hand_built_inconsistent_outcomes_are_caught() {
        let bad = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Completed,
            discrepancy_count: 0,
            executed_check_codes: v(&["A"]),
            unfinished_check_codes: v(&["B"]),
        };
        assert!(validate_run_outcome(&bad).is_err(), "证据会被闸门静默丢掉");

        let empty_unfinished = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Unfinished,
            discrepancy_count: 0,
            executed_check_codes: Vec::new(),
            unfinished_check_codes: Vec::new(),
        };
        assert!(validate_run_outcome(&empty_unfinished).is_err());

        let empty_completed = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Completed,
            discrepancy_count: 0,
            executed_check_codes: Vec::new(),
            unfinished_check_codes: Vec::new(),
        };
        assert!(
            validate_run_outcome(&empty_completed).is_err(),
            "一项都没跑过不是「通过」"
        );

        let both = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Unfinished,
            discrepancy_count: 0,
            executed_check_codes: v(&["A"]),
            unfinished_check_codes: v(&["A"]),
        };
        assert!(
            validate_run_outcome(&both).is_err(),
            "同一项不能既跑完又没跑完"
        );
    }

    /// `summarize_run` 产出的东西一律自洽——两个函数不能各说各话。
    #[test]
    fn everything_summarize_produces_validates() {
        let cases = [
            RunTally::default(),
            RunTally {
                expected_codes: v(&["A", "B"]),
                concluded_codes: v(&["A"]),
                ..RunTally::default()
            },
            RunTally {
                expected_codes: v(&["A"]),
                concluded_codes: v(&["A"]),
                discrepancy_count: 9,
                ..RunTally::default()
            },
        ];
        for t in cases {
            let o = summarize_run(&t, rid());
            assert_eq!(validate_run_outcome(&o), Ok(()), "自产的结果不自洽：{o:?}");
        }
    }

    /// 差异条数原样带出，不因状态被抹掉。
    #[test]
    fn the_discrepancy_count_survives_every_branch() {
        let o = summarize_run(
            &RunTally {
                expected_codes: v(&["A", "B"]),
                concluded_codes: v(&["A"]),
                discrepancy_count: 7,
                ..RunTally::default()
            },
            rid(),
        );
        assert_eq!(o.status, ReconRunStatus::Unfinished);
        assert_eq!(o.discrepancy_count, 7, "没跑完不代表已查到的差异不算数");
    }
}
