//! Outbox 投递的状态机与退避排程。
//!
//! 取值出处是技术基线第 6.2 节：状态取 `PENDING`、`DISPATCHING`、`DONE`、`DEAD`；
//! 重试退避固定为 1 秒、5 秒、30 秒、2 分钟、10 分钟、30 分钟、1 小时、2 小时。
//!
//! 本模块只做纯判定：给定「当前已尝试几次」与「这次成不成」，算出下一步是什么。
//! 取件语句、行锁、`available_at` 的落库都在适配层，本 crate 不碰数据库。

use std::time::Duration;

/// 八档退避。基线第 6.2 节逐字给的取值，顺序即档次。
pub const BACKOFF_SCHEDULE: [Duration; 8] = [
    Duration::from_secs(1),
    Duration::from_secs(5),
    Duration::from_secs(30),
    Duration::from_secs(2 * 60),
    Duration::from_secs(10 * 60),
    Duration::from_secs(30 * 60),
    Duration::from_secs(60 * 60),
    Duration::from_secs(2 * 60 * 60),
];

/// 有待处理条目时的轮询间隔。
pub const POLL_INTERVAL_BUSY: Duration = Duration::from_millis(200);
/// 无待处理条目时退避到的轮询间隔。
pub const POLL_INTERVAL_IDLE: Duration = Duration::from_secs(2);
/// 单次取件批量上限。
pub const FETCH_BATCH_LIMIT: usize = 100;

/// Outbox 条目状态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutboxStatus {
    Pending,
    Dispatching,
    Done,
    Dead,
}

impl OutboxStatus {
    pub fn as_db_value(self) -> &'static str {
        match self {
            OutboxStatus::Pending => "PENDING",
            OutboxStatus::Dispatching => "DISPATCHING",
            OutboxStatus::Done => "DONE",
            OutboxStatus::Dead => "DEAD",
        }
    }
}

/// 一次投递之后该做什么。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NextStep {
    /// 投递成功，置 `DONE`。
    Complete,
    /// 还有重试机会：置回 `PENDING`，并把 `available_at` 推后 `delay`。
    Retry { attempts: u32, delay: Duration },
    /// 重试机会用尽：置 `DEAD` 并写死信。
    DeadLetter { attempts: u32 },
}

/// 允许的重试次数。等于 [`BACKOFF_SCHEDULE`] 的档数——
/// **每一档都要被用到**，否则表里就有一个永远排不上的取值，那本身是缺陷。
pub const MAX_RETRIES: u32 = BACKOFF_SCHEDULE.len() as u32;

/// 一次投递之后的判定。
///
/// `attempts_before` 是**本次投递之前**已经失败过的次数，首投为 0。
///
/// # 「共 8 次」到底是 8 次尝试还是 8 次重试——本实现的取法与理由
///
/// 卷内两处措辞不一致，这里必须挑明而不是默默挑一个：
/// 基线第 6.2 节逐字「重试退避固定为 …… 共 8 次，全部失败后置为 `DEAD`」，
/// 阶段 6 计划逐字「失败 `attempts + 1` 并按基线第 6.2 节的八档退避重排 `available_at`，
/// 八次全部失败置 `DEAD`」。前者说的是**八次重试**，后者按字面读像是**八次尝试**，
/// 两者差一次。
///
/// 本实现取**八次重试**，即首投加八次重试共九次投递，第九次失败才进死信。
/// 理由是另一条更硬的约束：退避表有 8 档，取「八次尝试」会让最后一档（2 小时）
/// **永远排不上**——一个列在表里却永远用不到的取值本身就是缺陷。
///
/// 这处不一致已如实登记，落码时若使用方另有裁定，改的是本函数一处与其用例，
/// 不影响其余任何代码。
pub fn judge(attempts_before: u32, delivered_ok: bool) -> NextStep {
    if delivered_ok {
        return NextStep::Complete;
    }
    let attempts = attempts_before + 1;
    match BACKOFF_SCHEDULE.get(attempts_before as usize) {
        Some(delay) => NextStep::Retry {
            attempts,
            delay: *delay,
        },
        None => NextStep::DeadLetter { attempts },
    }
}

/// 下一轮轮询该等多久。取件为空即退避到空闲间隔。
pub fn poll_interval(fetched: usize) -> Duration {
    if fetched == 0 {
        POLL_INTERVAL_IDLE
    } else {
        POLL_INTERVAL_BUSY
    }
}

