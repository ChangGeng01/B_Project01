//! 补偿的选取与次序。
//!
//! 计划第 3.4.8 节逐字：进入 `COMPENSATING` 后，按 `process_steps` 中
//! `outcome = 'COMPLETED'` 且 `is_compensable = true` 的记录**按 `step_no` 降序**
//! 逐条执行，每条一个事务。
//!
//! 这一层单列并单测，因为它有两个各自都能悄悄出错的地方：
//! 选谁（选错会补偿一个从未成功的步骤，或漏掉一个已经生效的步骤）、
//! 按什么序（顺序反了会先撤销后面的依赖、再撤销前面的前提，把状态搅得更乱）。
//! 两处都不会当场报错，要到出事时才发现。

/// 一个已执行步骤的补偿相关事实。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StepRecord {
    /// 实例内的步骤序号，自增且唯一。
    pub step_no: u32,
    /// 步骤标识，用于写 `process_compensations.reverses_id` 时定位。
    pub step_id: String,
    /// 步骤结局。只有 `COMPLETED` 的步骤才谈得上补偿。
    pub outcome: StepOutcome,
    /// 定义是否声明该步骤可补偿。
    pub is_compensable: bool,
}

/// 步骤结局。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepOutcome {
    Completed,
    Failed,
    Skipped,
}

impl StepOutcome {
    pub fn as_db_value(self) -> &'static str {
        match self {
            StepOutcome::Completed => "COMPLETED",
            StepOutcome::Failed => "FAILED",
            StepOutcome::Skipped => "SKIPPED",
        }
    }
}

/// 选出待补偿的步骤，并排成执行次序。
///
/// **严格逆序**：`step_no` 降序。这不是风格偏好——补偿是撤销，
/// 而后发生的事往往依赖先发生的事，先撤后者再撤前者才是安全的拆解次序。
/// 反过来做会在撤销前提时发现依赖还在。
///
/// 入参不要求已排序：调用方从库里按什么序读出来都不影响结果，
/// 这样就不会出现「SQL 忘了写 order by，代码这边也没排」的双重疏漏。
pub fn plan(steps: &[StepRecord]) -> Vec<&StepRecord> {
    let mut selected: Vec<&StepRecord> = steps
        .iter()
        .filter(|s| s.outcome == StepOutcome::Completed && s.is_compensable)
        .collect();
    // 逆序：用 Reverse 而不是交换比较两边——交换两边同样对，但一眼看不出是逆序，
    // 而这里「是不是逆序」正是整个函数的要害。
    selected.sort_by_key(|s| std::cmp::Reverse(s.step_no));
    selected
}

/// 一条补偿执行完之后该做什么。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CompensationStep {
    /// 本条补偿成功，继续下一条。
    Next,
    /// 全部补偿完成。
    AllDone,
    /// 本条补偿重试耗尽：**必须先写 COMPENSATION_FAILURE 人工任务并告警**，
    /// 再把实例迁到 `MANUAL_INTERVENTION`。不得静默结束。
    Escalate,
}

/// 一条补偿之后的判定。
///
/// `remaining_after_this` 是本条之后还剩几条待补偿。
///
/// **补偿失败一律升级，不允许跳过继续**：跳过会让一部分效果被撤销、
/// 一部分留着，那个中间态没有任何人在看，而它已经不是一个合法的业务状态了。
/// 计划第 3.4.8 节与规格第 9.1 节都要求补偿失败进人工任务队列并告警。
pub fn judge(succeeded: bool, remaining_after_this: usize) -> CompensationStep {
    if !succeeded {
        return CompensationStep::Escalate;
    }
    if remaining_after_this == 0 {
        CompensationStep::AllDone
    } else {
        CompensationStep::Next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(step_no: u32, outcome: StepOutcome, is_compensable: bool) -> StepRecord {
        StepRecord {
            step_no,
            step_id: format!("step-{step_no}"),
            outcome,
            is_compensable,
        }
    }

    #[test]
    fn compensation_runs_in_strictly_descending_step_order() {
        let steps = vec![
            step(1, StepOutcome::Completed, true),
            step(2, StepOutcome::Completed, true),
            step(3, StepOutcome::Completed, true),
        ];
        let order: Vec<u32> = plan(&steps).iter().map(|s| s.step_no).collect();
        assert_eq!(order, vec![3, 2, 1], "补偿必须严格逆序");
    }

    /// 入参乱序也要排对——不依赖调用方的 SQL 写没写 order by。
    #[test]
    fn input_order_does_not_matter() {
        let steps = vec![
            step(2, StepOutcome::Completed, true),
            step(3, StepOutcome::Completed, true),
            step(1, StepOutcome::Completed, true),
        ];
        let order: Vec<u32> = plan(&steps).iter().map(|s| s.step_no).collect();
        assert_eq!(order, vec![3, 2, 1]);
    }

    /// 只补偿「已完成且可补偿」的。两个条件各自都要挡住。
    #[test]
    fn only_completed_and_compensable_steps_are_selected() {
        let steps = vec![
            step(1, StepOutcome::Completed, true),
            // 失败的步骤没有生效，不该补偿——补偿一个没做过的事会造出反向效果。
            step(2, StepOutcome::Failed, true),
            // 跳过的同理。
            step(3, StepOutcome::Skipped, true),
            // 完成了但定义声明不可补偿。
            step(4, StepOutcome::Completed, false),
        ];
        let order: Vec<u32> = plan(&steps).iter().map(|s| s.step_no).collect();
        assert_eq!(order, vec![1], "只有第 1 步该被补偿");
    }

    /// 没有可补偿的步骤时返回空——而不是把整批都当成可补偿。
    #[test]
    fn no_compensable_step_yields_empty_plan() {
        let steps = vec![
            step(1, StepOutcome::Failed, true),
            step(2, StepOutcome::Completed, false),
        ];
        assert!(plan(&steps).is_empty());
    }

    #[test]
    fn success_advances_and_finishes() {
        assert_eq!(judge(true, 2), CompensationStep::Next);
        assert_eq!(judge(true, 0), CompensationStep::AllDone);
    }

    /// 补偿失败一律升级，**即使后面还有别的可补偿步骤也不继续**。
    /// 跳过失败的那条继续往下补，会造出一个「部分撤销」的中间态——
    /// 它不是任何一个合法业务状态，而且没有人在看它。
    #[test]
    fn failure_escalates_and_never_silently_continues() {
        assert_eq!(judge(false, 5), CompensationStep::Escalate);
        assert_eq!(judge(false, 0), CompensationStep::Escalate);
    }

    #[test]
    fn outcome_db_values() {
        assert_eq!(StepOutcome::Completed.as_db_value(), "COMPLETED");
        assert_eq!(StepOutcome::Failed.as_db_value(), "FAILED");
        assert_eq!(StepOutcome::Skipped.as_db_value(), "SKIPPED");
    }
}
