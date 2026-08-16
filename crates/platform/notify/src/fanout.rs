//! 扇出规模与未读上限的判定。
//!
//! 两条判定各守一个「写反了就丢通知」的地方，取值出处是阶段 3 计划第 3.4.5 节。

/// 同步扇出的人数上限。超过它改走 Outbox 由 job-worker 分批扇出。
/// 计划第 3.4.5 节：配置键 `notify.sync_fanout_max`，默认 200。
pub const DEFAULT_SYNC_FANOUT_MAX: usize = 200;

/// 单用户未读上限。计划第 3.4.5 节：配置键 `unread_cap_per_user`，默认 2000。
///
/// 该取值与保留期 180 天同属 PRD 附录乙 U-K-04 的待决项，本阶段是**临时取值**、
/// 标注为待产品负责人决策；切换代价只是改配置，不涉及结构变更。
pub const DEFAULT_UNREAD_CAP_PER_USER: u32 = 2000;

/// 扇出该怎么走。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FanoutPlan {
    /// 在业务事务内同步写入全部接收人的通知。
    Synchronous,
    /// 写一条 Outbox 事件，由 job-worker 分批扇出。
    ViaOutbox,
}

/// 按接收人数决定扇出方式。
///
/// **边界取「不超过即同步」**：恰好等于阈值时仍走同步。阈值的语义是
/// 「同步最多做这么多」，不是「做到这个数就必须转异步」。
/// 写成 `>=` 会让默认 200 实际只同步到 199——一个没人写在文档里的差一。
pub fn plan_fanout(recipient_count: usize, sync_fanout_max: usize) -> FanoutPlan {
    if recipient_count <= sync_fanout_max {
        FanoutPlan::Synchronous
    } else {
        FanoutPlan::ViaOutbox
    }
}

/// 未读上限触发时该做什么。
///
/// **注意这里没有「拒绝写入」这一支，且这是有意的。**
/// 计划第 3.4.5 节逐字：「单用户未读数达到 `unread_cap_per_user` 时，
/// **新通知仍写入**，同时把该用户最旧的已超过保留期的已读通知纳入下一轮清理，
/// 并写一条 `WARN` 级运行日志。不丢新通知。」
///
/// 这一条最容易被实现成「满了就拒」——那等于用一个容量上限去丢业务提醒，
/// 而站内通知是规格第 5.1 章定的**首版唯一验收不可豁免的通知渠道**。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnreadCapOutcome {
    /// 新通知一律写入。这个字段恒为真，留着是为了让「要不要写」这件事
    /// 在类型上被看见，而不是靠读者记得它没有反面。
    pub write_anyway: bool,
    /// 是否触发一轮已读通知的清理。
    pub schedule_cleanup: bool,
    /// 是否记一条 WARN 级运行日志。
    pub warn: bool,
}

/// 判定未读上限。
pub fn judge_unread_cap(current_unread: u32, cap: u32) -> UnreadCapOutcome {
    let over = current_unread >= cap;
    UnreadCapOutcome {
        write_anyway: true,
        schedule_cleanup: over,
        warn: over,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 边界：恰好等于阈值走同步，超过一个才转异步。
    #[test]
    fn fanout_boundary_is_inclusive() {
        assert_eq!(plan_fanout(199, 200), FanoutPlan::Synchronous);
        assert_eq!(plan_fanout(200, 200), FanoutPlan::Synchronous);
        assert_eq!(plan_fanout(201, 200), FanoutPlan::ViaOutbox);
    }

    /// 首版命名用户 50，最大扇出是许可宽限期告警的 50 行——远在阈值内。
    /// 这条不是废话：它把「首版根本不会走异步扇出」这个事实钉在用例里，
    /// 日后有人把阈值调到 20 时会立刻看见这一条红。
    #[test]
    fn first_release_scale_stays_synchronous() {
        assert_eq!(
            plan_fanout(50, DEFAULT_SYNC_FANOUT_MAX),
            FanoutPlan::Synchronous
        );
    }

    /// 未读满了也要写——这是本模块最要紧的一条。
    /// 站内通知是首版唯一验收不可豁免的通知渠道，用容量上限丢它是不行的。
    #[test]
    fn new_notice_is_written_even_when_the_cap_is_hit() {
        let at_cap = judge_unread_cap(2000, 2000);
        assert!(at_cap.write_anyway, "满了也必须写，不得拒绝");
        assert!(at_cap.schedule_cleanup);
        assert!(at_cap.warn);

        let over_cap = judge_unread_cap(5000, 2000);
        assert!(over_cap.write_anyway, "超了更要写");
    }

    /// 未满时不触发清理、不记 WARN——否则日志里每条通知都带一条警告，
    /// 真正的告警就被淹掉了。
    #[test]
    fn under_the_cap_is_quiet() {
        let o = judge_unread_cap(1999, 2000);
        assert!(o.write_anyway);
        assert!(!o.schedule_cleanup);
        assert!(!o.warn);
    }

    /// 默认取值与计划逐字一致。改它要先改计划。
    #[test]
    fn defaults_match_the_plan() {
        assert_eq!(DEFAULT_SYNC_FANOUT_MAX, 200);
        assert_eq!(DEFAULT_UNREAD_CAP_PER_USER, 2000);
    }
}
