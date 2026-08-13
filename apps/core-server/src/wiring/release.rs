//! 配置发布一侧的装配（阶段 3a 装配位，阶段 4 注入三个 AUTHZ applier）。
//!
//! `ConfigItemApplierRegistry` 的注入落两个 apps 的 wiring 目录
//! （裁定 A-19/H-01）：ep-platform-release 一律不反向依赖任何
//! applier 属主 crate，实现体由属主模块阶段在此注册——
//! 阶段 4 注入 `AuthzRoleApplier`、`AuthzPolicyApplier`、
//! `AuthzFieldGrantApplier`（任务 #23，写入面取 db-pg 的
//! `PgAuthzConfigWriteStore`）；3b-1 注入 `FlowDefinitionApplier`；
//! 3b-2 注入 `NotifyRuleApplier`；其余按 13 计划 §4.6 分派。
//!
//! 未注册实现的 `item_kind` 由发布执行侧整包拒绝，不以假实现顶位。

use ep_adapter_db_pg::PgAuthzConfigWriteStore;
use ep_platform_authz::applier::register_authz_appliers;
use ep_platform_release::ConfigItemApplierRegistry;

/// 构建内容项 applier 注册表：阶段 4 注册三个 AUTHZ 类 applier，
/// 写入面经同一 `AuthzConfigWriteStore` 实现体在同事务内完成
/// 配置写入与版本推进。
pub fn config_item_applier_registry() -> Result<ConfigItemApplierRegistry, String> {
    let mut registry = ConfigItemApplierRegistry::new();
    let store = std::sync::Arc::new(PgAuthzConfigWriteStore::new());
    register_authz_appliers(&mut registry, store).map_err(|e| format!("applier 注册失败：{e}"))?;
    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_release::ItemKind;

    /// 阶段 4 装配位：三个 AUTHZ 类有实现，其余十二项查无实现，
    /// 发布执行侧据此整包拒绝（错误码归发布执行阶段登记）。
    #[test]
    fn only_the_three_authz_kinds_are_registered() {
        let registry = config_item_applier_registry().expect("注册可完成");
        for kind in ["AUTHZ_ROLE", "AUTHZ_POLICY", "AUTHZ_FIELD_GRANT"] {
            let item = ItemKind::ALL
                .iter()
                .copied()
                .find(|k| k.as_str() == kind)
                .expect("AUTHZ 类 item_kind 已在发布端口登记");
            assert!(registry.lookup(item).is_some(), "{kind} 应已注册");
        }
        let registered = ["AUTHZ_ROLE", "AUTHZ_POLICY", "AUTHZ_FIELD_GRANT"];
        for kind in ItemKind::ALL {
            if !registered.contains(&kind.as_str()) {
                assert!(
                    registry.lookup(kind).is_none(),
                    "{} 不应有实现",
                    kind.as_str()
                );
            }
        }
    }
}
