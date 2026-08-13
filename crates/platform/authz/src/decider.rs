//! 授权判定器：四阶段流水线的应用侧后三阶段。
//!
//! 阶段一（法人）由 PostgreSQL RLS 承担，判定器不做应用侧法人比较；
//! 阶段二（对象级）查 AuthzSnapshot：显式 DENY 策略优先，再看
//! role_permission_grants 是否命中所求动作；阶段三（记录级）交
//! [`ScopeCompiler`]；阶段四（字段级）交 [`crate::field::FieldProjector`]，
//! 判定器只到记录级为止（04 计划 §4.1）。角色集为空直接拒；
//! 未在 `object_scope_bindings` 登记的 object_type 拒 ScopeBindingMissing。

use std::sync::Arc;
use std::time::Instant;

use ep_foundation::error::AppError;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::SecurityContext;

use crate::metrics::AuthzMetricsSink;
use crate::scope::{CompiledScope, ScopeCompiler};
use crate::snapshot::{AuthzSnapshotHolder, EntityAuthzData};
use crate::types::{Action, Decision, DenyReason, PolicyEffect, RecordScope};

/// 一次完整判定的产出：结论与记录级范围（拒绝时为 None）。
#[derive(Clone, PartialEq, Debug)]
pub struct Verdict {
    pub decision: Decision,
    pub scope: Option<RecordScope>,
}

/// 授权判定器。读快照 + 编译记录范围，不做任何写操作。
pub struct AccessDecider {
    holder: Arc<AuthzSnapshotHolder>,
    compiler: ScopeCompiler,
    metrics: Arc<dyn AuthzMetricsSink>,
}

impl AccessDecider {
    pub fn new(
        holder: Arc<AuthzSnapshotHolder>,
        compiler: ScopeCompiler,
        metrics: Arc<dyn AuthzMetricsSink>,
    ) -> Self {
        Self {
            holder,
            compiler,
            metrics,
        }
    }

    /// 四阶段流水线的阶段二与阶段三；阶段四在端点侧经 FieldProjector 承接。
    pub async fn decide(
        &self,
        ctx: &SecurityContext,
        object_type: &str,
        action: Action,
        tx: &mut dyn Tx,
    ) -> Result<Verdict, AppError> {
        let started = Instant::now();
        let verdict = self.pipeline(ctx, object_type, action, tx).await?;
        let seconds = started.elapsed().as_secs_f64();
        let le = ctx.legal_entity_id.to_string();
        let allowed = verdict.decision == Decision::Allow;
        self.metrics.observe_decision(&le, allowed, seconds);
        if let Decision::Deny(reason) = &verdict.decision {
            self.metrics.count_denied(&le, reason.as_metric_reason());
        }
        Ok(verdict)
    }

    async fn pipeline(
        &self,
        ctx: &SecurityContext,
        object_type: &str,
        action: Action,
        tx: &mut dyn Tx,
    ) -> Result<Verdict, AppError> {
        // 前置硬条件：无角色直接拒；未登记对象类型拒范围绑定缺失。
        if ctx.roles.is_empty() {
            return Ok(deny(DenyReason::ObjectForbidden));
        }
        if !self.holder.is_object_registered(object_type) {
            return Ok(deny(DenyReason::ScopeBindingMissing {
                object_type: object_type.to_owned(),
            }));
        }
        // 阶段一由 RLS 承担：无分片即无授权数据，按对象级拒绝。
        let Some(entity) = self.holder.entity(ctx.legal_entity_id) else {
            return Ok(deny(DenyReason::ObjectForbidden));
        };
        // 阶段二：显式 DENY 优先，再看授予命中。
        if object_level_denied(&entity, ctx, object_type) {
            return Ok(deny(DenyReason::ObjectForbidden));
        }
        if !role_grants_hit(&entity, ctx, object_type, action) {
            return Ok(deny(DenyReason::ObjectForbidden));
        }
        // 阶段三：记录级范围编译。
        let Some(binding) = self.holder.binding(object_type) else {
            return Ok(deny(DenyReason::ScopeBindingMissing {
                object_type: object_type.to_owned(),
            }));
        };
        let CompiledScope { scope, .. } = self.compiler.compile(ctx, &binding, tx).await?;
        if scope == RecordScope::None {
            return Ok(deny(DenyReason::RecordNotVisible));
        }
        Ok(Verdict {
            decision: Decision::Allow,
            scope: Some(scope),
        })
    }
}

fn deny(reason: DenyReason) -> Verdict {
    Verdict {
        decision: Decision::Deny(reason),
        scope: None,
    }
}

/// 阶段二之一：匹配对象类型的 DENY 策略，条件成立即拒（优先级升序，
/// 显式拒绝先于一切允许）。
fn object_level_denied(entity: &EntityAuthzData, ctx: &SecurityContext, object_type: &str) -> bool {
    entity
        .policies
        .iter()
        .filter(|p| p.effect == PolicyEffect::Deny)
        .filter(|p| p.object_type.as_ref() == object_type)
        .any(|p| role_matches(p.role_code.as_deref(), ctx) && p.condition.evaluate(ctx))
}

/// 阶段二之二：角色 × 对象类型的授予集合包含所求动作。
fn role_grants_hit(
    entity: &EntityAuthzData,
    ctx: &SecurityContext,
    object_type: &str,
    action: Action,
) -> bool {
    ctx.roles.iter().any(|role| {
        entity
            .role_grants
            .get(&(
                Arc::from(role.as_str().to_owned()),
                Arc::from(object_type.to_owned()),
            ))
            .map(|actions| actions.contains(&action))
            .unwrap_or(false)
    })
}

