//! 配置发布的内容项端口（阶段 3a，03 计划 §3.4.12）。
//!
//! 本文件是 3a 段在 ep-platform-release 的唯一交付：trait、
//! `ItemKind` 十五项、`ConfigPackageItem` DTO 与注册表四件套。
//! 3a 段无表、无用例、不依赖身份与授权（退出条件 29 的判据），
//! applier 实现体随属主模块阶段注入：`AuthzRoleApplier` 等归
//! 阶段 4，`FlowDefinitionApplier` 归 3b-1，`NotifyRuleApplier`
//! 归 3b-2（裁定 A-19：release 一律不反向依赖任何 applier 属主
//! crate，注入落两个 apps 的 wiring 目录）。

use std::collections::HashMap;
use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::{SecurityContext, SecurityLevel};

/// 内容项种类。十五项与阶段 13 计划 §3.2.11 逐项一致
/// （裁定 A-19 恢复十五项）。`as_str` 给出 `item_kind` 列的
/// 落库字面量，建表时 CHECK 约束据此同口径。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemKind {
    CustomObject,
    CustomField,
    CustomRelation,
    CustomIndex,
    CustomView,
    UiLayout,
    FlowDefinition,
    AuthzRole,
    AuthzPolicy,
    AuthzFieldGrant,
    ReportDefinition,
    MetricDefinition,
    DashboardDefinition,
    PrintTemplate,
    NotifyRule,
}

impl ItemKind {
    /// 十五项全量，顺序与阶段 13 计划 §3.2.11 一致。
    pub const ALL: [ItemKind; 15] = [
        ItemKind::CustomObject,
        ItemKind::CustomField,
        ItemKind::CustomRelation,
        ItemKind::CustomIndex,
        ItemKind::CustomView,
        ItemKind::UiLayout,
        ItemKind::FlowDefinition,
        ItemKind::AuthzRole,
        ItemKind::AuthzPolicy,
        ItemKind::AuthzFieldGrant,
        ItemKind::ReportDefinition,
        ItemKind::MetricDefinition,
        ItemKind::DashboardDefinition,
        ItemKind::PrintTemplate,
        ItemKind::NotifyRule,
    ];

    /// `platform_meta.config_package_items.item_kind` 列的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            ItemKind::CustomObject => "CUSTOM_OBJECT",
            ItemKind::CustomField => "CUSTOM_FIELD",
            ItemKind::CustomRelation => "CUSTOM_RELATION",
            ItemKind::CustomIndex => "CUSTOM_INDEX",
            ItemKind::CustomView => "CUSTOM_VIEW",
            ItemKind::UiLayout => "UI_LAYOUT",
            ItemKind::FlowDefinition => "FLOW_DEFINITION",
            ItemKind::AuthzRole => "AUTHZ_ROLE",
            ItemKind::AuthzPolicy => "AUTHZ_POLICY",
            ItemKind::AuthzFieldGrant => "AUTHZ_FIELD_GRANT",
            ItemKind::ReportDefinition => "REPORT_DEFINITION",
            ItemKind::MetricDefinition => "METRIC_DEFINITION",
            ItemKind::DashboardDefinition => "DASHBOARD_DEFINITION",
            ItemKind::PrintTemplate => "PRINT_TEMPLATE",
            ItemKind::NotifyRule => "NOTIFY_RULE",
        }
    }
}

/// 内容项的变更形态。与阶段 13 计划 §4.6 的 `ChangeKind` 同名同形，
/// 落库字面量 ADD、MODIFY、REMOVE；规格互斥约束（ADD 时 before 空
/// after 非空、REMOVE 相反、MODIFY 两者均非空）由发布侧 validate 守。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ChangeKind {
    Add,
    Modify,
    Remove,
}

impl ChangeKind {
    /// `platform_meta.config_package_items.change_kind` 列的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            ChangeKind::Add => "ADD",
            ChangeKind::Modify => "MODIFY",
            ChangeKind::Remove => "REMOVE",
        }
    }
}

/// 配置包内容项 DTO。十一字段与阶段 13 计划 §3.2.11 的表列逐项对应；
/// jsonb 列以规范化 JSON 文本承载（键按字典序、无空白、UTF-8），
/// `item_hash` 即 `after_spec` 该形态的 SHA-256 十六进制小写。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigPackageItem {
    /// 内容项 ID（`pk_config_package_items`）。
    pub id: uuid::Uuid,
    /// 密级公共列。
    pub security_level: SecurityLevel,
    /// 所属配置包 ID。
    pub config_package_id: uuid::Uuid,
    /// 内容项种类。
    pub item_kind: ItemKind,
    /// 项码，包内与种类联合唯一。
    pub item_code: String,
    /// 变更形态。
    pub change_kind: ChangeKind,
    /// 适用法人；空数组表示全部法人。
    pub applies_to_legal_entity_ids: Vec<Id<LegalEntity>>,
    /// 变更前规格（jsonb 文本）。ADD 时为空。
    pub before_spec: Option<String>,
    /// 变更后规格（jsonb 文本）。REMOVE 时为空。
    pub after_spec: Option<String>,
    /// `after_spec` 规范化序列化后的 SHA-256 十六进制小写。
    pub item_hash: String,
    /// 发布执行序，升序 apply、逆序 revert。
    pub sort_no: i32,
}

