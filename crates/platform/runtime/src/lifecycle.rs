//! 进程生命周期状态机。阶段 1 计划第 5.4 节的迁移表，逐条照抄。
//!
//! 非法迁移返回错误而不是 panic：把「状态机被误用」升级为进程崩溃，
//! 等于用一次编排失误换一次停机，而这台机器没有备节点。

use std::fmt;

use crate::process::ProcessKind;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum State {
    Init,
    Configuring,
    SelfChecking,
    Ready,
    Degraded,
    Draining,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Event {
    Start,
    ConfigLoaded,
    ConfigInvalid,
    AllPassed,
    PassedWithDegradation,
    AnyFailed,
    DegradationDetected,
    DegradationCleared,
    Sigterm,
    DrainComplete,
    Panic,
}

pub const ALL_STATES: [State; 8] = [
    State::Init,
    State::Configuring,
    State::SelfChecking,
    State::Ready,
    State::Degraded,
    State::Draining,
    State::Stopped,
    State::Failed,
];

pub const ALL_EVENTS: [Event; 11] = [
    Event::Start,
    Event::ConfigLoaded,
    Event::ConfigInvalid,
    Event::AllPassed,
    Event::PassedWithDegradation,
    Event::AnyFailed,
    Event::DegradationDetected,
    Event::DegradationCleared,
    Event::Sigterm,
    Event::DrainComplete,
    Event::Panic,
];

impl State {
    pub const fn as_str(self) -> &'static str {
        match self {
            State::Init => "INIT",
            State::Configuring => "CONFIGURING",
            State::SelfChecking => "SELF_CHECKING",
            State::Ready => "READY",
            State::Degraded => "DEGRADED",
            State::Draining => "DRAINING",
            State::Stopped => "STOPPED",
            State::Failed => "FAILED",
        }
    }
}

/// 非法迁移。category 取 BUSINESS_CONFLICT，与阶段 1 计划第 5.4 节一致。
///
/// 这里不取 `ep_foundation::AppError`：阶段 1 冻结的十三条错误码中没有一条
/// 表达「状态机非法迁移」，借用其中任何一条都会让错误码表与实际语义脱节，
/// 而新增第十四条不在本交付物的权限面内。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IllegalTransition {
    pub from: State,
    pub event: Event,
}

impl IllegalTransition {
    pub const fn category(&self) -> &'static str {
        "BUSINESS_CONFLICT"
    }
}

impl fmt::Display for IllegalTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "非法状态迁移：{:?} 状态下不接受事件 {:?}",
            self.from, self.event
        )
    }
}

impl std::error::Error for IllegalTransition {}

/// 配置错误与 Blocking 自检失败的退出码。systemd 以
/// `RestartPreventExitStatus=78` 对它不重启，避免配置错误导致重启风暴。
pub const EXIT_CONFIG_OR_SELFCHECK: u8 = 78;
/// panic 捕获后的退出码，允许重启。
pub const EXIT_PANIC: u8 = 70;

/// 单条迁移的判定。守卫条件由调用方在发事件之前判定，这里只判状态可达性。
pub fn next(from: State, event: Event) -> Result<State, IllegalTransition> {
    let to = match (from, event) {
        (_, Event::Panic) => State::Failed,
        (State::Init, Event::Start) => State::Configuring,
        (State::Configuring, Event::ConfigLoaded) => State::SelfChecking,
        (State::Configuring, Event::ConfigInvalid) => State::Failed,
        (State::SelfChecking, Event::AllPassed) => State::Ready,
        (State::SelfChecking, Event::PassedWithDegradation) => State::Degraded,
        (State::SelfChecking, Event::AnyFailed) => State::Failed,
        (State::Ready, Event::DegradationDetected) => State::Degraded,
        (State::Degraded, Event::DegradationCleared) => State::Ready,
        (State::Ready | State::Degraded, Event::Sigterm) => State::Draining,
        (State::Draining, Event::DrainComplete) => State::Stopped,
        _ => return Err(IllegalTransition { from, event }),
    };
    Ok(to)
}

