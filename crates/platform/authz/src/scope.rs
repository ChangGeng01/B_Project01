//! 记录级范围编译器（判定流水线阶段三）。
//!
//! 部门闭包经 [`DepartmentClosureQuery::descendant_ids`]（A-04，`&mut dyn Tx`）；
//! `max_depth` 取 EP__AUTHZ__SCOPE__MAX_DEPARTMENT_DEPTH（默认 8），超限截断
//! 记 WARN 并计 `ep_authz_scope_truncated_total`。部门集合超过
//! EP__AUTHZ__SCOPE__IN_LIST_THRESHOLD（默认 200）时谓词标记改走 EXISTS。
//! 谓词构造完成后不再触库；SQL 渲染由 ep-adapter-db-pg 承接。

use std::sync::Arc;

use ep_foundation::error::AppError;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::context::{DepartmentScope, RecordShareGrant};
use ep_foundation::security::SecurityContext;
use ep_platform_tenancy::{DepartmentClosureQuery, MAX_ORG_DEPTH};

use crate::metrics::AuthzMetricsSink;
use crate::types::{
    ObjectScopeBinding, RecordPredicate, RecordScope, ValidityWindow, IN_LIST_THRESHOLD,
};

/// 范围编译配置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ScopeConfig {
    /// 部门闭包展开的深度上限。
    pub max_department_depth: u8,
    /// IN 列表退化 EXISTS 的阈值。
    pub in_list_threshold: usize,
}

impl Default for ScopeConfig {
    fn default() -> Self {
        Self {
            max_department_depth: 8,
            in_list_threshold: IN_LIST_THRESHOLD,
        }
    }
}

/// 编译结果：范围与是否发生深度截断。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompiledScope {
    pub scope: RecordScope,
    pub truncated: bool,
}

/// 记录级范围编译器。
pub struct ScopeCompiler {
    closure: Arc<dyn DepartmentClosureQuery>,
    config: ScopeConfig,
    metrics: Arc<dyn AuthzMetricsSink>,
}

impl ScopeCompiler {
    pub fn new(
        closure: Arc<dyn DepartmentClosureQuery>,
        config: ScopeConfig,
        metrics: Arc<dyn AuthzMetricsSink>,
    ) -> Self {
        Self {
            closure,
            config,
            metrics,
        }
    }

    pub fn config(&self) -> &ScopeConfig {
        &self.config
    }

    /// 阶段三编译：部门闭包展开是唯一触库步骤，其后一律纯谓词构造。
    pub async fn compile(
        &self,
        ctx: &SecurityContext,
        binding: &ObjectScopeBinding,
        tx: &mut dyn Tx,
    ) -> Result<CompiledScope, AppError> {
        let (departments, unlimited, truncated) = self.department_dimension(ctx, tx).await?;
        if truncated {
            self.metrics
                .count_scope_truncated(&ctx.legal_entity_id.to_string());
        }
        if unlimited {
            return Ok(CompiledScope {
                scope: RecordScope::All,
                truncated,
            });
        }
        let predicate = self.build_predicate(ctx, binding, departments);
        let scope = if allows_nothing(&predicate) {
            RecordScope::None
        } else {
            RecordScope::Predicate(predicate)
        };
        Ok(CompiledScope { scope, truncated })
    }

    /// 部门维度：返回（展开后的部门集合，是否不受限，是否截断）。
    async fn department_dimension(
        &self,
        ctx: &SecurityContext,
        tx: &mut dyn Tx,
    ) -> Result<
        (
            Arc<[ep_foundation::id::Id<ep_foundation::id::marker::Department>]>,
            bool,
            bool,
        ),
        AppError,
    > {
        match &ctx.department_scope {
            DepartmentScope::All => Ok((
                Arc::from([] as [ep_foundation::id::Id<ep_foundation::id::marker::Department>; 0]),
                true,
                false,
            )),
            DepartmentScope::Explicit(list) => Ok((list.clone(), false, false)),
            DepartmentScope::Subtree(root) => {
                let capped = self
                    .closure
                    .descendant_ids(
                        tx,
                        ctx.legal_entity_id,
                        *root,
                        self.config.max_department_depth,
                    )
                    .await?;
                let truncated = self.probe_truncation(ctx, tx, *root, capped.len()).await?;
                Ok((Arc::from(capped.into_boxed_slice()), false, truncated))
            }
        }
    }