/// 内容项 applier 端口。签名与阶段 13 计划 §4.6 逐字一致，
/// `Tx` 取自 `ep_foundation::port::tx`。发布执行在一个
/// READ COMMITTED 事务内按 `sort_no` 升序逐项 `validate` 后 `apply`；
/// 回退逆序 `revert` 以 `before_spec` 恢复。
pub trait ConfigItemApplier: Send + Sync {
    fn item_kind(&self) -> ItemKind;
    fn validate(&self, item: &ConfigPackageItem, ctx: &SecurityContext) -> Result<(), AppError>;
    fn apply(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        ctx: &SecurityContext,
    ) -> Result<(), AppError>;
    fn revert(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        ctx: &SecurityContext,
    ) -> Result<(), AppError>;
    fn requires_derived_store_rebuild(&self, item: &ConfigPackageItem) -> bool;
}

/// 按 `ItemKind` 注册与查找 applier 的注册表（裁定 A-19/H-01）。
/// 只提供两个方法；装配在两个 apps 的 wiring 目录，3a 段装配的
/// 是空注册表骨架，实现体随属主模块阶段注入。查不到实现的
/// `item_kind` 由发布侧整包拒绝（错误码归发布执行阶段登记）。
#[derive(Default)]
pub struct ConfigItemApplierRegistry {
    appliers: HashMap<ItemKind, Arc<dyn ConfigItemApplier>>,
}

impl ConfigItemApplierRegistry {
    /// 空注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 按 applier 自报的 `item_kind` 注册。同一 `ItemKind` 重复
    /// 注册是装配错误，直接返错拒绝，不静默覆盖。
    pub fn register(&mut self, applier: Arc<dyn ConfigItemApplier>) -> Result<(), AppError> {
        let kind = applier.item_kind();
        if self.appliers.insert(kind, applier).is_some() {
            return Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                format!("内容项 applier 重复注册：{}", kind.as_str()),
            ));
        }
        Ok(())
    }

    /// 按 `ItemKind` 查找已注册的 applier。
    pub fn lookup(&self, kind: ItemKind) -> Option<&dyn ConfigItemApplier> {
        self.appliers.get(&kind).map(|a| a.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 十五项与阶段 13 计划 §3.2.11 逐项一致，字面量无重复。
    #[test]
    fn fifteen_kinds_with_distinct_db_literals() {
        assert_eq!(ItemKind::ALL.len(), 15);
        let expected = [
            "CUSTOM_OBJECT",
            "CUSTOM_FIELD",
            "CUSTOM_RELATION",
            "CUSTOM_INDEX",
            "CUSTOM_VIEW",
            "UI_LAYOUT",
            "FLOW_DEFINITION",
            "AUTHZ_ROLE",
            "AUTHZ_POLICY",
            "AUTHZ_FIELD_GRANT",
            "REPORT_DEFINITION",
            "METRIC_DEFINITION",
            "DASHBOARD_DEFINITION",
            "PRINT_TEMPLATE",
            "NOTIFY_RULE",
        ];
        for (i, kind) in ItemKind::ALL.iter().enumerate() {
            assert_eq!(kind.as_str(), expected[i]);
        }
        // 两两不同（ALL 无重复项）。
        for (i, a) in ItemKind::ALL.iter().enumerate() {
            for b in &ItemKind::ALL[..i] {
                assert_ne!(a, b);
            }
        }
    }

    /// 变更形态三值的落库字面量与阶段 13 计划 §4.6 一致。
    #[test]
    fn change_kind_literals_match_the_thirteen_plan() {
        assert_eq!(ChangeKind::Add.as_str(), "ADD");
        assert_eq!(ChangeKind::Modify.as_str(), "MODIFY");
        assert_eq!(ChangeKind::Remove.as_str(), "REMOVE");
    }

    struct StubApplier {
        kind: ItemKind,
    }

    impl ConfigItemApplier for StubApplier {
        fn item_kind(&self) -> ItemKind {
            self.kind
        }
        fn validate(&self, _: &ConfigPackageItem, _: &SecurityContext) -> Result<(), AppError> {
            Ok(())
        }
        fn apply(
            &self,
            _: &mut dyn Tx,
            _: &ConfigPackageItem,
            _: &SecurityContext,
        ) -> Result<(), AppError> {
            Ok(())
        }
        fn revert(
            &self,
            _: &mut dyn Tx,
            _: &ConfigPackageItem,
            _: &SecurityContext,
        ) -> Result<(), AppError> {
            Ok(())
        }
        fn requires_derived_store_rebuild(&self, _: &ConfigPackageItem) -> bool {
            false
        }
    }

    /// 空注册表查无实现；注册后按种类命中；重复注册拒绝。
    #[test]
    fn registry_registers_and_looks_up_by_kind() {
        let mut registry = ConfigItemApplierRegistry::new();
        assert!(registry.lookup(ItemKind::FlowDefinition).is_none());
        registry
            .register(Arc::new(StubApplier {
                kind: ItemKind::FlowDefinition,
            }))
            .expect("首次注册可完成");
        assert_eq!(
            registry
                .lookup(ItemKind::FlowDefinition)
                .expect("已注册")
                .item_kind(),
            ItemKind::FlowDefinition
        );
        assert!(registry.lookup(ItemKind::NotifyRule).is_none());
        let err = registry
            .register(Arc::new(StubApplier {
                kind: ItemKind::FlowDefinition,
            }))
            .expect_err("重复注册必须拒绝");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }
}
