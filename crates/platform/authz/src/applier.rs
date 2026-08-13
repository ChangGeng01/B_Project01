//! 三个 AUTHZ 类 ConfigItemApplier（阶段 3a 端口的属主实现体）。
//!
//! AuthzRoleApplier 写 roles 与 role_permission_grants；
//! AuthzPolicyApplier 写 access_policies、sod_rules、approval_chains
//! 与 approval_chain_nodes；AuthzFieldGrantApplier 写 field_permissions。
//! 三者均在发布执行事务内经 [`AuthzConfigWriteStore`] 写库并推进
//! authz_config_versions 版本；不开新事务、不外调（04 计划 §4.7）。
//! SQL 实现体归 ep-adapter-db-pg，本阶段只注册进 Registry 与单测。

use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_REQUEST_INVALID_PAYLOAD;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::SecurityContext;
use ep_platform_release::port::config_item::{
    ChangeKind, ConfigItemApplier, ConfigPackageItem, ItemKind,
};

use crate::sod::SodRoleRule;
use crate::types::{check_permission_item_shape, guard_permission_item_code};

// 规格类型拆居 crate::spec，本模块维持原导出路径不变。
pub use crate::spec::{
    ApprovalChainSpec, AuthzPolicySpec, FieldGrantSpec, PolicySpec, RoleGrantSpec, RoleSpec,
    SodRuleSpec,
};

/// 授权配置写库端口。同事务语义由发布执行侧保证；applier 不开事务、
/// 不外调。空法人数组表示全部法人，由实现侧展开。
pub trait AuthzConfigWriteStore: Send + Sync {
    fn upsert_role(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &RoleSpec,
    ) -> Result<(), AppError>;
    fn delete_role(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        role_code: &str,
    ) -> Result<(), AppError>;
    fn replace_policy_domain(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &AuthzPolicySpec,
    ) -> Result<(), AppError>;
    fn upsert_field_grant(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &FieldGrantSpec,
    ) -> Result<(), AppError>;
    fn delete_field_grant(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &FieldGrantSpec,
    ) -> Result<(), AppError>;
    /// 推进 authz_config_versions 版本行（EFFECTIVE 唯一）。
    fn bump_authz_config_version(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
    ) -> Result<(), AppError>;
}

fn parse_spec<T: serde::de::DeserializeOwned>(
    spec: &Option<String>,
    what: &str,
) -> Result<T, AppError> {
    let Some(text) = spec else {
        return Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("{what} 缺少规格文本"),
        ));
    };
    serde_json::from_str(text).map_err(|e| {
        AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("{what} 规格解析失败：{e}"),
        )
    })
}

/// 变更形态互斥约束：ADD 时 before 空 after 非空、REMOVE 相反、
/// MODIFY 两者均非空。
fn check_change_shape(item: &ConfigPackageItem) -> Result<(), AppError> {
    let bad = || {
        AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("内容项 {} 的变更形态与前后规格不互斥", item.item_code),
        )
    };
    match item.change_kind {
        ChangeKind::Add => {
            if item.before_spec.is_some() || item.after_spec.is_none() {
                return Err(bad());
            }
        }
        ChangeKind::Remove => {
            if item.before_spec.is_none() || item.after_spec.is_some() {
                return Err(bad());
            }
        }
        ChangeKind::Modify => {
            if item.before_spec.is_none() || item.after_spec.is_none() {
                return Err(bad());
            }
        }
    }
    Ok(())
}

fn check_kind(item: &ConfigPackageItem, expected: ItemKind) -> Result<(), AppError> {
    if item.item_kind != expected {
        return Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!(
                "内容项种类不符：期望 {}，实为 {}",
                expected.as_str(),
                item.item_kind.as_str()
            ),
        ));
    }
    Ok(())
}

fn validate_role_code(code: &str) -> Result<(), AppError> {
    let ok = !code.is_empty()
        && code
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if !ok {
        return Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("角色码 {code} 非法，需为大写字母、数字与下划线"),
        ));
    }
    Ok(())
}

fn validate_role_spec(spec: &RoleSpec) -> Result<(), AppError> {
    validate_role_code(&spec.role_code)?;
    for grant in &spec.grants {
        check_permission_item_shape(&grant.permission_item_code)?;
        guard_permission_item_code(&grant.permission_item_code)?;
        if grant.actions.is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("权限项 {} 的动作集合为空", grant.permission_item_code),
            ));
        }
    }
    Ok(())
}

