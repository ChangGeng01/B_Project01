//! 移动推送出口的判定。
//!
//! 计划第 3.4.6 节的第一句就把这一层的性质定死了：
//! **推送是站内通知之上的可选增强，不是任何提醒的保证渠道**
//! （规格第 5.1 章与 PRD 第 10.5.1 节都明确）。
//! 因此推送链路的任何失败**一律不产生用户可见错误**，只记录送达状态。
//!
//! 这一条最容易被实现反：把推送失败冒泡成业务错误，会让一个已经成功的业务动作
//! 因为推送不通而报错——而站内通知早已写好，用户其实收得到。

/// 一次推送投递的结局。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
}

impl DeliveryStatus {
    pub fn as_db_value(self) -> &'static str {
        match self {
            DeliveryStatus::Pending => "PENDING",
            DeliveryStatus::Delivered => "DELIVERED",
            DeliveryStatus::Failed => "FAILED",
        }
    }
}

/// 推送失败之后该做什么。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PushFailureOutcome {
    /// 是否向调用方冒泡成错误。**恒为假。**
    ///
    /// 留这个字段而不是干脆不写，是为了让「推送失败不冒泡」这条纪律
    /// 在类型上被看见并被断言，而不是靠读代码的人记得它没有反面。
    pub surface_to_caller: bool,
    /// 记录的送达状态。
    pub status: DeliveryStatus,
    /// 是否把该推送注册置为失效。
    pub deactivate_registration: bool,
}

/// 推送失败的判定。
///
/// `consecutive_failures` 是**含本次**的连续失败次数，
/// `threshold` 是置为失效的阈值（计划第 3.4.6 节：连续失败达阈值即
/// `is_active = false`，理由是失效令牌会持续消耗出网重试预算）。
pub fn judge_failure(consecutive_failures: u32, threshold: u32) -> PushFailureOutcome {
    PushFailureOutcome {
        surface_to_caller: false,
        status: DeliveryStatus::Failed,
        deactivate_registration: consecutive_failures >= threshold,
    }
}

/// 推送整体不可用时，业务提醒是否中断。
///
/// **恒为假**：站内通知已在业务事务内同步写好，推送不可用只是少了一路增强。
/// 计划第 3.1 节交付物第 7 项逐字「推送不可用时只剩站内通知，业务提醒不中断」。
pub fn business_reminder_interrupted_when_push_unavailable() -> bool {
    false
}

/// 是否该为某接收人发起推送。
///
/// 两个条件同时成立才发（计划第 3.4.6 节链路第一步）：
/// 该接收人存在**活跃**的推送注册，且 `notify.push_enabled` 为真。
pub fn should_push(has_active_registration: bool, push_enabled: bool) -> bool {
    has_active_registration && push_enabled
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 本模块最要紧的一条：推送失败**永远不冒泡**。
    /// 冒泡会让一个已经成功的业务动作因为推送不通而报错，
    /// 而站内通知早已写好，用户其实收得到。
    #[test]
    fn push_failure_never_surfaces_to_the_caller() {
        for n in [1u32, 5, 100] {
            assert!(
                !judge_failure(n, 5).surface_to_caller,
                "连续失败 {n} 次也不得冒泡"
            );
        }
    }

    #[test]
    fn failure_is_recorded_as_failed() {
        assert_eq!(judge_failure(1, 5).status, DeliveryStatus::Failed);
    }

    /// 边界：达到阈值即停用，未达到不停用。
    /// 停早了会让偶发网络抖动把一台正常设备踢掉；停晚了会持续烧出网重试预算。
    #[test]
    fn deactivation_happens_exactly_at_the_threshold() {
        assert!(!judge_failure(4, 5).deactivate_registration);
        assert!(judge_failure(5, 5).deactivate_registration);
        assert!(judge_failure(6, 5).deactivate_registration);
    }

    #[test]
    fn push_needs_both_an_active_registration_and_the_switch() {
        assert!(should_push(true, true));
        assert!(!should_push(false, true));
        assert!(!should_push(true, false));
        assert!(!should_push(false, false));
    }

    /// 推送整体不可用不中断业务提醒——站内通知是同步写的，不依赖推送。
    #[test]
    fn business_reminders_survive_a_dead_push_channel() {
        assert!(!business_reminder_interrupted_when_push_unavailable());
    }

    #[test]
    fn delivery_status_db_values() {
        assert_eq!(DeliveryStatus::Pending.as_db_value(), "PENDING");
        assert_eq!(DeliveryStatus::Delivered.as_db_value(), "DELIVERED");
        assert_eq!(DeliveryStatus::Failed.as_db_value(), "FAILED");
    }
}
