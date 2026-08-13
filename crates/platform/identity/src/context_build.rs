//! SecurityContext 衔接：会话建立时读授权集合一次，填全 19 字段冻结。
//!
//! 唯一构造入口是 `SecurityContext::human(HumanContextInput)`
//! （foundation 冻结，context.rs L246）；本模块只做字段装配。

use std::sync::Arc;

use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::security::context::{
    DataScopeTag, DepartmentScope, DeviceId, DutyClass, HumanContextInput, RecordShare,
    RecordShareGrant, RequestId, RoleCode, TraceId,
};
use ep_foundation::security::level::SecurityLevel;
use ep_foundation::security::SecurityContext;

use crate::ports::UserAuthzSet;
use crate::types::UserAccountRow;

/// 会话上下文的装配入参（会话级事实 + 授权集合）。
pub struct SessionContextInput {
    pub account: UserAccountRow,
    pub session_id: uuid::Uuid,
    pub legal_entity_id: Id<LegalEntity>,
    pub device_id: String,
    pub client: ep_foundation::security::context::ClientKind,
    pub authz: UserAuthzSet,
    pub request_id: String,
    pub trace_id: String,
    pub is_breakglass: bool,
}

/// 构造冻结的会话安全上下文。字符集以 foundation 冻结实现为准：
/// 非法角色码/设备标识/数据范围标签逐项丢弃（登记面保证合法，防御性处理）。
pub fn build_session_context(
    input: SessionContextInput,
) -> Result<SecurityContext, ep_foundation::error::AppError> {
    let device_id = DeviceId::new(&input.device_id)?;
    let request_id = RequestId::new(&input.request_id)?;
    let trace_id = TraceId::new(&input.trace_id)?;
    let roles = input
        .authz
        .role_codes
        .iter()
        .filter_map(|r| RoleCode::new(r).ok())
        .collect::<Vec<_>>();
    let tags = input
        .authz
        .data_scope_tags
        .iter()
        .filter_map(|t| DataScopeTag::new(t).ok())
        .collect::<Vec<_>>();
    let shares = input
        .authz
        .record_shares
        .iter()
        .map(|(object_type, object_id)| RecordShare {
            object_type: Arc::from(object_type.as_str()),
            object_id: *object_id,
            // U-B-07 共享不可转授；授予形态首版取 Write（可读写该记录）。
            grant: RecordShareGrant::Write,
        })
        .collect::<Vec<_>>();
    let clearance =
        SecurityLevel::from_code(input.account.clearance_level).unwrap_or(SecurityLevel::Internal);
    Ok(SecurityContext::human(HumanContextInput {
        user_id: input.account.id,
        session_id: Id::from_uuid(input.session_id),
        legal_entity_id: input.legal_entity_id,
        device_id,
        client: input.client,
        clearance_level: clearance,
        roles: Arc::from(roles.into_boxed_slice()),
        duty_classes: Arc::from(input.authz.duty_classes.into_boxed_slice()),
        department_scope: DepartmentScope::Explicit(Arc::from(
            input.authz.department_ids.into_boxed_slice(),
        )),
        position_ids: Arc::from(input.authz.position_ids.into_boxed_slice()),
        project_scope: Arc::from(input.authz.project_ids.into_boxed_slice()),
        customer_scope: Arc::from(input.authz.customer_ids.into_boxed_slice()),
        record_shares: Arc::from(shares.into_boxed_slice()),
        data_scope_tags: Arc::from(tags.into_boxed_slice()),
        snapshot_version: input.authz.snapshot_version,
        is_breakglass: input.is_breakglass,
        request_id,
        trace_id,
    }))
}