/// 进程内持有当前状态。
#[derive(Debug)]
pub struct Lifecycle {
    process: ProcessKind,
    state: State,
}

impl Lifecycle {
    pub fn new(process: ProcessKind) -> Self {
        Self {
            process,
            state: State::Init,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn process(&self) -> ProcessKind {
        self.process
    }

    /// 迁移成功返回新状态；非法迁移不改变当前状态，由调用方记 ERROR。
    pub fn fire(&mut self, event: Event) -> Result<State, IllegalTransition> {
        let to = next(self.state, event)?;
        self.state = to;
        Ok(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 阶段 1 计划第 5.4 节迁移表的 12 条合法迁移，逐条写死。
    const LEGAL: [(State, Event, State); 12] = [
        (State::Init, Event::Start, State::Configuring),
        (State::Configuring, Event::ConfigLoaded, State::SelfChecking),
        (State::Configuring, Event::ConfigInvalid, State::Failed),
        (State::SelfChecking, Event::AllPassed, State::Ready),
        (
            State::SelfChecking,
            Event::PassedWithDegradation,
            State::Degraded,
        ),
        (State::SelfChecking, Event::AnyFailed, State::Failed),
        (State::Ready, Event::DegradationDetected, State::Degraded),
        (State::Degraded, Event::DegradationCleared, State::Ready),
        (State::Ready, Event::Sigterm, State::Draining),
        (State::Degraded, Event::Sigterm, State::Draining),
        (State::Draining, Event::DrainComplete, State::Stopped),
        (State::Ready, Event::Panic, State::Failed),
    ];

    #[test]
    fn twelve_legal_transitions_each_pass() {
        for (from, event, want) in LEGAL {
            assert_eq!(next(from, event), Ok(want), "{from:?} + {event:?}");
        }
    }

    #[test]
    fn panic_is_accepted_from_every_state() {
        for from in ALL_STATES {
            assert_eq!(next(from, Event::Panic), Ok(State::Failed));
        }
    }

    /// 负样例断言的是迁移表这条规则本身：笛卡尔积中除合法条目与 Panic 之外，
    /// 逐条必须报非法且 category 为 BUSINESS_CONFLICT。
    #[test]
    fn every_other_pair_in_the_cartesian_product_is_rejected() {
        let mut rejected = 0;
        for from in ALL_STATES {
            for event in ALL_EVENTS {
                let legal = LEGAL.iter().any(|(f, e, _)| *f == from && *e == event)
                    || event == Event::Panic;
                if legal {
                    continue;
                }
                let err = next(from, event).expect_err("{from:?} + {event:?} 不在迁移表内");
                assert_eq!(err.category(), "BUSINESS_CONFLICT");
                assert_eq!(err, IllegalTransition { from, event });
                rejected += 1;
            }
        }
        // 8 状态 × 11 事件 = 88 对，减 8 条各状态下的 Panic，减 11 条非 Panic 合法迁移。
        assert_eq!(rejected, 88 - 8 - 11);
    }

    #[test]
    fn illegal_transition_does_not_move_the_state() {
        let mut lc = Lifecycle::new(ProcessKind::CoreServer);
        assert_eq!(lc.fire(Event::DrainComplete).unwrap_err().from, State::Init);
        assert_eq!(lc.state(), State::Init, "非法迁移不得改变当前状态");
    }

    #[test]
    fn happy_path_reaches_stopped() {
        let mut lc = Lifecycle::new(ProcessKind::CoreServer);
        for e in [
            Event::Start,
            Event::ConfigLoaded,
            Event::AllPassed,
            Event::Sigterm,
            Event::DrainComplete,
        ] {
            lc.fire(e).expect("正常路径逐步可达");
        }
        assert_eq!(lc.state(), State::Stopped);
    }

    #[test]
    fn stopped_and_failed_are_terminal_except_panic() {
        for from in [State::Stopped, State::Failed] {
            for event in ALL_EVENTS.into_iter().filter(|e| *e != Event::Panic) {
                assert!(next(from, event).is_err(), "{from:?} 是终态");
            }
        }
    }
}