    /// 截断探测：以 max_depth+1 再查一次，返回更多即说明被深度截断。
    /// 仅在深度上限未达组织硬上限时探测，避免无谓查询。
    async fn probe_truncation(
        &self,
        ctx: &SecurityContext,
        tx: &mut dyn Tx,
        root: ep_foundation::id::Id<ep_foundation::id::marker::Department>,
        capped_len: usize,
    ) -> Result<bool, AppError> {
        if self.config.max_department_depth as usize >= MAX_ORG_DEPTH {
            return Ok(false);
        }
        let probe = self
            .closure
            .descendant_ids(
                tx,
                ctx.legal_entity_id,
                root,
                self.config.max_department_depth.saturating_add(1),
            )
            .await?;
        Ok(probe.len() > capped_len)
    }

    /// 纯谓词构造：部门、项目、客户、共享记录、密级上限与日期窗口。
    fn build_predicate(
        &self,
        ctx: &SecurityContext,
        binding: &ObjectScopeBinding,
        departments: Arc<[ep_foundation::id::Id<ep_foundation::id::marker::Department>]>,
    ) -> RecordPredicate {
        let prefer_exists = departments.len() > self.config.in_list_threshold;
        let shared: Vec<uuid::Uuid> = ctx
            .record_shares
            .iter()
            .filter(|s| s.object_type.as_ref() == binding.object_type.as_ref())
            .filter(|s| matches!(s.grant, RecordShareGrant::Read | RecordShareGrant::Write))
            .map(|s| s.object_id)
            .collect();
        let windows: Vec<ValidityWindow> =
            if binding.valid_from_col.is_some() || binding.valid_to_col.is_some() {
                vec![ValidityWindow {
                    from_col: binding.valid_from_col.clone(),
                    to_col: binding.valid_to_col.clone(),
                }]
            } else {
                Vec::new()
            };
        RecordPredicate {
            owner_self: binding.owner_user_col.is_some(),
            departments,
            prefer_exists_for_departments: prefer_exists,
            projects: ctx.project_scope.clone(),
            customers: ctx.customer_scope.clone(),
            shared_record_ids: Arc::from(shared.into_boxed_slice()),
            max_security_level: ctx.clearance_level,
            validity_windows: Arc::from(windows.into_boxed_slice()),
        }
    }
}

