//! 关账受理的对账前提判定。
//!
//! 规格第 10.2 章逐字：**差异清零前不得关账**。这一条是本 crate 最重的一句——
//! 判松了会让一个账实不符的期间被关掉，而关账之后该期间不再接受任何凭证写入
//! （规格第 5.2 章「已关闭期间不再接受任何凭证写入」），差异就**永远修不掉了**；
//! 首版又不做反结账（规格第 5.7 章登记为延期项），连回头的路都没有。
//!
//! 判紧了只是关不了账，运维会来问；判松了是不可逆的。两种错的代价不对称，
//! 因此本模块的每一条判定都取**保守的那一侧**，并在下面逐条写明取的是哪一侧。

use crate::model::{DiscrepancyState, ReconRunStatus};

/// 关账受理被拒的原因。**逐条分开，不合成一个布尔**：
/// 运维要知道的是「卡在哪一项、还差几条」，「不满足前提」五个字帮不上忙。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CloseBlocker {
    /// 对账压根没跑过。
    NoReconRun,
    /// 对账还在跑。
    ReconRunning,
    /// 对账运行本身失败了。
    ReconFailed,
    /// 对账跑完了但有检查项没跑到底。列出是哪几项——
    /// 与「有差异」是两回事：没跑到底意味着**那部分账实相符与否根本不知道**。
    ReconUnfinished { check_codes: Vec<String> },
    /// 还有未了结的差异。
    OpenDiscrepancies { count: u32 },
    /// 还有未了结的死信。规格第 10.2 章的另一项前提。
    UnsettledDeadLetters { count: u32 },
}

impl std::fmt::Display for CloseBlocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloseBlocker::NoReconRun => f.write_str("该法人该期间尚未执行过对账"),
            CloseBlocker::ReconRunning => f.write_str("对账正在执行中，请等其结束"),
            CloseBlocker::ReconFailed => f.write_str("最近一次对账运行失败，需先排除故障并重跑"),
            CloseBlocker::ReconUnfinished { check_codes } => write!(
                f,
                "对账有 {} 个检查项未跑到底：{}；未跑到底不等于无差异",
                check_codes.len(),
                check_codes.join("、")
            ),
            CloseBlocker::OpenDiscrepancies { count } => {
                write!(f, "尚有 {count} 条未了结的对账差异")
            }
            CloseBlocker::UnsettledDeadLetters { count } => {
                write!(f, "尚有 {count} 条未了结的死信")
            }
        }
    }
}

/// 关账受理时能看到的事实。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CloseFacts {
    /// 该法人该期间最近一次对账运行的状态。`None` 表示从没跑过。
    pub latest_run: Option<ReconRunStatus>,
    /// 该次运行里没跑到底的检查项。
    pub unfinished_check_codes: Vec<String>,
    /// 该期间全部差异事项的状态。
    pub discrepancies: Vec<DiscrepancyState>,
    /// 该法人该记账日期范围内未了结的死信条数。
    pub unsettled_dead_letters: u32,
}