fn validate_policy_spec(spec: &AuthzPolicySpec) -> Result<(), AppError> {
    for p in &spec.policies {
        check_permission_item_shape(&p.object_type)?;
        if let Some(role) = &p.role_code {
            validate_role_code(role)?;
        }
    }
    // 审批链形态与空展开校验：与运行期共用 sod 纯函数。
    for chain in &spec.approval_chains {
        if let Some(v) = crate::sod::check_chain_shape(&chain.nodes) {
            return Err(crate::sod::violation_error(&v));
        }
        if let Some(v) = crate::sod::check_nodes_non_empty(&chain.nodes) {
            return Err(crate::sod::violation_error(&v));
        }
    }
    Ok(())
}

/// AUTHZ_ROLE applier。
pub struct AuthzRoleApplier {
    store: Arc<dyn AuthzConfigWriteStore>,
}

impl AuthzRoleApplier {
    pub fn new(store: Arc<dyn AuthzConfigWriteStore>) -> Self {
        Self { store }
    }
}

impl ConfigItemApplier for AuthzRoleApplier {
    fn item_kind(&self) -> ItemKind {
        ItemKind::AuthzRole
    }

    fn validate(&self, item: &ConfigPackageItem, _ctx: &SecurityContext) -> Result<(), AppError> {
        check_kind(item, ItemKind::AuthzRole)?;
        check_change_shape(item)?;
        if item.change_kind != ChangeKind::Remove {
            validate_role_spec(&parse_spec::<RoleSpec>(&item.after_spec, "AUTHZ_ROLE")?)?;
        }
        Ok(())
    }

    fn apply(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        match item.change_kind {
            ChangeKind::Remove => {
                let before: RoleSpec = parse_spec(&item.before_spec, "AUTHZ_ROLE")?;
                self.store.delete_role(tx, le, &before.role_code)?;
            }
            _ => {
                let spec: RoleSpec = parse_spec(&item.after_spec, "AUTHZ_ROLE")?;
                self.store.upsert_role(tx, le, &spec)?;
            }
        }
        self.store.bump_authz_config_version(tx, le)
    }

    fn revert(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        match item.change_kind {
            ChangeKind::Add => {
                let after: RoleSpec = parse_spec(&item.after_spec, "AUTHZ_ROLE")?;
                self.store.delete_role(tx, le, &after.role_code)?;
            }
            _ => {
                let before: RoleSpec = parse_spec(&item.before_spec, "AUTHZ_ROLE")?;
                self.store.upsert_role(tx, le, &before)?;
            }
        }
        self.store.bump_authz_config_version(tx, le)
    }

    /// 授权快照属派生存储，随版本推进轮询重建。
    fn requires_derived_store_rebuild(&self, _item: &ConfigPackageItem) -> bool {
        true
    }
}

/// AUTHZ_POLICY applier。
pub struct AuthzPolicyApplier {
    store: Arc<dyn AuthzConfigWriteStore>,
}

impl AuthzPolicyApplier {
    pub fn new(store: Arc<dyn AuthzConfigWriteStore>) -> Self {
        Self { store }
    }
}

impl ConfigItemApplier for AuthzPolicyApplier {
    fn item_kind(&self) -> ItemKind {
        ItemKind::AuthzPolicy
    }

    fn validate(&self, item: &ConfigPackageItem, _ctx: &SecurityContext) -> Result<(), AppError> {
        check_kind(item, ItemKind::AuthzPolicy)?;
        check_change_shape(item)?;
        if item.change_kind != ChangeKind::Remove {
            validate_policy_spec(&parse_spec::<AuthzPolicySpec>(
                &item.after_spec,
                "AUTHZ_POLICY",
            )?)?;
        }
        Ok(())
    }

    fn apply(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        // 策略域整体替换：REMOVE 落空规格。
        let spec: AuthzPolicySpec = match item.change_kind {
            ChangeKind::Remove => AuthzPolicySpec::default(),
            _ => parse_spec(&item.after_spec, "AUTHZ_POLICY")?,
        };
        self.store.replace_policy_domain(tx, le, &spec)?;
        self.store.bump_authz_config_version(tx, le)
    }

    fn revert(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        let spec: AuthzPolicySpec = match item.change_kind {
            ChangeKind::Add => AuthzPolicySpec::default(),
            _ => parse_spec(&item.before_spec, "AUTHZ_POLICY")?,
        };
        self.store.replace_policy_domain(tx, le, &spec)?;
        self.store.bump_authz_config_version(tx, le)
    }

