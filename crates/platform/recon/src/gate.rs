//! 关账受理的对账前提判定。
//!
//! 规格第 10.2 章逐字：**差异清零前不得关账**。这一条是本 crate 最重的一句——
//! 判松了会让一个账实不符的期间被关掉，而关账之后该期间不再接受任何凭证写入
//! （规格第 5.2 章「已关闭期间不再接受任何凭证写入」），差异就**永远修不掉了**；
//! 首版又不做反结账（规格第 5.7 章登记为延期项），连回头的路都没有。
//!
//! 判紧了只是关不了账，运维会来问；判松了是不可逆的。两种错的代价不对称，
//! 因此本模块的每一条判定都取**保守的那一侧**，并在下面逐条写明取的是哪一侧。

use crate::model::ReconRunStatus;
use crate::registry::EXPECTED_REGISTERED_CHECK_COUNT;

/// 关账受理被拒的原因。**逐条分开，不合成一个布尔**：
/// 运维要知道的是「卡在哪一项、还差几条」，「不满足前提」五个字帮不上忙。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CloseBlocker {
    /// 对账压根没跑过。
    NoReconRun,
    /// 对账还在跑。
    ///
    /// **这是一条活路径**：裁定 F-14 把 `recon_runs` 的登记改为 `IMMUTABLE_COLUMNS`，
    /// `RUNNING` 到终态的更新走得通，因而运行中确实读得到这个状态。
    /// 一次崩掉的运行会把行留在 `RUNNING` 上，届时闸门拦住关账而不是放行——
    /// 方向是保守的。谁替死掉的进程把它推到终态，见裁定 F-14 末节的未了结项。
    ReconRunning,
    /// 对账跑完了但有检查项没跑到底。列出是哪几项——
    /// 与「有差异」是两回事：没跑到底意味着**那部分账实相符与否根本不知道**。
    ReconUnfinished { check_codes: Vec<String> },
    /// 本次校验检出了阻断性差异。
    ///
    /// **不叫「未了结」**：按裁定 F-13 没有「了结」这个动作，
    /// 差异是修数据修掉的，不是标状态标掉的。
    OpenDiscrepancies { count: u32 },
    /// 还有未了结的死信。规格第 10.2 章的另一项前提。
    UnsettledDeadLetters { count: u32 },
    /// 注册表里的阻断性校验项不足裁定 A-06 冻结的十五项。
    ///
    /// **这一条的期望值来自 A-06 的十五，不来自注册表自己。** 这是要害：
    /// 拿注册表的内容当期望值，期望与实际必然相等、差集恒空，
    /// 那样的覆盖面判定是恒真的。
    ReconRosterIncomplete { registered: usize, expected: usize },
    /// 注册在册、但这次运行没跑到的阻断性校验项。
    ///
    /// 与上一条不是重复：上一条判「该有的项有没有注册」，这一条判
    /// 「注册了的项这次有没有跑」——一次中途断掉的运行会让后者非空而前者干净。
    ReconNotCovered { missing_check_codes: Vec<String> },
}

