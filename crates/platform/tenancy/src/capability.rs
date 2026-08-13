//! 平台管理域九端点的能力常量（A-20）。
//!
//! 端点清单以 02 计划 §5 为准，共九对：四只读端点取 `ActionClass::Read`，
//! 五个 actions/ 端点取 `ActionClass::Submit`。A-09 一行含 open-window 与
//! close-window 两条路由，二者属同一用例（窗口控制的开与合），共用一对常量。
//! 域一律为 `CapabilityDomain::PlatformAdminLowcodeOps`。
//!
//! 命名口径：`<用例全大写>_DOMAIN` 与 `<用例全大写>_ACTION` 成对出现，
//! 路由侧在阶段 2 集成 B 的 wiring 中逐对引用，不得另起字面量。

use ep_foundation::capability::{ActionClass, CapabilityDomain};

/// 端点域：九对常量一律落在平台管理与低代码运维域。
pub const PLATFORM_ADMIN_DOMAIN: CapabilityDomain = CapabilityDomain::PlatformAdminLowcodeOps;

// A-01 GET /api/v1/platform/key-domains —— 密钥域列表（只读）。
pub const KEY_DOMAIN_LIST_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_LIST_ACTION: ActionClass = ActionClass::Read;

// A-02 GET /api/v1/platform/key-domains/{id} —— 密钥域详情（只读）。
pub const KEY_DOMAIN_GET_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_GET_ACTION: ActionClass = ActionClass::Read;

// A-03 POST /api/v1/platform/key-domains/actions/provision —— 开通密钥域。
pub const KEY_DOMAIN_PROVISION_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_PROVISION_ACTION: ActionClass = ActionClass::Submit;

// A-04 POST /api/v1/platform/key-domains/{id}/actions/rotate —— 轮换。
pub const KEY_DOMAIN_ROTATE_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_ROTATE_ACTION: ActionClass = ActionClass::Submit;

// A-05 POST /api/v1/platform/key-domains/{id}/actions/plan-destroy —— 销毁前排程。
pub const KEY_DOMAIN_PLAN_DESTROY_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_PLAN_DESTROY_ACTION: ActionClass = ActionClass::Submit;

// A-06 POST /api/v1/platform/key-domains/{id}/actions/cancel-destroy —— 撤销销毁。
pub const KEY_DOMAIN_CANCEL_DESTROY_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const KEY_DOMAIN_CANCEL_DESTROY_ACTION: ActionClass = ActionClass::Submit;

// A-07 GET /api/v1/platform/sensitive-fields —— 敏感字段清单（只读）。
pub const SENSITIVE_FIELD_LIST_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const SENSITIVE_FIELD_LIST_ACTION: ActionClass = ActionClass::Read;

// A-08 GET /api/v1/platform/migrations —— 迁移历史视图（只读）。
pub const MIGRATION_HISTORY_LIST_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const MIGRATION_HISTORY_LIST_ACTION: ActionClass = ActionClass::Read;

// A-09 POST /api/v1/platform/migrations/actions/open-window 与 close-window
// 两条路由共用一对常量：同一用例「迁移窗口控制」的开与合。
pub const MIGRATION_WINDOW_CONTROL_DOMAIN: CapabilityDomain = PLATFORM_ADMIN_DOMAIN;
pub const MIGRATION_WINDOW_CONTROL_ACTION: ActionClass = ActionClass::Submit;

/// 九对常量的登记清单：（用例名, 域, 动作类别）。
/// 用例数与动作分布由测试穷举断言，增删端点必须同步改本表与测试。
pub const CAPABILITY_REGISTRY: [(&str, CapabilityDomain, ActionClass); 9] = [
    (
        "KEY_DOMAIN_LIST",
        KEY_DOMAIN_LIST_DOMAIN,
        KEY_DOMAIN_LIST_ACTION,
    ),
    (
        "KEY_DOMAIN_GET",
        KEY_DOMAIN_GET_DOMAIN,
        KEY_DOMAIN_GET_ACTION,
    ),
    (
        "KEY_DOMAIN_PROVISION",
        KEY_DOMAIN_PROVISION_DOMAIN,
        KEY_DOMAIN_PROVISION_ACTION,
    ),
    (
        "KEY_DOMAIN_ROTATE",
        KEY_DOMAIN_ROTATE_DOMAIN,
        KEY_DOMAIN_ROTATE_ACTION,
    ),
    (
        "KEY_DOMAIN_PLAN_DESTROY",
        KEY_DOMAIN_PLAN_DESTROY_DOMAIN,
        KEY_DOMAIN_PLAN_DESTROY_ACTION,
    ),
    (
        "KEY_DOMAIN_CANCEL_DESTROY",
        KEY_DOMAIN_CANCEL_DESTROY_DOMAIN,
        KEY_DOMAIN_CANCEL_DESTROY_ACTION,
    ),
    (
        "SENSITIVE_FIELD_LIST",
        SENSITIVE_FIELD_LIST_DOMAIN,
        SENSITIVE_FIELD_LIST_ACTION,
    ),
    (
        "MIGRATION_HISTORY_LIST",
        MIGRATION_HISTORY_LIST_DOMAIN,
        MIGRATION_HISTORY_LIST_ACTION,
    ),
    (
        "MIGRATION_WINDOW_CONTROL",
        MIGRATION_WINDOW_CONTROL_DOMAIN,
        MIGRATION_WINDOW_CONTROL_ACTION,
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_pairs_all_in_platform_admin_domain() {
        assert_eq!(CAPABILITY_REGISTRY.len(), 9, "九对常量不多不少");
        for (name, domain, _) in CAPABILITY_REGISTRY {
            assert_eq!(
                domain,
                CapabilityDomain::PlatformAdminLowcodeOps,
                "用例 {name} 的域必须是平台管理与低代码运维域"
            );
        }
    }

    #[test]
    fn four_reads_and_five_submits() {
        let reads = CAPABILITY_REGISTRY
            .iter()
            .filter(|(_, _, a)| *a == ActionClass::Read)
            .count();
        let submits = CAPABILITY_REGISTRY
            .iter()
            .filter(|(_, _, a)| *a == ActionClass::Submit)
            .count();
        assert_eq!(reads, 4, "只读端点四个：A-01、A-02、A-07、A-08");
        assert_eq!(submits, 5, "actions/ 端点五个：A-03 至 A-06 与 A-09");
    }

    #[test]
    fn usecase_names_are_unique_and_screaming() {
        let mut seen = std::collections::BTreeSet::new();
        for (name, _, _) in CAPABILITY_REGISTRY {
            assert!(seen.insert(name), "用例名 {name} 重复");
            assert!(
                name.bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'),
                "用例名 {name} 必须全大写下划线形态"
            );
        }
    }
}