    fn requires_derived_store_rebuild(&self, _item: &ConfigPackageItem) -> bool {
        true
    }
}

/// AUTHZ_FIELD_GRANT applier。
pub struct AuthzFieldGrantApplier {
    store: Arc<dyn AuthzConfigWriteStore>,
}

impl AuthzFieldGrantApplier {
    pub fn new(store: Arc<dyn AuthzConfigWriteStore>) -> Self {
        Self { store }
    }
}

impl ConfigItemApplier for AuthzFieldGrantApplier {
    fn item_kind(&self) -> ItemKind {
        ItemKind::AuthzFieldGrant
    }

    fn validate(&self, item: &ConfigPackageItem, _ctx: &SecurityContext) -> Result<(), AppError> {
        check_kind(item, ItemKind::AuthzFieldGrant)?;
        check_change_shape(item)?;
        if item.change_kind != ChangeKind::Remove {
            let spec: FieldGrantSpec = parse_spec(&item.after_spec, "AUTHZ_FIELD_GRANT")?;
            validate_role_code(&spec.role_code)?;
            check_permission_item_shape(&spec.object_type)?;
        }
        Ok(())
    }

    fn apply(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        match item.change_kind {
            ChangeKind::Remove => {
                let before: FieldGrantSpec = parse_spec(&item.before_spec, "AUTHZ_FIELD_GRANT")?;
                self.store.delete_field_grant(tx, le, &before)?;
            }
            _ => {
                let spec: FieldGrantSpec = parse_spec(&item.after_spec, "AUTHZ_FIELD_GRANT")?;
                self.store.upsert_field_grant(tx, le, &spec)?;
            }
        }
        self.store.bump_authz_config_version(tx, le)
    }

    fn revert(
        &self,
        tx: &mut dyn Tx,
        item: &ConfigPackageItem,
        _ctx: &SecurityContext,
    ) -> Result<(), AppError> {
        let le = item.applies_to_legal_entity_ids.as_slice();
        match item.change_kind {
            ChangeKind::Add => {
                let after: FieldGrantSpec = parse_spec(&item.after_spec, "AUTHZ_FIELD_GRANT")?;
                self.store.delete_field_grant(tx, le, &after)?;
            }
            _ => {
                let before: FieldGrantSpec = parse_spec(&item.before_spec, "AUTHZ_FIELD_GRANT")?;
                self.store.upsert_field_grant(tx, le, &before)?;
            }
        }
        self.store.bump_authz_config_version(tx, le)
    }

    fn requires_derived_store_rebuild(&self, _item: &ConfigPackageItem) -> bool {
        true
    }
}

/// 三个 applier 一次注册进表；供两个 apps 的 wiring 调用。
pub fn register_authz_appliers(
    registry: &mut ep_platform_release::port::config_item::ConfigItemApplierRegistry,
    store: Arc<dyn AuthzConfigWriteStore>,
) -> Result<(), AppError> {
    registry.register(Arc::new(AuthzRoleApplier::new(store.clone())))?;
    registry.register(Arc::new(AuthzPolicyApplier::new(store.clone())))?;
    registry.register(Arc::new(AuthzFieldGrantApplier::new(store)))?;
    Ok(())
}

/// sod_rules 规格 → 运行形态。
impl From<SodRuleSpec> for SodRoleRule {
    fn from(s: SodRuleSpec) -> Self {
        SodRoleRule {
            rule_code: s.rule_code,
            role_a: s.role_a,
            role_b: s.role_b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sod::ApprovalNodeSpec;
    use crate::types::{Action, FieldVisibility, PolicyCondition, PolicyEffect};
    use ep_foundation::security::SecurityLevel;
    use ep_platform_release::port::config_item::ConfigItemApplierRegistry;
    use std::sync::Mutex;

    struct FixtureTx;
    impl Tx for FixtureTx {
        fn tx_id(&self) -> ep_foundation::port::tx::TxId {
            ep_foundation::port::tx::TxId(uuid::Uuid::nil())
        }
        fn isolation(&self) -> ep_foundation::port::tx::IsolationKind {
            ep_foundation::port::tx::IsolationKind::ReadCommitted
        }
        fn legal_entity_id(&self) -> Id<LegalEntity> {
            Id::from_uuid(uuid::Uuid::from_u128(1))
        }
        fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
            self
        }
    }

    /// 记录调用序列的写库载体：用于同事务写入与版本推进的顺序断言。
    #[derive(Default)]
    struct RecordingStore {
        calls: Mutex<Vec<String>>,
    }

    impl AuthzConfigWriteStore for RecordingStore {
        fn upsert_role(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
            spec: &RoleSpec,
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(format!("upsert_role:{}", spec.role_code));
            Ok(())
        }
        fn delete_role(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
            role_code: &str,
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(format!("delete_role:{role_code}"));
            Ok(())
        }
        fn replace_policy_domain(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
            spec: &AuthzPolicySpec,
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(format!("replace_policy_domain:{}", spec.policies.len()));
            Ok(())
        }
        fn upsert_field_grant(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
            spec: &FieldGrantSpec,
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(format!("upsert_field_grant:{}", spec.field_name));
            Ok(())
        }
        fn delete_field_grant(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
            spec: &FieldGrantSpec,
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(format!("delete_field_grant:{}", spec.field_name));
            Ok(())
        }
        fn bump_authz_config_version(
            &self,
            _: &mut dyn Tx,
            _: &[Id<LegalEntity>],
        ) -> Result<(), AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push("bump".into());
            Ok(())
        }
    }

