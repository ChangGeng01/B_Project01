//! 事务重试策略。出处是 02 计划第 4.7 节：序列化失败 40001 与死锁 40P01
//! 在数据访问层重试，退避 50、150、450 毫秒；只对尚未产生任何外部可见
//! 副作用的事务重试，判定依据是 `PgTx` 上的 side_effect_marker 标志位。
//!
//! EP__DB__RETRY__MAX_ATTEMPTS 与 EP__DB__RETRY__BACKOFF_MS 两键按
//! config-reference 的登记为 SIGHUP 热生效：装配侧在热加载时重建
//! [`RetryPolicy`]，不在旧策略上打补丁。

use std::time::Duration;

use crate::conn::DbErrorClass;

/// 重试策略（C-04 冻结形态）：尝试上限、三段退避、可重试 SQLSTATE 表。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RetryPolicy {
    /// 重试次数上限（不含首次执行）。标准取值 3。
    pub max_attempts: u8,
    /// 逐次重试前的退避毫秒数，与 max_attempts 一一对应。
    pub backoff_ms: [u16; 3],
    /// 可重试的 SQLSTATE。标准取值 40001 与 40P01。
    pub retryable_sqlstates: [&'static str; 2],
}

impl RetryPolicy {
    /// 标准策略：重试 3 次、退避 50/150/450 毫秒、40001 与 40P01。
    pub const fn standard() -> Self {
        Self {
            max_attempts: 3,
            backoff_ms: [50, 150, 450],
            retryable_sqlstates: ["40001", "40P01"],
        }
    }

    /// 从配置段取值构造。backoff 取值不足或超过三段时截断/补齐到三段，
    /// 保证与 max_attempts 的形态一致。
    pub fn from_config(max_attempts: u8, backoff_ms: &[u32]) -> Self {
        let mut arr = [50u16, 150, 450];
        for (i, slot) in arr.iter_mut().enumerate() {
            if let Some(v) = backoff_ms.get(i) {
                *slot = (*v).min(u32::from(u16::MAX)) as u16;
            }
        }
        Self {
            max_attempts,
            backoff_ms: arr,
            retryable_sqlstates: ["40001", "40P01"],
        }
    }

    pub fn is_retryable_sqlstate(&self, sqlstate: Option<&str>) -> bool {
        sqlstate.is_some_and(|s| self.retryable_sqlstates.contains(&s))
    }

    /// 第 `retry_index` 次重试（0 起）前的退避。越界取最后一段。
    pub fn backoff(&self, retry_index: usize) -> Duration {
        let idx = retry_index.min(self.backoff_ms.len() - 1);
        Duration::from_millis(u64::from(self.backoff_ms[idx]))
    }
}

/// 一次失败后的处置结论。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RetryDecision {
    /// 等待指定退避时长后重试。
    Retry(Duration),
    /// 可重试但已用尽次数：返回 SERIALIZATION_RETRY_EXHAUSTED。
    Exhausted,
    /// 不属于可重试错误：原样返回，不重试。
    NotRetryable,
    /// side_effect_marker 已置位：事务已产生外部可见副作用，
    /// 一律不重试，直接返回 SERIALIZATION_RETRY_EXHAUSTED。
    SideEffectMarked,
}

/// 判定一次失败该不该重试。
///
/// `failures_so_far` 是包含本次在内的已失败次数。标准策略下三次重试
/// 对应失败计数 1、2、3 时退避重试，4 时用尽。
pub fn decide_retry(
    policy: &RetryPolicy,
    class: DbErrorClass,
    sqlstate: Option<&str>,
    side_effect_marked: bool,
    failures_so_far: usize,
) -> RetryDecision {
    if class != DbErrorClass::Retryable || !policy.is_retryable_sqlstate(sqlstate) {
        return RetryDecision::NotRetryable;
    }
    if side_effect_marked {
        return RetryDecision::SideEffectMarked;
    }
    if failures_so_far <= usize::from(policy.max_attempts) {
        RetryDecision::Retry(policy.backoff(failures_so_far - 1))
    } else {
        RetryDecision::Exhausted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conn::SQLSTATE_DEADLOCK_DETECTED;
    use crate::conn::SQLSTATE_FOREIGN_KEY_VIOLATION;
    use crate::conn::SQLSTATE_SERIALIZATION_FAILURE;

    #[test]
    fn standard_policy_matches_the_ruling() {
        let p = RetryPolicy::standard();
        assert_eq!(p.max_attempts, 3);
        assert_eq!(p.backoff_ms, [50, 150, 450]);
        assert_eq!(p.retryable_sqlstates, ["40001", "40P01"]);
        assert_eq!(p.backoff(0), Duration::from_millis(50));
        assert_eq!(p.backoff(1), Duration::from_millis(150));
        assert_eq!(p.backoff(2), Duration::from_millis(450));
        assert_eq!(p.backoff(9), Duration::from_millis(450), "越界取最后一段");
    }

    #[test]
    fn only_40001_and_40p01_are_retryable() {
        let p = RetryPolicy::standard();
        assert_eq!(
            decide_retry(
                &p,
                DbErrorClass::Retryable,
                Some(SQLSTATE_SERIALIZATION_FAILURE),
                false,
                1
            ),
            RetryDecision::Retry(Duration::from_millis(50))
        );
        assert_eq!(
            decide_retry(
                &p,
                DbErrorClass::Retryable,
                Some(SQLSTATE_DEADLOCK_DETECTED),
                false,
                1
            ),
            RetryDecision::Retry(Duration::from_millis(50))
        );
        assert_eq!(
            decide_retry(
                &p,
                DbErrorClass::ReferencedRowMissing,
                Some(SQLSTATE_FOREIGN_KEY_VIOLATION),
                false,
                1
            ),
            RetryDecision::NotRetryable,
            "23503 直接返回不重试"
        );
        assert_eq!(
            decide_retry(&p, DbErrorClass::Other, None, false, 1),
            RetryDecision::NotRetryable
        );
    }

    #[test]
    fn side_effect_marker_disables_retry_unconditionally() {
        let p = RetryPolicy::standard();
        assert_eq!(
            decide_retry(
                &p,
                DbErrorClass::Retryable,
                Some(SQLSTATE_SERIALIZATION_FAILURE),
                true,
                1
            ),
            RetryDecision::SideEffectMarked,
            "置位后不重试"
        );
    }

    #[test]
    fn retries_exhaust_after_max_attempts() {
        let p = RetryPolicy::standard();
        let s = Some(SQLSTATE_SERIALIZATION_FAILURE);
        assert_eq!(
            decide_retry(&p, DbErrorClass::Retryable, s, false, 1),
            RetryDecision::Retry(Duration::from_millis(50))
        );
        assert_eq!(
            decide_retry(&p, DbErrorClass::Retryable, s, false, 2),
            RetryDecision::Retry(Duration::from_millis(150))
        );
        assert_eq!(
            decide_retry(&p, DbErrorClass::Retryable, s, false, 3),
            RetryDecision::Retry(Duration::from_millis(450))
        );
        assert_eq!(
            decide_retry(&p, DbErrorClass::Retryable, s, false, 4),
            RetryDecision::Exhausted,
            "首次加三次重试共四次执行后用尽"
        );
    }

    #[test]
    fn from_config_takes_the_first_three_backoff_values() {
        let p = RetryPolicy::from_config(3, &[10, 20]);
        assert_eq!(p.backoff_ms, [10, 20, 450], "不足三段保留标准第三段");
        let p = RetryPolicy::from_config(3, &[10, 20, 30, 40]);
        assert_eq!(p.backoff_ms, [10, 20, 30], "超过三段截断");
    }
}