/// 死信状态。基线第 6.2 节逐字四态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeadLetterState {
    Open,
    Repairing,
    Repaired,
    Discarded,
}

impl DeadLetterState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            DeadLetterState::Open => "OPEN",
            DeadLetterState::Repairing => "REPAIRING",
            DeadLetterState::Repaired => "REPAIRED",
            DeadLetterState::Discarded => "DISCARDED",
        }
    }

    /// 死信是否已了结。关账受理的两项前提要枚举**未了结**的死信
    /// （基线第 6.2 节「死信按 `legal_entity_id` 与 `posting_date` 可枚举，
    /// 直接支撑规格第 10.2 章关账受理的两项前提判定」）。
    ///
    /// `Discarded` 算了结，但它要求双人审批——那一层在流程侧，不在这里。
    pub fn is_settled(self) -> bool {
        matches!(self, DeadLetterState::Repaired | DeadLetterState::Discarded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_completes_regardless_of_attempts() {
        assert_eq!(judge(0, true), NextStep::Complete);
        assert_eq!(judge(7, true), NextStep::Complete);
        // 已经用尽重试的那一次若成功，仍然是成功——不能因为次数到了就判死。
        assert_eq!(judge(99, true), NextStep::Complete);
    }

    /// 八档全部排得上，且顺序与基线逐字一致。
    /// 这条守的是「列在表里却永远用不到的取值」那种缺陷。
    #[test]
    fn every_backoff_tier_is_reachable() {
        let mut seen = Vec::new();
        for before in 0..MAX_RETRIES {
            match judge(before, false) {
                NextStep::Retry { attempts, delay } => {
                    assert_eq!(attempts, before + 1);
                    seen.push(delay);
                }
                other => panic!("第 {before} 次失败后不该是 {other:?}"),
            }
        }
        assert_eq!(
            seen,
            BACKOFF_SCHEDULE.to_vec(),
            "八档必须全部排得上且顺序一致"
        );
    }

    /// 边界两侧各一条：第八次失败仍有退避，第九次失败进死信。
    #[test]
    fn dead_letter_only_after_the_eighth_retry() {
        match judge(7, false) {
            NextStep::Retry { attempts, delay } => {
                assert_eq!(attempts, 8);
                assert_eq!(delay, Duration::from_secs(2 * 60 * 60), "第八档是 2 小时");
            }
            other => panic!("第八次失败应仍可重试，实为 {other:?}"),
        }
        match judge(8, false) {
            NextStep::DeadLetter { attempts } => assert_eq!(attempts, 9),
            other => panic!("第九次失败应进死信，实为 {other:?}"),
        }
    }

    #[test]
    fn backoff_values_match_the_baseline_verbatim() {
        let want = [1, 5, 30, 120, 600, 1800, 3600, 7200];
        let got: Vec<u64> = BACKOFF_SCHEDULE.iter().map(Duration::as_secs).collect();
        assert_eq!(got, want, "取值出处是基线第 6.2 节，改它必须先改基线");
    }

    #[test]
    fn poll_backs_off_when_idle() {
        assert_eq!(poll_interval(0), POLL_INTERVAL_IDLE);
        assert_eq!(poll_interval(1), POLL_INTERVAL_BUSY);
        assert_eq!(poll_interval(FETCH_BATCH_LIMIT), POLL_INTERVAL_BUSY);
    }

    #[test]
    fn status_and_dead_letter_db_values() {
        assert_eq!(OutboxStatus::Pending.as_db_value(), "PENDING");
        assert_eq!(OutboxStatus::Dispatching.as_db_value(), "DISPATCHING");
        assert_eq!(OutboxStatus::Done.as_db_value(), "DONE");
        assert_eq!(OutboxStatus::Dead.as_db_value(), "DEAD");
        assert_eq!(DeadLetterState::Open.as_db_value(), "OPEN");
        assert_eq!(DeadLetterState::Discarded.as_db_value(), "DISCARDED");
    }

    /// 未了结的两态要能被枚举出来——关账受理的前提判定挂在这上面。
    /// 把 `Open` 或 `Repairing` 误判成已了结，会让一个还有未修复死信的法人
    /// 通过关账受理，那是账务上的实质错误，不是统计口径问题。
    #[test]
    fn only_repaired_and_discarded_count_as_settled() {
        assert!(!DeadLetterState::Open.is_settled());
        assert!(!DeadLetterState::Repairing.is_settled());
        assert!(DeadLetterState::Repaired.is_settled());
        assert!(DeadLetterState::Discarded.is_settled());
    }
}