impl std::fmt::Display for CloseBlocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloseBlocker::NoReconRun => f.write_str("该法人该期间尚未执行过对账"),
            CloseBlocker::ReconRunning => f.write_str("对账正在执行中，请等其结束"),
            CloseBlocker::ReconUnfinished { check_codes } => write!(
                f,
                "对账有 {} 个检查项未跑到底：{}；未跑到底不等于无差异",
                check_codes.len(),
                check_codes.join("、")
            ),
            CloseBlocker::OpenDiscrepancies { count } => {
                write!(f, "本次校验检出 {count} 条阻断性差异；修复来源数据后重新发起关账")
            }
            CloseBlocker::UnsettledDeadLetters { count } => {
                write!(f, "尚有 {count} 条未了结的死信")
            }
            CloseBlocker::ReconRosterIncomplete {
                registered,
                expected,
            } => write!(
                f,
                "关账前强制校验只注册了 {registered} 项，应为 {expected} 项；缺的那些从未跑过，不等于无差异"
            ),
            CloseBlocker::ReconNotCovered {
                missing_check_codes,
            } => write!(
                f,
                "有 {} 个已注册的校验项本次未跑到：{}",
                missing_check_codes.len(),
                missing_check_codes.join("、")
            ),
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
    /// **本次 `PERIOD_CLOSE` 运行产出的、阻断性校验项的差异条数。**
    ///
    /// 按裁定 F-13：关账拦截的判据是**本次校验的校验项结论**，
    /// 不是 `recon_discrepancies` 的累计行集。历史差异行是台账，不参与判定。
    ///
    /// 上一版这里是「该期间全部差异事项的状态」`Vec<DiscrepancyState>`，
    /// 配 `!is_settled()` 过滤计数。那是错的，而且错得不显眼：
    /// 三个已了结取值在首版**没有生产者**（F-13 结论四），`is_settled()` 因此恒假、
    /// 过滤恒真，于是那个计数**一旦非零就再也减不回零**——
    /// 一个期间只要出过一条差异就永远关不上账。附录辛第 14 条观察到的死锁，
    /// 在代码里就是这一行。
    ///
    /// 现在的形状让「修数据、重跑、本次校验通过」这条规格逐字的解除路径能走通：
    /// 新的一次运行不再检出差异，这个数就是零。
    pub blocking_discrepancy_count: u32,
    /// 该法人该记账日期范围内未了结的死信条数。
    pub unsettled_dead_letters: u32,
    /// 受理时注册表里全部阻断性校验项的 code，取自 `ReconRegistry::blocking_codes`。
    pub registered_blocking_check_codes: Vec<String>,
    /// 最近一次运行真正跑到结论的检查项，取自 `ReconRunOutcome::executed_check_codes`。
    pub executed_check_codes: Vec<String>,
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
        Some(ReconRunStatus::Unfinished) => {
            blockers.push(CloseBlocker::ReconUnfinished {
                check_codes: facts.unfinished_check_codes.clone(),
            });
        }
        Some(ReconRunStatus::Completed) => {}
    }

    if facts.blocking_discrepancy_count > 0 {
        blockers.push(CloseBlocker::OpenDiscrepancies {
            count: facts.blocking_discrepancy_count,
        });
    }

    if facts.unsettled_dead_letters > 0 {
        blockers.push(CloseBlocker::UnsettledDeadLetters {
            count: facts.unsettled_dead_letters,
        });
    }

    // 名册齐备与覆盖面两条，期望值来自两个**互相独立**的源：
    // 前者来自 A-06 冻结的十五，后者来自注册表与运行结果的差。
    // 若两者同源，差集恒空，这两条就都是恒真的。
    if facts.registered_blocking_check_codes.len() != EXPECTED_REGISTERED_CHECK_COUNT {
        blockers.push(CloseBlocker::ReconRosterIncomplete {
            registered: facts.registered_blocking_check_codes.len(),
            expected: EXPECTED_REGISTERED_CHECK_COUNT,
        });
    }
    let missing: Vec<String> = facts
        .registered_blocking_check_codes
        .iter()
        .filter(|c| !facts.executed_check_codes.contains(c))
        .cloned()
        .collect();
    if !missing.is_empty() {
        blockers.push(CloseBlocker::ReconNotCovered {
            missing_check_codes: missing,
        });
    }

    blockers
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 十五个互异的码，凑齐 A-06 冻结的名册规模。
    fn roster() -> Vec<String> {
        (0..EXPECTED_REGISTERED_CHECK_COUNT)
            .map(|i| format!("C{i:02}"))
            .collect()
    }

    fn clean() -> CloseFacts {
        CloseFacts {
            latest_run: Some(ReconRunStatus::Completed),
            unfinished_check_codes: Vec::new(),
            blocking_discrepancy_count: 0,
            unsettled_dead_letters: 0,
            registered_blocking_check_codes: roster(),
            executed_check_codes: roster(),
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
            ..clean()
        };
        match &check_close_admission(&facts)[..] {
            [CloseBlocker::ReconUnfinished { check_codes }] => {
                assert_eq!(check_codes.len(), 2);
            }
            other => panic!("应报未跑完并列出检查项，实为 {other:?}"),
        }
    }

    /// 本次校验检出的阻断性差异即拦关账。
    ///
    /// 按裁定 F-13 这个数只统计**本次运行**的差异，历史行不进来——
    /// 于是「修数据、重跑、本次不再检出」这条规格逐字的解除路径能走通。
    /// 上一版这里统计的是该期间全部差异事项里「未了结」的那些，
    /// 而三个已了结取值首版没有生产者，那个过滤恒真、计数只增不减。
    #[test]
    fn discrepancies_found_by_this_run_block_the_close() {
        let facts = CloseFacts {
            blocking_discrepancy_count: 1,
            ..clean()
        };
        assert_eq!(
            check_close_admission(&facts),
            vec![CloseBlocker::OpenDiscrepancies { count: 1 }]
        );
    }

    /// 解除路径必须走得通：同一个期间，上一次跑出过差异、这一次没跑出，
    /// 就该放行。**不需要任何人给历史差异行改状态。**
    /// 这一条是 F-13 结论二在代码里的被测对象——
    /// 累计口径的实现过不了它，因为历史行永远在。
    #[test]
    fn a_rerun_with_no_discrepancy_releases_the_close() {
        let blocked = CloseFacts {
            blocking_discrepancy_count: 3,
            ..clean()
        };
        assert!(!check_close_admission(&blocked).is_empty());

        let after_repair = CloseFacts {
            blocking_discrepancy_count: 0,
            ..clean()
        };
        assert!(
            check_close_admission(&after_repair).is_empty(),
            "修好数据重跑之后必须能关账，否则「差异清零前不得关账」就是死锁"
        );
    }

    /// 全部阻断项一次给全，不在第一项短路——
    /// 短路会让运维修一项来一趟，一次只知道一件事。
    #[test]
    fn all_blockers_are_reported_at_once() {
        let facts = CloseFacts {
            latest_run: Some(ReconRunStatus::Unfinished),
            unfinished_check_codes: vec!["SUB_VS_LED_AR".into()],
            blocking_discrepancy_count: 2,
            unsettled_dead_letters: 3,
            ..clean()
        };
        let blockers = check_close_admission(&facts);
        assert_eq!(blockers.len(), 3, "三项都要报出来，实为 {blockers:?}");
        assert!(blockers
            .iter()
            .any(|b| matches!(b, CloseBlocker::ReconUnfinished { .. })));
        assert!(blockers.contains(&CloseBlocker::OpenDiscrepancies { count: 2 }));
        assert!(blockers.contains(&CloseBlocker::UnsettledDeadLetters { count: 3 }));
    }

    /// 名册不齐即拒。**期望值来自 A-06 冻结的十五，不来自注册表自己**——
    /// 拿注册表的内容当期望值，期望与实际必然相等、差集恒空，判定就是恒真的。
    ///
    /// 这不是假想的边界：阶段 9a 交付本体那一刻，十五项里有十一项还不存在，
    /// 一次关账前校验会跑完、无差异，而那十一项从没跑过。
    #[test]
    fn an_incomplete_roster_blocks_even_when_everything_else_is_clean() {
        let four: Vec<String> = roster().into_iter().take(4).collect();
        let facts = CloseFacts {
            registered_blocking_check_codes: four.clone(),
            executed_check_codes: four,
            ..clean()
        };
        let blockers = check_close_admission(&facts);
        assert_eq!(
            blockers,
            vec![CloseBlocker::ReconRosterIncomplete {
                registered: 4,
                expected: EXPECTED_REGISTERED_CHECK_COUNT,
            }],
            "只注册了四项却判成可关账——那十一项从没跑过"
        );
    }

    /// 一个注册在册却没跑到的校验项即拒，并列出是哪几个。
    /// 与名册那条不是重复：这一条在名册齐备时仍可能非空（运行中途断掉）。
    #[test]
    fn a_registered_check_that_never_ran_blocks_and_is_named() {
        let mut executed = roster();
        let dropped = executed.pop().expect("名册非空");
        let facts = CloseFacts {
            executed_check_codes: executed,
            ..clean()
        };
        assert_eq!(
            check_close_admission(&facts),
            vec![CloseBlocker::ReconNotCovered {
                missing_check_codes: vec![dropped],
            }]
        );
    }

    /// 反向锁：两条新阻断项不得变成「无脑拒绝」。
    /// 一份名册齐备、逐项跑到、零差异的事实必须返回空阻断——
    /// 少了这一条，把两条判定写成恒真也没人发现。
    #[test]
    fn the_two_new_blockers_are_not_always_on() {
        assert!(check_close_admission(&clean()).is_empty());
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
