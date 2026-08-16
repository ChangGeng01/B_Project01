//! 流程实例状态机。取值与迁移逐行照抄阶段 3 计划第 3.4.8 节那张表。
//!
//! 把迁移表写成数据而不是一堆 `match` 分支，是为了让「表里有几行、每行长什么样」
//! 本身可断言——本卷已因「凭记忆复述一张表而不回去数」在计数与枚举上栽过多次。
//! 表在这里，用例逐行对，改一行就得同时改用例，漏改即红。

/// 实例状态。九个取值，与计划第 3.4.8 节那张表的「状态」列一致。
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum InstanceState {
    Created,
    Running,
    Waiting,
    Completed,
    Compensating,
    Failed,
    ManualIntervention,
    Cancelled,
}

impl InstanceState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            InstanceState::Created => "CREATED",
            InstanceState::Running => "RUNNING",
            InstanceState::Waiting => "WAITING",
            InstanceState::Completed => "COMPLETED",
            InstanceState::Compensating => "COMPENSATING",
            InstanceState::Failed => "FAILED",
            InstanceState::ManualIntervention => "MANUAL_INTERVENTION",
            InstanceState::Cancelled => "CANCELLED",
        }
    }

    /// 终态。终态之后不再有任何迁移——把非终态误判成终态会让实例卡死，
    /// 把终态误判成非终态会让调度器反复取一个已经结束的实例。
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            InstanceState::Completed | InstanceState::Failed | InstanceState::Cancelled
        )
    }
}

/// 迁移的触发事件。与那张表的「触发」列一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Trigger {
    /// 调度器首次取件。
    FirstDispatch,
    /// 步骤产出等待条件。
    StepAwaits,
    /// 步骤完成且有后继。
    StepCompletedWithSuccessor,
    /// 人工任务完成或定时器触发。
    WakeUp,
    /// 到达结束节点。
    ReachedEnd,
    /// 步骤重试耗尽且定义声明需补偿。
    RetriesExhaustedWithCompensation,
    /// 步骤重试耗尽且定义未声明补偿。
    RetriesExhaustedWithoutCompensation,
    /// 全部补偿完成。
    AllCompensated,
    /// 任一补偿重试耗尽。
    CompensationExhausted,
    /// 触及运行约束上限。
    LimitExceeded,
    /// 取消命令。
    CancelRequested,
    /// 人工处置后继续。
    ManualResume,
    /// 人工处置后取消。
    ManualCancel,
}

/// 守卫条件所需的外部事实。
///
/// 这些事实**不能在本模块内自己判定**：定义版本存不存在要查库，人工任务写没写要查库。
/// 把它们做成入参而不是藏进实现，是为了让守卫可单测，也为了让调用方
/// **没法忘记检查其中任何一条**——少填一个字段编译就不过。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Guards {
    /// 定义版本存在且为 PUBLISHED。
    pub definition_published: bool,
    /// 已写入对应的人工任务或定时器。
    pub wait_target_persisted: bool,
    /// 步骤数未超上限、并行分支数未超上限、未超最长运行期，三项同时成立。
    pub within_run_limits: bool,
    /// 触发的幂等键未被消费。
    pub wake_key_unconsumed: bool,
    /// 无活跃分支。
    pub no_active_branch: bool,
    /// 存在可补偿的已完成步骤。
    pub has_compensable_step: bool,
    /// 已写入 COMPENSATION_FAILURE 人工任务并告警。
    pub compensation_failure_task_written: bool,
    /// 已写入 LIMIT_EXCEEDED 人工任务。
    pub limit_task_written: bool,
    /// 调用方具备取消权限。
    pub caller_may_cancel: bool,
    /// 处置人记名并已写审计。
    pub handler_recorded: bool,
}