fn role_matches(role_code: Option<&str>, ctx: &SecurityContext) -> bool {
    match role_code {
        // 空表示约束全部角色，条件求值即命中。
        None => true,
        Some(code) => ctx.roles.iter().any(|r| r.as_str() == code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::SilentMetricsSink;
    use crate::scope::ScopeConfig;
    use crate::snapshot::{AccessPolicyEntry, SnapshotMap};
    use crate::types::tests::ctx_with;
    use crate::types::{ObjectScopeBinding, PolicyCondition};
    use ep_foundation::id::marker::{Department, LegalEntity};
    use ep_foundation::id::Id;
    use ep_foundation::security::context::ClientKind;
    use std::collections::{BTreeSet, HashMap};

    struct FixtureTx;
    impl Tx for FixtureTx {
        fn tx_id(&self) -> ep_foundation::port::tx::TxId {
            ep_foundation::port::tx::TxId(uuid::Uuid::nil())
        }
        fn isolation(&self) -> ep_foundation::port::tx::IsolationKind {
            ep_foundation::port::tx::IsolationKind::ReadCommitted
        }
        fn legal_entity_id(&self) -> Id<LegalEntity> {
            Id::from_uuid(uuid::Uuid::from_u128(3))
        }
        fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
            self
        }
    }

    struct FixtureClosure;
    #[async_trait::async_trait]
    impl ep_platform_tenancy::DepartmentClosureQuery for FixtureClosure {
        async fn descendant_ids(
            &self,
            _tx: &mut dyn Tx,
            _le: Id<LegalEntity>,
            root: Id<Department>,
            _max_depth: u8,
        ) -> Result<Vec<Id<Department>>, AppError> {
            Ok(vec![root])
        }
    }

    fn binding() -> ObjectScopeBinding {
        ObjectScopeBinding {
            object_type: Arc::from("sales.sales_order"),
            schema_name: Arc::from("sales"),
            table_name: Arc::from("sales_order"),
            owner_user_col: Some(Arc::from("owner_user_id")),
            owning_dept_col: None,
            project_col: None,
            customer_col: None,
            security_level_col: Arc::from("security_level"),
            valid_from_col: None,
            valid_to_col: None,
        }
    }

    fn entity_with_grant(role: &str, object_type: &str) -> EntityAuthzData {
        let mut role_grants = HashMap::new();
        role_grants.insert(
            (Arc::from(role), Arc::from(object_type)),
            BTreeSet::from([Action::View]),
        );
        EntityAuthzData {
            version_no: 1,
            checksum: Arc::from("c1"),
            role_grants,
            ..EntityAuthzData::default()
        }
    }

    fn decider_with(entity: Option<EntityAuthzData>) -> AccessDecider {
        let holder = Arc::new(AuthzSnapshotHolder::empty());
        let mut entities = SnapshotMap::new();
        if let Some(data) = entity {
            entities.insert(Id::from_uuid(uuid::Uuid::from_u128(3)), Arc::new(data));
        }
        holder.replace(entities, vec![binding()]);
        let compiler = ScopeCompiler::new(
            Arc::new(FixtureClosure),
            ScopeConfig::default(),
            Arc::new(SilentMetricsSink),
        );
        AccessDecider::new(holder, compiler, Arc::new(SilentMetricsSink))
    }

    #[tokio::test]
    async fn empty_roles_are_denied_outright() {
        let d = decider_with(Some(entity_with_grant("SALES", "sales.sales_order")));
        let ctx = ctx_with(vec![], ClientKind::Win);
        let mut tx = FixtureTx;
        let v = d
            .decide(&ctx, "sales.sales_order", Action::View, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(v.decision, Decision::Deny(DenyReason::ObjectForbidden));
    }

    #[tokio::test]
    async fn unregistered_object_type_yields_scope_binding_missing() {
        let d = decider_with(Some(entity_with_grant("SALES", "sales.sales_order")));
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let mut tx = FixtureTx;
        let v = d
            .decide(&ctx, "hr.payroll", Action::View, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(
            v.decision,
            Decision::Deny(DenyReason::ScopeBindingMissing {
                object_type: "hr.payroll".into()
            })
        );
    }

    #[tokio::test]
    async fn deny_policy_wins_over_grant() {
        let mut entity = entity_with_grant("SALES", "sales.sales_order");
        entity.policies.push(AccessPolicyEntry {
            role_code: Some(Arc::from("SALES")),
            object_type: Arc::from("sales.sales_order"),
            effect: PolicyEffect::Deny,
            priority: 1,
            condition: PolicyCondition::default(),
        });
        let d = decider_with(Some(entity));
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let mut tx = FixtureTx;
        let v = d
            .decide(&ctx, "sales.sales_order", Action::View, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(v.decision, Decision::Deny(DenyReason::ObjectForbidden));
    }

    #[tokio::test]
    async fn grant_hit_without_deny_allows_with_scope() {
        let d = decider_with(Some(entity_with_grant("SALES", "sales.sales_order")));
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let mut tx = FixtureTx;
        let v = d
            .decide(&ctx, "sales.sales_order", Action::View, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(v.decision, Decision::Allow);
        assert_eq!(v.scope, Some(RecordScope::All), "部门全量即记录全量");
        // 动作未授予即拒。
        let v = d
            .decide(&ctx, "sales.sales_order", Action::Export, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(v.decision, Decision::Deny(DenyReason::ObjectForbidden));
    }

    #[tokio::test]
    async fn missing_entity_shard_denies() {
        let d = decider_with(None);
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let mut tx = FixtureTx;
        let v = d
            .decide(&ctx, "sales.sales_order", Action::View, &mut tx)
            .await
            .expect("可判定");
        assert_eq!(v.decision, Decision::Deny(DenyReason::ObjectForbidden));
    }
}
