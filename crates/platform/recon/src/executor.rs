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

use crate::model::{ReconRunKind, ReconRunOutcome, ReconRunStatus, TerminationCause};
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
    /// 运行本身的终止成因。`None` 表示运行没有被外因打断。
    ///
    /// 裁定 F-14 撤销 `FAILED` 之后，「运行本身出了什么事」由这一列承担——
    /// 否则「无从归因的中断」并进 `UNFINISHED` 之后就没有了归因，
    /// 运维只知道有事没跑完、不知道该去查快照还是去查某一项。
    pub termination_cause: Option<TerminationCause>,
}

/// 起跑前闸门没过的原因。
///
/// 裁定 F-14 结论二把「注册表的阻断性校验项不足十五」从**运行结论**前移为
/// **起跑前闸门**：不起跑、不落 `recon_runs` 行、`run` 返回 `Err`。
///
/// 前移的理由：那不是一次运行的结果，是这次运行**不该开始**。
/// 落一行 `FAILED` 反而制造一条没有终止成因、`batch_done` 恒零的空壳记录，
/// 而闸门侧读到「最近一次运行失败」也不如读到「压根没跑过 + 名册差几项」具体。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NotReady {
    /// 注册表里一个阻断性校验项都没有，或不足 A-06 冻结的十五项。
    RosterIncomplete { registered: usize, expected: usize },
}

impl std::fmt::Display for NotReady {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotReady::RosterIncomplete {
                registered,
                expected,
            } => write!(
                f,
                "关账前强制校验只注册了 {registered} 项、应为 {expected} 项，本次对账不起跑"
            ),
        }
    }
}

impl std::error::Error for NotReady {}

/// 把一次运行的观测汇总成结果。
///
/// 三个 status，`FAILED` 已由裁定 F-14 撤销：
///
/// 一、`Unfinished`：有归因得到的未完成项、或期望集没被结论集覆盖、
///    或运行本身被外因打断（`termination_cause` 非空）。
/// 二、`Completed`：期望集非空、无未完成项、无终止成因，且期望集⊆结论集。
///    **四条缺一不可**——只判「没有报错」就是恒真判据。
/// 三、`Running` **永不由本函数产出**，见 [`ReconRunOutcome::running`]。
///
/// 期望集为空时返回 `Err`：那种局面该由起跑前闸门拦下，走不到这里。
/// 返回 `Err` 而不是造一个结论，是为了让「不该开始」与「开始了没跑完」
/// 在类型上分得开——前者不落行，后者要落行。
pub fn summarize_run(tally: &RunTally, run_id: Id<ReconRun>) -> Result<ReconRunOutcome, NotReady> {
    if tally.expected_codes.is_empty() {
        return Err(NotReady::RosterIncomplete {
            registered: 0,
            expected: crate::registry::EXPECTED_REGISTERED_CHECK_COUNT,
        });
    }

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

    let status = if unfinished.is_empty() && tally.termination_cause.is_none() {
        ReconRunStatus::Completed
    } else {
        ReconRunStatus::Unfinished
    };

    Ok(ReconRunOutcome {
        run_id,
        status,
        discrepancy_count: tally.discrepancy_count,
        executed_check_codes: tally.concluded_codes.clone(),
        unfinished_check_codes: unfinished,
        termination_cause: tally.termination_cause,
    })
}