    fn item(
        kind: ItemKind,
        change: ChangeKind,
        before: Option<String>,
        after: Option<String>,
    ) -> ConfigPackageItem {
        ConfigPackageItem {
            id: uuid::Uuid::from_u128(1),
            security_level: SecurityLevel::Internal,
            config_package_id: uuid::Uuid::from_u128(2),
            item_kind: kind,
            item_code: "IT-1".into(),
            change_kind: change,
            applies_to_legal_entity_ids: vec![],
            before_spec: before,
            after_spec: after,
            item_hash: String::new(),
            sort_no: 1,
        }
    }

    fn role_json(code: &str) -> String {
        serde_json::to_string(&RoleSpec {
            role_code: code.into(),
            is_portal_role: false,
            grants: vec![RoleGrantSpec {
                permission_item_code: "sales.sales_order".into(),
                actions: vec![Action::View],
            }],
        })
        .expect("可序列化")
    }

    #[test]
    fn role_applier_applies_and_bumps_in_order() {
        let store = Arc::new(RecordingStore::default());
        let applier = AuthzRoleApplier::new(store.clone());
        let ctx = crate::types::tests::ctx_with(
            vec!["SALES"],
            ep_foundation::security::context::ClientKind::Win,
        );
        let it = item(
            ItemKind::AuthzRole,
            ChangeKind::Add,
            None,
            Some(role_json("SALES_ADMIN")),
        );
        applier.validate(&it, &ctx).expect("可校验");
        let mut tx = FixtureTx;
        applier.apply(&mut tx, &it, &ctx).expect("可应用");
        let calls = store.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(
            calls.as_slice(),
            ["upsert_role:SALES_ADMIN", "bump"],
            "写库与版本推进同事务按序"
        );
        drop(calls);
        // revert ADD = 删除回滚。
        applier.revert(&mut tx, &it, &ctx).expect("可回退");
        let calls = store.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(calls[2..], ["delete_role:SALES_ADMIN", "bump"]);
    }

    #[test]
    fn role_applier_rejects_forbidden_permission_prefix() {
        let store = Arc::new(RecordingStore::default());
        let applier = AuthzRoleApplier::new(store);
        let ctx = crate::types::tests::ctx_with(
            vec!["SALES"],
            ep_foundation::security::context::ClientKind::Win,
        );
        let spec = RoleSpec {
            role_code: "EVIL".into(),
            is_portal_role: false,
            grants: vec![RoleGrantSpec {
                permission_item_code: "platform.legal_entity_isolation".into(),
                actions: vec![Action::View],
            }],
        };
        let it = item(
            ItemKind::AuthzRole,
            ChangeKind::Add,
            None,
            Some(serde_json::to_string(&spec).expect("可序列化")),
        );
        assert!(applier.validate(&it, &ctx).is_err(), "禁入前缀拒");
        // 小写角色码同样拒。
        let it = item(
            ItemKind::AuthzRole,
            ChangeKind::Add,
            None,
            Some(role_json("lower_case")),
        );
        assert!(applier.validate(&it, &ctx).is_err());
    }

