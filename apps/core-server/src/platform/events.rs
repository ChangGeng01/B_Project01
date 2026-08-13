//! 阶段 2 平台事件类型的登记与临时记录点。
//!
//! D-13 的三个事件登记在 `docs/event-catalog.md` 的 platform 段。本阶段
//! 不实现 Outbox（02 计划第 6 节明写只交付同 `Tx` 接缝），事件真正写出
//! 属阶段 3b；接缝就位之前，三个写端点的成功路径经 [`record_pending_emit`]
//! 以 Info 级结构化日志记下本应发出的事件类型，使遗漏在日志里可见，
//! 也让 `xtask eventcatalog` 的代码侧字面量扫描有真实被测输入。

use ep_platform_obs::log::{Level, LogFields};

use super::PlatformState;

/// A-03 密钥域建成。聚合类型 `platform.key_domains`。
pub const KEY_DOMAIN_PROVISIONED: &str = "platform.key_domain.provisioned.v1";
/// A-04 数据密钥轮换完成。聚合类型 `platform.key_domains`。
pub const KEY_DOMAIN_ROTATED: &str = "platform.key_domain.rotated.v1";
/// A-09 迁移窗口打开。聚合类型 `platform.migration_windows`。
pub const MIGRATION_WINDOW_OPENED: &str = "platform.migration_window.opened.v1";

// 阶段 4 任务 #23 登记（事件目录五事件中的三个未点名事件）。
// 04 计划正文只点名 platform.user_account.deactivated.v1（任务 #21
// 已登记，字面量在 ep-platform-identity lifecycle.rs）与
// platform.authz_policy.published.v1（由 3b 的 activate 路由发出，
// 不属本阶段登记面）；其余三个按基线第 6.1 节四段命名
// `<module>.<aggregate>.<past_participle>.v<major>` 派生，模块段取
// platform，聚合段取表名单数 snake 形态，动作段取已完成时态。
// 派生依据逐条：锁定取 04 计划登录算法与 U-B-14 锁定策略；调岗取
// §5.4 transfer 用例（PRD 10.2.3 调岗行，会话即时撤销）；应急
// 关闭取 §8.2 退出条件 14（关闭与凭据轮换同事务）。

/// 账号锁定：登录失败达阈值，锁定与窗口计数落库后。
/// 聚合类型 `platform.account_lockouts`。派生依据：U-B-14。
pub const USER_ACCOUNT_LOCKED: &str = "platform.user_account.locked.v1";
/// 账号调岗：transfer 端点成功，授权集合更新且全部会话撤销后。
/// 聚合类型 `platform.user_accounts`。派生依据：04 计划 §5.4。
pub const USER_ACCOUNT_TRANSFERRED: &str = "platform.user_account.transferred.v1";
/// 应急账号关闭：关闭端点成功且凭据轮换完成后。
/// 聚合类型 `platform.breakglass_activations`。派生依据：退出条件 14。
pub const BREAKGLASS_ACTIVATION_CLOSED: &str = "platform.breakglass_activation.closed.v1";

/// 本阶段登记的全部事件类型，与事件目录 platform 段逐条对应。
/// 当前唯一取用者是同文件的命名形态测试；阶段 3b 接 Outbox 时
/// 改由发出侧取用，届时去掉测试限定。
#[cfg(test)]
pub const ALL: [&str; 3] = [
    KEY_DOMAIN_PROVISIONED,
    KEY_DOMAIN_ROTATED,
    MIGRATION_WINDOW_OPENED,
];

/// 阶段 4 派生登记的三个事件（任务 #23）：写出同样属阶段 3b 的
/// Outbox 接缝，接缝就位前用例成功路径经 [`record_pending_emit`]
/// 登记发生，同阶段 2 纪律。
#[cfg(test)]
pub const STAGE4_DERIVED: [&str; 3] = [
    USER_ACCOUNT_LOCKED,
    USER_ACCOUNT_TRANSFERRED,
    BREAKGLASS_ACTIVATION_CLOSED,
];

/// Outbox 接缝就位前的临时记录：把本应发出的事件类型与其主体写进日志。
///
/// `subject` 取事件聚合的标识（密钥域或迁移窗口的 id）。阶段 3b 接入
/// Outbox 后，本函数整体替换为同事务写出，调用点不变。
pub fn record_pending_emit(state: &PlatformState, event_type: &'static str, subject: &str) {
    state.system.logger().log(
        Level::Info,
        LogFields::msg(
            "platform-event",
            format!("事件 {event_type} 主体 {subject}：Outbox 接缝属阶段 3b，本阶段仅登记发生"),
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 命名必须合事件目录第 1 节的四段形态，模块段取 platform。
    #[test]
    fn every_registered_event_has_four_segments_under_platform() {
        for e in ALL.iter().chain(STAGE4_DERIVED.iter()) {
            let parts: Vec<&str> = e.split('.').collect();
            assert_eq!(parts.len(), 4, "{e} 应为四段");
            assert_eq!(parts[0], "platform", "{e} 模块段应为 platform");
            assert!(
                parts[3].starts_with('v') && parts[3][1..].chars().all(|c| c.is_ascii_digit()),
                "{e} 版本段应为 v<主版本号>"
            );
        }
    }
}