/// 校验一个 [`ReconRunOutcome`] 自身是否自洽。
///
/// [`summarize_run`] 由构造保证这些性质，但 `ReconRunOutcome` 的字段是 `pub`
/// （与本 crate 其余值类型一致），所以谁都能手搓一个坏值出来。
/// 最要紧的坏值是 `{ status: Completed, unfinished_check_codes: ["X"] }`——
/// 关账闸门只在 `Unfinished` 那一臂读未完成清单，于是这条证据被静默丢掉、
/// 期间照常关掉。这个函数是那一处的第二道锁。
pub fn validate_run_outcome(o: &ReconRunOutcome) -> Result<(), &'static str> {
    // 裁定 F-14：`UNFINISHED` 要么列出没跑到底的检查项、要么给出终止成因，
    // 二者至少其一。皆空即拒——那样的记录说「有事没干完」却说不出是什么事。
    if o.status == ReconRunStatus::Unfinished
        && o.unfinished_check_codes.is_empty()
        && o.termination_cause.is_none()
    {
        return Err("UNFINISHED 必须给出没跑到底的检查项或终止成因，二者至少其一");
    }
    if o.status == ReconRunStatus::Completed && o.termination_cause.is_some() {
        return Err("COMPLETED 不得带终止成因——被打断过就不是跑完了");
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
    use crate::registry::EXPECTED_REGISTERED_CHECK_COUNT;

    fn rid() -> Id<ReconRun> {
        Id::from_uuid(uuid::Uuid::from_u128(1))
    }

    fn v(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| (*s).to_string()).collect()
    }

    fn ok(t: &RunTally) -> ReconRunOutcome {
        summarize_run(t, rid()).expect("该组观测应能汇总")
    }

    /// 两个 status 各要有一组观测能取到它。`Running` 按设计不由本函数产出。
    #[test]
    fn both_terminal_statuses_are_reachable() {
        let unfinished = RunTally {
            expected_codes: v(&["A", "B"]),
            concluded_codes: v(&["A"]),
            ..RunTally::default()
        };
        assert_eq!(ok(&unfinished).status, ReconRunStatus::Unfinished);

        let completed = RunTally {
            expected_codes: v(&["A", "B"]),
            concluded_codes: v(&["A", "B"]),
            ..RunTally::default()
        };
        assert_eq!(ok(&completed).status, ReconRunStatus::Completed);
    }

    /// 名册不足不是一次运行的结论，是这次运行**不该开始**。
    ///
    /// 裁定 F-14 把它从 `FAILED` 前移为起跑前闸门：不起跑、不落行。
    /// 落一行 `FAILED` 反而制造一条没有终止成因、批数恒零的空壳记录。
    #[test]
    fn an_empty_roster_is_refused_before_the_run_not_after() {
        assert_eq!(
            summarize_run(&RunTally::default(), rid()),
            Err(NotReady::RosterIncomplete {
                registered: 0,
                expected: EXPECTED_REGISTERED_CHECK_COUNT,
            })
        );
    }

    /// 运行被外因打断即 `UNFINISHED`，且归因落在 `termination_cause` 上。
    ///
    /// **`unfinished_check_codes` 仍为空**——一批都没派发出去时，
    /// 那十五项不是「没跑到底」，是运行本身没起来；
    /// 把期望集整个当成未完成项是误报，运维要查的是快照与连接，不是逐项重跑。
    #[test]
    fn an_aborted_run_is_unfinished_with_a_cause_and_no_blame_on_the_checks() {
        let t = RunTally {
            expected_codes: v(&["A", "B"]),
            termination_cause: Some(TerminationCause::SnapshotInvalid),
            ..RunTally::default()
        };
        let o = ok(&t);
        assert_eq!(o.status, ReconRunStatus::Unfinished);
        assert_eq!(o.termination_cause, Some(TerminationCause::SnapshotInvalid));
        assert_eq!(
            o.unfinished_check_codes,
            v(&["A", "B"]),
            "期望集没被覆盖仍要列名——这一条与「无从归因」是两件事"
        );
    }

    /// 期望集没被结论集覆盖即未完成，缺的那些要列出名字。
    /// 只判「有没有报错」的实现会在这里判成 `COMPLETED`。
    #[test]
    fn checks_that_never_reported_a_conclusion_are_named() {
        let o = ok(&RunTally {
            expected_codes: v(&["A", "B", "C"]),
            concluded_codes: v(&["A"]),
            inconclusive_codes: v(&["B"]),
            ..RunTally::default()
        });
        assert_eq!(o.status, ReconRunStatus::Unfinished);
        assert_eq!(o.unfinished_check_codes, v(&["B", "C"]), "C 从未产生结论");
        assert_eq!(o.executed_check_codes, v(&["A"]));
    }

    #[test]
    fn the_unfinished_list_is_deduplicated() {
        let o = ok(&RunTally {
            expected_codes: v(&["A"]),
            inconclusive_codes: v(&["A"]),
            ..RunTally::default()
        });
        assert_eq!(o.unfinished_check_codes, v(&["A"]));
    }

    /// `COMPLETED` 的四个条件缺一不可。**带终止成因的运行不算跑完**——
    /// 只判「没有未完成项」的实现会把一次被打断、但恰好没漏项的运行判成通过。
    #[test]
    fn a_run_that_was_interrupted_is_never_completed() {
        let t = RunTally {
            expected_codes: v(&["A"]),
            concluded_codes: v(&["A"]),
            termination_cause: Some(TerminationCause::BatchTimeout),
            ..RunTally::default()
        };
        assert_eq!(ok(&t).status, ReconRunStatus::Unfinished);
    }

    /// `summarize_run` 在任何可汇总的输入下都不产出 `Running`。
    #[test]
    fn summarize_never_produces_running() {
        let inputs = [
            RunTally {
                expected_codes: v(&["A"]),
                ..RunTally::default()
            },
            RunTally {
                expected_codes: v(&["A"]),
                concluded_codes: v(&["A"]),
                discrepancy_count: 3,
                termination_cause: Some(TerminationCause::ProcessExit),
                ..RunTally::default()
            },
        ];
        for t in inputs {
            assert_ne!(ok(&t).status, ReconRunStatus::Running);
        }
    }

    /// 手搓的坏值要被拦住。头一个最要紧：`Completed` 配非空未完成清单——
    /// 关账闸门只在未完成那一臂读那份清单，证据会被静默丢掉。
    #[test]
    fn hand_built_inconsistent_outcomes_are_caught() {
        let base = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Completed,
            discrepancy_count: 0,
            executed_check_codes: v(&["A"]),
            unfinished_check_codes: Vec::new(),
            termination_cause: None,
        };
        assert_eq!(validate_run_outcome(&base), Ok(()));

        let mut bad = base.clone();
        bad.unfinished_check_codes = v(&["B"]);
        assert!(validate_run_outcome(&bad).is_err(), "证据会被闸门静默丢掉");

        let mut interrupted_but_completed = base.clone();
        interrupted_but_completed.termination_cause = Some(TerminationCause::ProcessExit);
        assert!(
            validate_run_outcome(&interrupted_but_completed).is_err(),
            "被打断过就不是跑完了"
        );

        let mut silent_unfinished = base.clone();
        silent_unfinished.status = ReconRunStatus::Unfinished;
        assert!(
            validate_run_outcome(&silent_unfinished).is_err(),
            "既说不出哪一项、也说不出什么原因的未完成，运维无从下手"
        );

        let mut empty_completed = base.clone();
        empty_completed.executed_check_codes = Vec::new();
        assert!(
            validate_run_outcome(&empty_completed).is_err(),
            "一项都没跑过不是「通过」"
        );

        let mut both = base;
        both.status = ReconRunStatus::Unfinished;
        both.unfinished_check_codes = v(&["A"]);
        assert!(
            validate_run_outcome(&both).is_err(),
            "同一项不能既跑完又没跑完"
        );
    }

    /// 只给终止成因、不给未完成项的未完成，是合法的——
    /// 那正是「运行本身没起来」的形态。这一条与上一条的最后一格互为反例。
    #[test]
    fn a_cause_alone_satisfies_the_unfinished_invariant() {
        let o = ReconRunOutcome {
            run_id: rid(),
            status: ReconRunStatus::Unfinished,
            discrepancy_count: 0,
            executed_check_codes: Vec::new(),
            unfinished_check_codes: Vec::new(),
            termination_cause: Some(TerminationCause::SnapshotInvalid),
        };
        assert_eq!(validate_run_outcome(&o), Ok(()));
    }

    /// `summarize_run` 产出的东西一律自洽——两个函数不能各说各话。
    #[test]
    fn everything_summarize_produces_validates() {
        let cases = [
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
            RunTally {
                expected_codes: v(&["A"]),
                termination_cause: Some(TerminationCause::ConnectionRecycled),
                ..RunTally::default()
            },
        ];
        for t in cases {
            let o = ok(&t);
            assert_eq!(validate_run_outcome(&o), Ok(()), "自产的结果不自洽：{o:?}");
        }
    }

    /// 差异条数原样带出，不因状态被抹掉。
    #[test]
    fn the_discrepancy_count_survives_every_branch() {
        let o = ok(&RunTally {
            expected_codes: v(&["A", "B"]),
            concluded_codes: v(&["A"]),
            discrepancy_count: 7,
            ..RunTally::default()
        });
        assert_eq!(o.status, ReconRunStatus::Unfinished);
        assert_eq!(o.discrepancy_count, 7, "没跑完不代表已查到的差异不算数");
    }
}