/// 判定关账能否受理。返回全部阻断项，**不在第一项就短路**。
///
/// 不短路是有意的：短路会让运维修完第一项、再来一次、又被第二项挡住，
/// 一次只知道一件事。一次把全部阻断项给出来，他能一趟修完。
///
/// 空的返回值表示可以受理。
pub fn check_close_admission(facts: &CloseFacts) -> Vec<CloseBlocker> {
    let mut blockers = Vec::new();

    match facts.latest_run {
        // 从没跑过对账即拒。**不把「没跑过」当成「没差异」**——
        // 这是本模块最容易写反的一处：空集合的「无未了结差异」恒为真，
        // 若只看差异不看运行状态，一个从没对过账的期间会一路放行。
        None => blockers.push(CloseBlocker::NoReconRun),
        Some(ReconRunStatus::Running) => blockers.push(CloseBlocker::ReconRunning),
        Some(ReconRunStatus::Failed) => blockers.push(CloseBlocker::ReconFailed),
        Some(ReconRunStatus::Unfinished) => {
            blockers.push(CloseBlocker::ReconUnfinished {
                check_codes: facts.unfinished_check_codes.clone(),
            });
        }
        Some(ReconRunStatus::Completed) => {}
    }

    let open = facts
        .discrepancies
        .iter()
        .filter(|s| !s.is_settled())
        .count();
    if open > 0 {
        blockers.push(CloseBlocker::OpenDiscrepancies {
            count: u32::try_from(open).unwrap_or(u32::MAX),
        });
    }

    if facts.unsettled_dead_letters > 0 {
        blockers.push(CloseBlocker::UnsettledDeadLetters {
            count: facts.unsettled_dead_letters,
        });
    }

    blockers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clean() -> CloseFacts {
        CloseFacts {
            latest_run: Some(ReconRunStatus::Completed),
            unfinished_check_codes: Vec::new(),
            discrepancies: vec![DiscrepancyState::Repaired, DiscrepancyState::Waived],
            unsettled_dead_letters: 0,
        }
    }

    #[test]
    fn a_clean_period_can_be_closed() {
        assert!(check_close_admission(&clean()).is_empty());
    }

    /// 本模块最容易写反的一处：从没跑过对账 ≠ 没有差异。
    /// 只看差异集合的话，空集合的「无未了结差异」恒为真，
    /// 一个从没对过账的期间会一路放行——而关账不可逆。
    #[test]
    fn never_reconciled_is_not_the_same_as_no_discrepancy() {
        let facts = CloseFacts {
            latest_run: None,
            discrepancies: Vec::new(),
            ..clean()
        };
        let blockers = check_close_admission(&facts);
        assert_eq!(blockers, vec![CloseBlocker::NoReconRun]);
    }

    /// 没跑到底也不算无差异，且要列出是哪几项。
    #[test]
    fn unfinished_run_blocks_and_names_the_checks() {
        let facts = CloseFacts {
            latest_run: Some(ReconRunStatus::Unfinished),
            unfinished_check_codes: vec!["SUB_VS_LED_AR".into(), "INV_QTY_VALUE".into()],
            discrepancies: Vec::new(),
            ..clean()
        };
        match &check_close_admission(&facts)[..] {
            [CloseBlocker::ReconUnfinished { check_codes }] => {
                assert_eq!(check_codes.len(), 2);
            }
            other => panic!("应报未跑完并列出检查项，实为 {other:?}"),
        }
    }

    /// 修复中的差异不算了结。
    #[test]
    fn repairing_discrepancy_still_blocks() {
        let facts = CloseFacts {
            discrepancies: vec![DiscrepancyState::Repairing, DiscrepancyState::Repaired],
            ..clean()
        };
        assert_eq!(
            check_close_admission(&facts),
            vec![CloseBlocker::OpenDiscrepancies { count: 1 }]
        );
    }

    /// 全部阻断项一次给全，不在第一项短路——
    /// 短路会让运维修一项来一趟，一次只知道一件事。
    #[test]
    fn all_blockers_are_reported_at_once() {
        let facts = CloseFacts {
            latest_run: Some(ReconRunStatus::Failed),
            unfinished_check_codes: Vec::new(),
            discrepancies: vec![DiscrepancyState::Open, DiscrepancyState::Open],
            unsettled_dead_letters: 3,
        };
        let blockers = check_close_admission(&facts);
        assert_eq!(blockers.len(), 3, "三项都要报出来，实为 {blockers:?}");
        assert!(blockers.contains(&CloseBlocker::ReconFailed));
        assert!(blockers.contains(&CloseBlocker::OpenDiscrepancies { count: 2 }));
        assert!(blockers.contains(&CloseBlocker::UnsettledDeadLetters { count: 3 }));
    }

    /// 对账正在跑也不放行——跑到一半的结论不是结论。
    #[test]
    fn running_reconciliation_blocks() {
        let facts = CloseFacts {
            latest_run: Some(ReconRunStatus::Running),
            ..clean()
        };
        assert_eq!(
            check_close_admission(&facts),
            vec![CloseBlocker::ReconRunning]
        );
    }

    /// 死信未了结单独成一项前提（规格第 10.2 章）。
    #[test]
    fn unsettled_dead_letters_block_on_their_own() {
        let facts = CloseFacts {
            unsettled_dead_letters: 1,
            ..clean()
        };
        assert_eq!(
            check_close_admission(&facts),
            vec![CloseBlocker::UnsettledDeadLetters { count: 1 }]
        );
    }

    /// 阻断原因的文案要带上数字与项名——运维靠它决定下一步做什么。
    #[test]
    fn blocker_messages_carry_the_numbers() {
        let msg = CloseBlocker::OpenDiscrepancies { count: 7 }.to_string();
        assert!(msg.contains('7'), "文案要带条数，实为 {msg}");
        let msg = CloseBlocker::ReconUnfinished {
            check_codes: vec!["A_B".into()],
        }
        .to_string();
        assert!(msg.contains("A_B"), "文案要带检查项名，实为 {msg}");
    }
}
