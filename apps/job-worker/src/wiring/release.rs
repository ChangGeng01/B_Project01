//! 配置发布一侧的装配（阶段 3a 装配位）。
//!
//! 发布执行归本进程（03 计划 §3.4.12）：一个 READ COMMITTED 事务内
//! 按 `sort_no` 升序逐项 `validate`/`apply`，回退逆序 `revert`。
//! `ConfigItemApplierRegistry` 的注入落两个 apps 的 wiring 目录
//! （裁定 A-19/H-01）：ep-platform-release 一律不反向依赖任何
//! applier 属主 crate，实现体由属主模块阶段在此注册——
//! 阶段 4 注入 `AuthzRoleApplier`、`AuthzPolicyApplier`、
//! `AuthzFieldGrantApplier`；3b-1 注入 `FlowDefinitionApplier`；
//! 3b-2 注入 `NotifyRuleApplier`；其余按 13 计划 §4.6 分派。
//!
//! 3a 段交付的是真实类型的空注册表骨架：不注册任何 applier，
//! 查不到实现的 `item_kind` 由发布执行侧整包拒绝，不以假实现顶位。

use ep_platform_release::ConfigItemApplierRegistry;

/// 构建内容项 applier 注册表。本阶段为空注册表骨架，
/// applier 实现随属主模块阶段在本函数内逐批注册。
pub fn config_item_applier_registry() -> ConfigItemApplierRegistry {
    ConfigItemApplierRegistry::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_release::ItemKind;

    /// 3a 段装配位是空注册表：十五项一律查无实现，
    /// 发布执行侧据此整包拒绝（错误码归发布执行阶段登记）。
    #[test]
    fn the_phase_3a_assembly_slot_is_an_empty_registry() {
        let registry = config_item_applier_registry();
        for kind in ItemKind::ALL {
            assert!(
                registry.lookup(kind).is_none(),
                "{} 不应有实现",
                kind.as_str()
            );
        }
    }
}