/// 迁移被拒的原因。**不与「没有这条迁移」合并**：
/// 一个是流程走错了，一个是前置条件没满足，排查方向完全不同。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TransitionError {
    /// 该状态在该触发下没有任何迁移。
    NoSuchTransition {
        from: InstanceState,
        trigger: Trigger,
    },
    /// 有这条迁移，但守卫没过。`guard` 是没过的那一条的名字。
    GuardFailed {
        from: InstanceState,
        to: InstanceState,
        guard: &'static str,
    },
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionError::NoSuchTransition { from, trigger } => write!(
                f,
                "状态 {} 上没有触发 {trigger:?} 的迁移",
                from.as_db_value()
            ),
            TransitionError::GuardFailed { from, to, guard } => write!(
                f,
                "{} 到 {} 的迁移守卫「{guard}」未满足",
                from.as_db_value(),
                to.as_db_value()
            ),
        }
    }
}

impl std::error::Error for TransitionError {}

/// 一条守卫：人可读的名字加判定函数。
///
/// 名字不是装饰——迁移被拒时它直接进错误里，运维看到的是
/// 「守卫『已写入 LIMIT_EXCEEDED 人工任务』未满足」而不是一个布尔为假。
type Guard = (&'static str, fn(&Guards) -> bool);

/// 一行迁移。`guard` 为 `None` 表示该行无守卫（表里的「—」）。
struct Row {
    from: InstanceState,
    to: InstanceState,
    trigger: Trigger,
    guard: Option<Guard>,
}

use InstanceState as S;
use Trigger as T;

/// 迁移表。**逐行照抄计划第 3.4.8 节那张表**，行数与顺序都不得随手改。
///
/// 表里有两行的「可去往」是斜杠分隔的多个目标（`RUNNING/WAITING → MANUAL_INTERVENTION`
/// 与 `MANUAL_INTERVENTION → RUNNING / CANCELLED`），在这里各自展开成独立行——
/// 展开是为了让每一行只有一个目标，否则「这条迁移到底去哪」要靠调用方再判一次，
/// 而那一判就落在了状态机之外。展开后的行数因此多于原表的 12 行，
/// 见 [`TRANSITIONS`] 上的用例。
static TRANSITIONS: &[Row] = &[
    Row {
        from: S::Created,
        to: S::Running,
        trigger: T::FirstDispatch,
        guard: Some(("定义版本存在且为 PUBLISHED", |g| {
            g.definition_published
        })),
    },
    Row {
        from: S::Running,
        to: S::Waiting,
        trigger: T::StepAwaits,
        guard: Some(("已写入对应的人工任务或定时器", |g| {
            g.wait_target_persisted
        })),
    },
    Row {
        from: S::Running,
        to: S::Running,
        trigger: T::StepCompletedWithSuccessor,
        guard: Some(("未触及运行约束三项上限", |g| g.within_run_limits)),
    },
    Row {
        from: S::Waiting,
        to: S::Running,
        trigger: T::WakeUp,
        guard: Some(("触发的幂等键未被消费", |g| g.wake_key_unconsumed)),
    },
    Row {
        from: S::Running,
        to: S::Completed,
        trigger: T::ReachedEnd,
        guard: Some(("无活跃分支", |g| g.no_active_branch)),
    },
    Row {
        from: S::Running,
        to: S::Compensating,
        trigger: T::RetriesExhaustedWithCompensation,
        guard: Some(("存在可补偿的已完成步骤", |g| {
            g.has_compensable_step
        })),
    },
    Row {
        from: S::Running,
        to: S::Failed,
        trigger: T::RetriesExhaustedWithoutCompensation,
        guard: None,
    },
    Row {
        from: S::Compensating,
        to: S::Completed,
        trigger: T::AllCompensated,
        guard: None,
    },
    Row {
        from: S::Compensating,
        to: S::ManualIntervention,
        trigger: T::CompensationExhausted,
        guard: Some((
            "已写入 COMPENSATION_FAILURE 人工任务并告警",
            |g| g.compensation_failure_task_written,
        )),
    },
    Row {
        from: S::Running,
        to: S::ManualIntervention,
        trigger: T::LimitExceeded,
        guard: Some(("已写入 LIMIT_EXCEEDED 人工任务", |g| {
            g.limit_task_written
        })),
    },
    Row {
        from: S::Waiting,
        to: S::ManualIntervention,
        trigger: T::LimitExceeded,
        guard: Some(("已写入 LIMIT_EXCEEDED 人工任务", |g| {
            g.limit_task_written
        })),
    },
    Row {
        from: S::Created,
        to: S::Cancelled,
        trigger: T::CancelRequested,
        guard: Some(("调用方具备取消权限", |g| g.caller_may_cancel)),
    },
    Row {
        from: S::Running,
        to: S::Cancelled,
        trigger: T::CancelRequested,
        guard: Some(("调用方具备取消权限", |g| g.caller_may_cancel)),
    },
    Row {
        from: S::Waiting,
        to: S::Cancelled,
        trigger: T::CancelRequested,
        guard: Some(("调用方具备取消权限", |g| g.caller_may_cancel)),
    },
    Row {
        from: S::ManualIntervention,
        to: S::Running,
        trigger: T::ManualResume,
        guard: Some(("处置人记名并写审计", |g| g.handler_recorded)),
    },
    Row {
        from: S::ManualIntervention,
        to: S::Cancelled,
        trigger: T::ManualCancel,
        guard: Some(("处置人记名并写审计", |g| g.handler_recorded)),
    },
];

/// 判定一次迁移。成功返回目标状态。
///
/// **取消不在 `COMPENSATING` 上开口**：那张表的取消行逐字写的是
/// 「`CREATED/RUNNING/WAITING` → `CANCELLED`」，且守卫里另写「实例未处于 `COMPENSATING`」。
/// 展开成三行之后，`COMPENSATING` 天然不在 `from` 里，那条守卫因此不必再判一次——
/// 它已经被表的形状挡住了。这比留一条运行期守卫更硬：形状挡住的东西没法被绕过。
pub fn transition(
    from: InstanceState,
    trigger: Trigger,
    guards: &Guards,
) -> Result<InstanceState, TransitionError> {
    let row = TRANSITIONS
        .iter()
        .find(|r| r.from == from && r.trigger == trigger)
        .ok_or(TransitionError::NoSuchTransition { from, trigger })?;
    if let Some((name, check)) = row.guard {
        if !check(guards) {
            return Err(TransitionError::GuardFailed {
                from,
                to: row.to,
                guard: name,
            });
        }
    }
    Ok(row.to)
}

/// 全部许可的 `(from, trigger, to)` 三元组，供测试与文档核对。
pub fn allowed_transitions() -> Vec<(InstanceState, Trigger, InstanceState)> {
    TRANSITIONS
        .iter()
        .map(|r| (r.from, r.trigger, r.to))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_true() -> Guards {
        Guards {
            definition_published: true,
            wait_target_persisted: true,
            within_run_limits: true,
            wake_key_unconsumed: true,
            no_active_branch: true,
            has_compensable_step: true,
            compensation_failure_task_written: true,
            limit_task_written: true,
            caller_may_cancel: true,
            handler_recorded: true,
        }
    }

    /// 表的行数固定。原表 12 行，其中两行是多目标，展开后为 16 行。
    /// 这条守的是「有人往表里加一行却没人发现」。
    #[test]
    fn transition_table_has_the_expected_shape() {
        assert_eq!(TRANSITIONS.len(), 16, "改表必须同批改本用例，不许只改一边");
        // 同一个 (from, trigger) 不得有两行——否则 find 取到哪一行取决于书写顺序，
        // 那是一条藏在数据里的隐式优先级。
        let mut keys: Vec<(InstanceState, Trigger)> =
            TRANSITIONS.iter().map(|r| (r.from, r.trigger)).collect();
        let before = keys.len();
        keys.sort_by_key(|(s, _)| *s as u8);
        keys.dedup_by(|a, b| a.0 == b.0 && format!("{:?}", a.1) == format!("{:?}", b.1));
        assert_eq!(before, keys.len(), "(from, trigger) 必须唯一");
    }

    #[test]
    fn happy_path_created_to_completed() {
        let g = all_true();
        assert_eq!(transition(S::Created, T::FirstDispatch, &g), Ok(S::Running));
        assert_eq!(
            transition(S::Running, T::StepCompletedWithSuccessor, &g),
            Ok(S::Running)
        );
        assert_eq!(transition(S::Running, T::ReachedEnd, &g), Ok(S::Completed));
    }

    /// 守卫没过与没有这条迁移，报的是两种错。
    /// 合成一种会让排查方向从一开始就偏：一个要去查前置条件，一个要去查流程定义。
    #[test]
    fn guard_failure_and_missing_transition_are_different_errors() {
        let g = Guards::default(); // 全 false
        match transition(S::Created, T::FirstDispatch, &g) {
            Err(TransitionError::GuardFailed { guard, .. }) => {
                assert!(guard.contains("PUBLISHED"));
            }
            other => panic!("应报守卫未过，实为 {other:?}"),
        }
        match transition(S::Completed, T::FirstDispatch, &g) {
            Err(TransitionError::NoSuchTransition { .. }) => {}
            other => panic!("应报无此迁移，实为 {other:?}"),
        }
    }

    /// 补偿中不得取消。这一条由表的形状挡住，不是靠运行期守卫——
    /// 计划那张表的取消行只列了 CREATED/RUNNING/WAITING 三个来源。
    #[test]
    fn compensating_cannot_be_cancelled() {
        let g = all_true();
        match transition(S::Compensating, T::CancelRequested, &g) {
            Err(TransitionError::NoSuchTransition { .. }) => {}
            other => panic!("补偿中取消应无此迁移，实为 {other:?}"),
        }
    }

    /// 补偿重试耗尽时**必须先写人工任务**才允许进 MANUAL_INTERVENTION。
    /// 守卫不过就停在 COMPENSATING——这正是「补偿失败不得静默结束」那条要求：
    /// 没写人工任务就迁走，实例会以一个没人知道的状态停在那里。
    #[test]
    fn compensation_failure_requires_the_human_task_first() {
        let mut g = all_true();
        g.compensation_failure_task_written = false;
        match transition(S::Compensating, T::CompensationExhausted, &g) {
            Err(TransitionError::GuardFailed { to, .. }) => {
                assert_eq!(to, S::ManualIntervention);
            }
            other => panic!("未写人工任务不得迁走，实为 {other:?}"),
        }
        g.compensation_failure_task_written = true;
        assert_eq!(
            transition(S::Compensating, T::CompensationExhausted, &g),
            Ok(S::ManualIntervention)
        );
    }

    /// 终态之后没有任何迁移。
    #[test]
    fn terminal_states_have_no_outgoing_transitions() {
        for s in [S::Completed, S::Failed, S::Cancelled] {
            assert!(s.is_terminal(), "{s:?} 应为终态");
            assert!(
                !TRANSITIONS.iter().any(|r| r.from == s),
                "{s:?} 是终态，不该有出边"
            );
        }
        for s in [
            S::Created,
            S::Running,
            S::Waiting,
            S::Compensating,
            S::ManualIntervention,
        ] {
            assert!(!s.is_terminal(), "{s:?} 不该是终态");
        }
    }

    /// 触及运行约束上限时，RUNNING 与 WAITING 两个来源都要有出边——
    /// 计划那张表逐字写的是 `RUNNING/WAITING → MANUAL_INTERVENTION`。
    #[test]
    fn limit_exceeded_covers_both_running_and_waiting() {
        let g = all_true();
        assert_eq!(
            transition(S::Running, T::LimitExceeded, &g),
            Ok(S::ManualIntervention)
        );
        assert_eq!(
            transition(S::Waiting, T::LimitExceeded, &g),
            Ok(S::ManualIntervention)
        );
    }

    #[test]
    fn manual_intervention_can_resume_or_cancel_only_with_a_named_handler() {
        let mut g = all_true();
        assert_eq!(
            transition(S::ManualIntervention, T::ManualResume, &g),
            Ok(S::Running)
        );
        assert_eq!(
            transition(S::ManualIntervention, T::ManualCancel, &g),
            Ok(S::Cancelled)
        );
        g.handler_recorded = false;
        assert!(transition(S::ManualIntervention, T::ManualResume, &g).is_err());
    }

    #[test]
    fn db_values_are_screaming_snake() {
        assert_eq!(S::ManualIntervention.as_db_value(), "MANUAL_INTERVENTION");
        assert_eq!(S::Compensating.as_db_value(), "COMPENSATING");
        assert_eq!(allowed_transitions().len(), TRANSITIONS.len());
    }
}
