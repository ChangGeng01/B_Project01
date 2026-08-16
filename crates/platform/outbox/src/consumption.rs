//! 消费端去重判定。
//!
//! 投递语义是**至少一次**（基线第 6.2 节）——同一条事件会被投多次，
//! 这不是缺陷，是设计。去重靠 `platform_msg.inbox_consumptions(consumer, event_id)`
//! 的唯一约束，且**消费副作用与该行插入同事务**：两者分开写，就会出现
//! 「副作用做了、去重行没写」的窗口，重投时副作用做第二遍。
//!
//! 本模块只做判定，不碰数据库。

/// 一次消费尝试的判定结果。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConsumeDecision {
    /// 首次见到，执行副作用并在同一事务插入去重行。
    Proceed,
    /// 已消费过，直接确认，不再执行副作用。
    AlreadyConsumed,
}

/// 判定是否该执行副作用。
///
/// `existing` 是 `(consumer, event_id)` 在 `inbox_consumptions` 里是否已有行。
///
/// 这个函数短到看着多余，但它存在有两个理由：一是给「至少一次加唯一约束」
/// 这条语义一个可单测的落点；二是让调用方**没法把判定写在别处**——
/// 判定散落到各消费者里，就会有人忘了写去重行。
pub fn judge(existing: bool) -> ConsumeDecision {
    if existing {
        ConsumeDecision::AlreadyConsumed
    } else {
        ConsumeDecision::Proceed
    }
}

/// 消费端的唯一键。取值与 `platform_msg.inbox_consumptions` 的唯一约束逐字一致。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ConsumptionKey {
    /// 消费者标识，形如 `platform.impact_assess`。一个事件被多个消费者消费时，
    /// 各自独立去重——**不能只按 `event_id` 去重**，那会让第二个消费者
    /// 因为第一个消费过就被跳过。
    pub consumer: String,
    pub event_id: String,
}

impl ConsumptionKey {
    pub fn new(consumer: impl Into<String>, event_id: impl Into<String>) -> Self {
        Self {
            consumer: consumer.into(),
            event_id: event_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sight_proceeds_and_repeat_does_not() {
        assert_eq!(judge(false), ConsumeDecision::Proceed);
        assert_eq!(judge(true), ConsumeDecision::AlreadyConsumed);
    }

    /// 同一事件、不同消费者，是两把不同的键。
    /// 只按 event_id 去重会让第二个消费者被静默跳过——那是丢消费，不是去重。
    #[test]
    fn different_consumers_of_one_event_are_distinct_keys() {
        let a = ConsumptionKey::new("platform.impact_assess", "evt-1");
        let b = ConsumptionKey::new("platform.notify", "evt-1");
        assert_ne!(a, b);
        assert_eq!(a.event_id, b.event_id, "夹具确实是同一条事件");
    }

    #[test]
    fn same_consumer_same_event_is_one_key() {
        let a = ConsumptionKey::new("platform.notify", "evt-1");
        let b = ConsumptionKey::new("platform.notify", "evt-1");
        assert_eq!(a, b);
    }
}
