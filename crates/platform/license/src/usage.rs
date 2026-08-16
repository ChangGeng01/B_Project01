//! 命名用户用量的判定。
//!
//! 规格第 3.4 章逐字：「用量**超过**许可上限时**不阻断业务**，
//! 记录超用事件并在管理控制台显著提示。」
//!
//! 本模块只对 `named_user_limit` 一项建模——它是 `platform_core.license_grants`
//! 上唯一存在的上限列。规格第 3.4 章另点了四项统计量（已启用模块、活跃法人数、
//! 已激活命名用户数、已注册设备数），但那四项与「用量上限」单数之间的字段对应关系
//! 全卷未给，也没有对应的上限列。**没有被测对象的判据不写**，登记在 crate 文档。

/// 一次用量判定的结论。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UsageVerdict {
    /// 是否阻断业务。**恒为假。**
    ///
    /// 留这个字段是为了让规格的「不阻断」这条纪律在类型上被看见并被断言，
    /// 而不是只写在注释里。只写注释的话，下一次有人加拦截时不会有任何东西变红。
    pub blocks_business: bool,
    /// 是否记录一条超用事件。
    pub emit_overuse_event: bool,
    /// 是否在管理控制台显著提示。
    pub console_prominent_notice: bool,
}

/// 判定一次命名用户用量。
///
/// 阈值取**严格大于**：规格逐字是「超过许可上限时」，等于上限不算超。
/// 这一处不照抄 `notify` 里未读上限那条 `>=`——那条判的是另一件事，
/// 且它的规格措辞本来就不同。两处同形不同源，抄过来会把边界错一位。
pub fn named_user_usage_verdict(active_named_users: i64, named_user_limit: i64) -> UsageVerdict {
    let over = active_named_users > named_user_limit;
    UsageVerdict {
        blocks_business: false,
        emit_overuse_event: over,
        console_prominent_notice: over,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 用例一律不取 50 这个数。规格与 PRD 都明写许可用量上限是商业计量口径、
    // 与附录 A 的认证规模基线（命名用户 50）不是一回事，两者不得互相引用为对方的依据。
    const LIMIT: i64 = 100;

    /// 超上限记事件、给提示，但**不阻断**。
    #[test]
    fn over_the_limit_reports_but_never_blocks() {
        let v = named_user_usage_verdict(LIMIT + 1, LIMIT);
        assert!(v.emit_overuse_event);
        assert!(v.console_prominent_notice);
        assert!(!v.blocks_business, "规格逐字：用量超过许可上限时不阻断业务");
    }

    /// 边界：**恰好等于上限不算超**。规格的措辞是「超过」，不是「达到」。
    /// 这一条容易照隔壁未读上限那条 `>=` 抄错一位，抄错的后果是
    /// 一家正好用满额度的公司天天收到超用告警。
    #[test]
    fn exactly_at_the_limit_is_not_over() {
        let v = named_user_usage_verdict(LIMIT, LIMIT);
        assert!(!v.emit_overuse_event, "等于上限不是超过");
        assert!(!v.console_prominent_notice);
        assert!(!v.blocks_business);
    }

    /// 未超上限不得报事件——防「恒发超用事件」这种反向恒真。
    #[test]
    fn under_the_limit_is_quiet() {
        let v = named_user_usage_verdict(LIMIT - 1, LIMIT);
        assert!(!v.emit_overuse_event);
        assert!(!v.console_prominent_notice);
    }

    /// 不阻断这条在任何倍率下都成立，包括离谱的超用。
    #[test]
    fn blocking_never_turns_on_no_matter_how_far_over() {
        for active in [LIMIT + 1, LIMIT * 2, LIMIT * 100] {
            assert!(
                !named_user_usage_verdict(active, LIMIT).blocks_business,
                "用量 {active} 也不得阻断"
            );
        }
    }
}