/// 谓词各分支全空即没有任何可见记录。
fn allows_nothing(p: &RecordPredicate) -> bool {
    !p.owner_self
        && p.departments.is_empty()
        && p.projects.is_empty()
        && p.customers.is_empty()
        && p.shared_record_ids.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::tests::ctx_with;
    use ep_foundation::id::marker::{Department, LegalEntity};
    use ep_foundation::id::Id;
    use ep_foundation::security::context::{ClientKind, HumanContextInput};
    use ep_foundation::security::SecurityContext;
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
            Id::from_uuid(uuid::Uuid::from_u128(3))
        }
        fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
            self
        }
    }

    /// 恒定返回 base 个后代的闭包载体；探测层数超出配置深度时追加
    /// probe_extra 个，用于模拟「更深还有子孙」的截断现场。
    struct DepthClosure {
        base: usize,
        probe_extra: usize,
        calls: Mutex<Vec<u8>>,
    }

    #[async_trait::async_trait]
    impl DepartmentClosureQuery for DepthClosure {
        async fn descendant_ids(
            &self,
            _tx: &mut dyn Tx,
            _le: Id<LegalEntity>,
            _root: Id<Department>,
            max_depth: u8,
        ) -> Result<Vec<Id<Department>>, AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(max_depth);
            let extra = usize::from(max_depth > 8) * self.probe_extra;
            Ok((0..self.base + extra)
                .map(|n| Id::from_uuid(uuid::Uuid::from_u128(n as u128 + 1)))
                .collect())
        }
    }

    fn binding_with(owner: bool, valid: bool) -> ObjectScopeBinding {
        ObjectScopeBinding {
            object_type: Arc::from("sales.sales_order"),
            schema_name: Arc::from("sales"),
            table_name: Arc::from("sales_order"),
            owner_user_col: owner.then(|| Arc::from("owner_user_id")),
            owning_dept_col: None,
            project_col: None,
            customer_col: None,
            security_level_col: Arc::from("security_level"),
            valid_from_col: valid.then(|| Arc::from("valid_from")),
            valid_to_col: valid.then(|| Arc::from("valid_to")),
        }
    }

    fn compiler_with(
        closure: Arc<dyn DepartmentClosureQuery>,
        config: ScopeConfig,
    ) -> (ScopeCompiler, Arc<crate::metrics::SilentMetricsSink>) {
        let metrics = Arc::new(crate::metrics::SilentMetricsSink);
        (
            ScopeCompiler::new(closure, config, metrics.clone()),
            metrics,
        )
    }

    #[tokio::test]
    async fn subtree_expansion_truncation_is_detected_by_probe() {
        // 配置深度内恒 8 个，多探一层多出 1 个 → 截断成立。
        let closure = Arc::new(DepthClosure {
            base: 8,
            probe_extra: 1,
            calls: Mutex::new(Vec::new()),
        });
        let (compiler, _) = compiler_with(closure.clone(), ScopeConfig::default());
        let mut ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        ctx = with_subtree(ctx);
        let mut tx = FixtureTx;
        let out = compiler
            .compile(&ctx, &binding_with(true, true), &mut tx)
            .await
            .expect("可编译");
        assert!(out.truncated, "探测深度多一层返回更多即截断");
        match &out.scope {
            RecordScope::Predicate(p) => {
                assert_eq!(p.departments.len(), 8);
                assert_eq!(p.validity_windows.len(), 1, "日期窗口按绑定登记产出");
                assert!(p.owner_self);
            }
            other => panic!("应为谓词，实为 {other:?}"),
        }
    }

    #[tokio::test]
    async fn in_list_degrades_to_exists_beyond_threshold() {
        let closure = Arc::new(DepthClosure {
            base: 205,
            probe_extra: 0,
            calls: Mutex::new(Vec::new()),
        });
        let (compiler, _) = compiler_with(closure, ScopeConfig::default());
        let ctx = with_subtree(ctx_with(vec!["SALES"], ClientKind::Win));
        let mut tx = FixtureTx;
        let out = compiler
            .compile(&ctx, &binding_with(false, false), &mut tx)
            .await
            .expect("可编译");
        assert!(!out.truncated, "base 恒定不随深度增长，未截断");
        match out.scope {
            RecordScope::Predicate(p) => {
                assert!(p.departments.len() > IN_LIST_THRESHOLD);
                assert!(p.prefer_exists_for_departments, "超阈值退化 EXISTS");
            }
            other => panic!("应为谓词，实为 {other:?}"),
        }
    }

    #[tokio::test]
    async fn department_all_yields_scope_all_and_explicit_empty_yields_none() {
        let closure = Arc::new(DepthClosure {
            base: 0,
            probe_extra: 0,
            calls: Mutex::new(Vec::new()),
        });
        let (compiler, _) = compiler_with(closure, ScopeConfig::default());
        let mut tx = FixtureTx;
        // DepartmentScope::All（ctx_with 默认）→ 全部可见。
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let out = compiler
            .compile(&ctx, &binding_with(true, false), &mut tx)
            .await
            .expect("可编译");
        assert_eq!(out.scope, RecordScope::All);
        // Explicit 空集且无其他维度 → 不可见。
        let ctx = with_explicit_empty(ctx_with(vec!["SALES"], ClientKind::Win));
        let out = compiler
            .compile(&ctx, &binding_with(false, false), &mut tx)
            .await
            .expect("可编译");
        assert_eq!(out.scope, RecordScope::None);
    }

    fn with_subtree(ctx: SecurityContext) -> SecurityContext {
        let mut input = to_input(ctx);
        input.department_scope = DepartmentScope::Subtree(Id::from_uuid(uuid::Uuid::from_u128(99)));
        SecurityContext::human(input)
    }

    fn with_explicit_empty(ctx: SecurityContext) -> SecurityContext {
        let mut input = to_input(ctx);
        input.department_scope = DepartmentScope::Explicit(Arc::from([] as [Id<Department>; 0]));
        SecurityContext::human(input)
    }

    fn to_input(ctx: SecurityContext) -> HumanContextInput {
        HumanContextInput {
            user_id: ctx.user_id,
            session_id: ctx.session_id,
            legal_entity_id: ctx.legal_entity_id,
            device_id: ctx.device_id,
            client: ctx.client,
            clearance_level: ctx.clearance_level,
            roles: ctx.roles,
            duty_classes: ctx.duty_classes,
            department_scope: ctx.department_scope,
            position_ids: ctx.position_ids,
            project_scope: ctx.project_scope,
            customer_scope: ctx.customer_scope,
            record_shares: ctx.record_shares,
            data_scope_tags: ctx.data_scope_tags,
            snapshot_version: ctx.snapshot_version,
            is_breakglass: ctx.is_breakglass,
            request_id: ctx.request_id,
            trace_id: ctx.trace_id,
        }
    }
}
