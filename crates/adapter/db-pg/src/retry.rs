//! 未来事务重试策略的纯判定器。出处是 02 计划第 4.7 节：序列化失败 40001
//! 与死锁 40P01 使用 50、150、450 毫秒退避。当前尚无类型化幂等证明，
//! [`crate::tx::PgUnitOfWork`] 的公共工厂入口不会调用本判定器并始终单次
//! 失败关闭；这里保留的 SQLSTATE、次数和遗留 side-effect marker 测试只
//! 冻结未来策略形态，marker 未置位本身不是重试授权。
//!
//! EP__DB__RETRY__MAX_ATTEMPTS 与 EP__DB__RETRY__BACKOFF_MS 两键按
//! config-reference 的登记在进程启动时构造策略，修改后须重启对应服务。

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

    /// 从配置段取值构造。当前尚无签名策略代，只有冻结的三次退避序列
    /// 可被接受；任何漂移都失败关闭，绝不截断、补齐或数值钳制。
    pub fn try_from_config(max_attempts: u8, backoff_ms: &[u32]) -> Result<Self, &'static str> {
        if max_attempts != 3 || backoff_ms != [50, 150, 450] {
            return Err("db.retry 必须精确为 max_attempts=3、backoff_ms=[50,150,450]");
        }
        Ok(Self::standard())
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
    fn config_constructor_rejects_instead_of_coercing_non_frozen_values() {
        for (max_attempts, backoff_ms) in [
            (0, vec![50, 150, 450]),
            (255, vec![50, 150, 450]),
            (3, vec![]),
            (3, vec![50, 150]),
            (3, vec![50, 150, 450, 900]),
            (3, vec![50, 150, 65_536]),
        ] {
            assert!(
                RetryPolicy::try_from_config(max_attempts, &backoff_ms).is_err(),
                "必须拒绝 max_attempts={max_attempts}, backoff_ms={backoff_ms:?}"
            );
        }
        assert_eq!(
            RetryPolicy::try_from_config(3, &[50, 150, 450]),
            Ok(RetryPolicy::standard())
        );
    }
}