/// 预认证上下文：sign-in 事务尚无会话，以最小字段装配 human 上下文，
/// 仅用于 UnitOfWork 的法人会话变量设置（身份九表无 RLS，判定不受影响）。
pub fn pre_auth_context(
    account: &UserAccountRow,
    device_id: &str,
    request_id: &str,
    trace_id: &str,
) -> Result<SecurityContext, ep_foundation::error::AppError> {
    let device = DeviceId::new(device_id).or_else(|_| DeviceId::new("PREAUTH"))?;
    let req = RequestId::new(request_id).or_else(|_| RequestId::new("PREAUTH0000"))?;
    let trace = TraceId::new(trace_id).or_else(|_| TraceId::new(&"0".repeat(32)))?;
    let duties: [DutyClass; 0] = [];
    Ok(SecurityContext::human(HumanContextInput {
        user_id: account.id,
        session_id: Id::from_uuid(uuid::Uuid::nil()),
        legal_entity_id: account.home_legal_entity_id,
        device_id: device,
        client: ep_foundation::security::context::ClientKind::Ops,
        clearance_level: SecurityLevel::Internal,
        roles: Arc::from([] as [RoleCode; 0]),
        duty_classes: Arc::from(duties),
        department_scope: DepartmentScope::Explicit(Arc::from([] as [Id<
            ep_foundation::id::marker::Department,
        >; 0])),
        position_ids: Arc::from([] as [Id<ep_foundation::id::marker::Position>; 0]),
        project_scope: Arc::from([] as [Id<ep_foundation::id::marker::Project>; 0]),
        customer_scope: Arc::from([] as [Id<ep_foundation::id::marker::Customer>; 0]),
        record_shares: Arc::from([] as [RecordShare; 0]),
        data_scope_tags: Arc::from([] as [DataScopeTag; 0]),
        snapshot_version: 0,
        is_breakglass: false,
        request_id: req,
        trace_id: trace,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AccountKind, AccountStatus};
    use chrono::Utc;

    fn account() -> UserAccountRow {
        UserAccountRow {
            id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            account_kind: AccountKind::Employee,
            login_name: "alice".into(),
            employee_no: None,
            display_name: "Alice".into(),
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            status: AccountStatus::Active,
            clearance_level: 30,
            security_level: 30,
            is_mfa_required: false,
            created_at: Utc::now(),
        }
    }

    fn input(authz: UserAuthzSet) -> SessionContextInput {
        SessionContextInput {
            account: account(),
            session_id: uuid::Uuid::from_u128(7),
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            device_id: "DEV-01".into(),
            client: ep_foundation::security::context::ClientKind::Win,
            authz,
            request_id: "0199aa11".into(),
            trace_id: "0".repeat(32),
            is_breakglass: false,
        }
    }

    #[test]
    fn session_context_fills_all_nineteen_fields() {
        let authz = UserAuthzSet {
            role_codes: vec!["FINANCE_CLERK".into(), "lowercase_bad".into()],
            duty_classes: vec![DutyClass::Data],
            data_scope_tags: vec!["region:APAC".into(), "bad tag".into()],
            record_shares: vec![("sales.sales_order".into(), uuid::Uuid::from_u128(9))],
            snapshot_version: 12,
            ..Default::default()
        };
        let ctx = build_session_context(input(authz)).expect("装配成功");
        assert_eq!(ctx.roles.len(), 1, "小写角色码被丢弃（冻结字符集）");
        assert_eq!(ctx.duty_classes.len(), 1);
        assert_eq!(ctx.data_scope_tags.len(), 1, "非法标签被丢弃");
        assert_eq!(ctx.record_shares.len(), 1);
        assert_eq!(ctx.snapshot_version, 12);
        assert_eq!(ctx.user_id.as_uuid(), uuid::Uuid::from_u128(1));
        assert!(!ctx.is_breakglass);
    }

    #[test]
    fn pre_auth_context_is_buildable_with_minimal_fields() {
        let ctx = pre_auth_context(&account(), "DEV-01", "0199aa11", &"0".repeat(32))
            .expect("预认证上下文");
        assert_eq!(ctx.user_id.as_uuid(), uuid::Uuid::from_u128(1));
        assert!(ctx.roles.is_empty());
        let fallback = pre_auth_context(&account(), "bad device!", "short", "zz");
        assert!(fallback.is_ok(), "非法形态回落缺省值");
    }
}