    #[test]
    fn policy_applier_rejects_empty_node_expansion_and_applies_replace() {
        let store = Arc::new(RecordingStore::default());
        let applier = AuthzPolicyApplier::new(store.clone());
        let ctx = crate::types::tests::ctx_with(
            vec!["SALES"],
            ep_foundation::security::context::ClientKind::Win,
        );
        let bad = AuthzPolicySpec {
            approval_chains: vec![ApprovalChainSpec {
                chain_code: "C1".into(),
                nodes: vec![ApprovalNodeSpec {
                    node_seq: 1,
                    quorum: 1,
                    approver_user_ids: vec![],
                }],
            }],
            ..AuthzPolicySpec::default()
        };
        let it = item(
            ItemKind::AuthzPolicy,
            ChangeKind::Add,
            None,
            Some(serde_json::to_string(&bad).expect("可序列化")),
        );
        let err = applier.validate(&it, &ctx).expect_err("空展开拒保存");
        assert_eq!(
            err.code,
            ep_foundation::error::codes::PLATFORM_APPROVAL_NODE_HAS_NO_APPROVER
        );
        let ok = AuthzPolicySpec {
            policies: vec![PolicySpec {
                policy_code: "P1".into(),
                role_code: None,
                object_type: "sales.sales_order".into(),
                effect: PolicyEffect::Allow,
                priority: 10,
                condition: PolicyCondition::default(),
            }],
            ..AuthzPolicySpec::default()
        };
        let it = item(
            ItemKind::AuthzPolicy,
            ChangeKind::Add,
            None,
            Some(serde_json::to_string(&ok).expect("可序列化")),
        );
        applier.validate(&it, &ctx).expect("可校验");
        let mut tx = FixtureTx;
        applier.apply(&mut tx, &it, &ctx).expect("可应用");
        let calls = store.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(calls.as_slice(), ["replace_policy_domain:1", "bump"]);
    }

    #[test]
    fn field_grant_applier_handles_add_modify_remove() {
        let store = Arc::new(RecordingStore::default());
        let applier = AuthzFieldGrantApplier::new(store.clone());
        let ctx = crate::types::tests::ctx_with(
            vec!["SALES"],
            ep_foundation::security::context::ClientKind::Win,
        );
        let spec = FieldGrantSpec {
            role_code: "FINANCE".into(),
            object_type: "finance.cash_accounts".into(),
            field_name: "bank_no".into(),
            visibility: FieldVisibility::Masked(crate::types::MaskStyle::KeepLast4),
        };
        let json = serde_json::to_string(&spec).expect("可序列化");
        let it = item(
            ItemKind::AuthzFieldGrant,
            ChangeKind::Add,
            None,
            Some(json.clone()),
        );
        let mut tx = FixtureTx;
        applier.apply(&mut tx, &it, &ctx).expect("ADD 可应用");
        let it = item(
            ItemKind::AuthzFieldGrant,
            ChangeKind::Remove,
            Some(json),
            None,
        );
        applier.apply(&mut tx, &it, &ctx).expect("REMOVE 可应用");
        let calls = store.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(
            calls.as_slice(),
            [
                "upsert_field_grant:bank_no",
                "bump",
                "delete_field_grant:bank_no",
                "bump"
            ]
        );
    }

    #[test]
    fn change_shape_must_be_exclusive() {
        let store = Arc::new(RecordingStore::default());
        let applier = AuthzRoleApplier::new(store);
        let ctx = crate::types::tests::ctx_with(
            vec!["SALES"],
            ep_foundation::security::context::ClientKind::Win,
        );
        // ADD 携带 before 即拒。
        let it = item(
            ItemKind::AuthzRole,
            ChangeKind::Add,
            Some(role_json("A")),
            Some(role_json("B")),
        );
        assert!(applier.validate(&it, &ctx).is_err());
        // 种类错配即拒。
        let it = item(
            ItemKind::AuthzPolicy,
            ChangeKind::Add,
            None,
            Some(role_json("B")),
        );
        assert!(applier.validate(&it, &ctx).is_err());
    }

    #[test]
    fn three_appliers_register_into_registry_once() {
        let store = Arc::new(RecordingStore::default());
        let mut registry = ConfigItemApplierRegistry::new();
        register_authz_appliers(&mut registry, store).expect("注册成功");
        assert!(registry.lookup(ItemKind::AuthzRole).is_some());
        assert!(registry.lookup(ItemKind::AuthzPolicy).is_some());
        assert!(registry.lookup(ItemKind::AuthzFieldGrant).is_some());
        assert!(registry
            .lookup(ItemKind::AuthzRole)
            .expect("已注册")
            .requires_derived_store_rebuild(&item(
                ItemKind::AuthzRole,
                ChangeKind::Add,
                None,
                Some(role_json("X"))
            )));
    }
}
