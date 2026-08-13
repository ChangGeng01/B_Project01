//! 安全上下文的请求头推导（阶段 2 权宜，见 02 计划 §12 偏离登记）。
//!
//! 阶段 2 没有 authn/authz 基建：authz crate 为空，运行期 HTTP 层没有
//! 会话提取器。九个平台端点的主体信息改由受控请求头推导，职责类别按
//! 端点逐项校验，缺失或不符一律 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。
//! 阶段 4 交付 authz 后本文件整体替换为会话提取，调用点签名不动。
//!
//! 头与字段对应：X-EP-User-Id、X-EP-Legal-Entity-Id、X-EP-Session-Id
//! 取 UUID；X-EP-Request-Id、X-EP-Trace-Id 缺省取固定常量；
//! X-EP-Duty-Classes 逗号分隔取 SYSTEM/DATA/SECURITY/AUDIT/KEY/CONFIG；
//! X-EP-Roles 逗号分隔取全大写角色码。

use axum::http::HeaderMap;
use ep_foundation::error::codes::PLATFORM_AUTHZ_OBJECT_FORBIDDEN;
use ep_foundation::id::marker::{LegalEntity, Session, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::security::context::{
    ClientKind, DepartmentScope, DeviceId, DutyClass, HumanContextInput, RequestId, RoleCode,
    TraceId,
};
use ep_foundation::security::level::SecurityLevel;
use ep_foundation::security::SecurityContext;
use ep_platform_runtime::http::{ApiError, SystemState};
use uuid::Uuid;

/// 缺省追踪标识：32 位十六进制零串，形态与 TraceId 冻结口径一致。
const DEFAULT_TRACE_ID: &str = "00000000000000000000000000000000";

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

fn uuid_header(headers: &HeaderMap, name: &str) -> Option<Uuid> {
    header_str(headers, name).and_then(|v| v.parse().ok())
}

fn duty_of(raw: &str) -> Option<DutyClass> {
    match raw {
        "SYSTEM" => Some(DutyClass::System),
        "DATA" => Some(DutyClass::Data),
        "SECURITY" => Some(DutyClass::Security),
        "AUDIT" => Some(DutyClass::Audit),
        "KEY" => Some(DutyClass::Key),
        "CONFIG" => Some(DutyClass::Config),
        _ => None,
    }
}

/// 从请求头推导安全上下文。`required_duties` 为该端点允许的职责类别，
/// 上下文一项都不命中即 403。
pub fn extract_context(
    headers: &HeaderMap,
    state: &SystemState,
    required_duties: &[DutyClass],
) -> Result<SecurityContext, ApiError> {
    let trace_str = header_str(headers, "x-ep-trace-id")
        .filter(|v| TraceId::new(v).is_ok())
        .unwrap_or(DEFAULT_TRACE_ID);
    let forbidden = |reason: &str| {
        ApiError::new(
            PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
            state.next_incident_no(),
            trace_str.to_string(),
        )
        .with_details(vec![ep_platform_runtime::http::Detail {
            field: "security-context".into(),
            reason: reason.into(),
            value: None,
        }])
    };

    let user_id = uuid_header(headers, "x-ep-user-id").ok_or_else(|| forbidden("缺用户标识"))?;
    let legal_entity_id =
        uuid_header(headers, "x-ep-legal-entity-id").ok_or_else(|| forbidden("缺法人标识"))?;
    let session_id =
        uuid_header(headers, "x-ep-session-id").ok_or_else(|| forbidden("缺会话标识"))?;

    let duty_classes: Vec<DutyClass> = header_str(headers, "x-ep-duty-classes")
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(duty_of)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| forbidden("职责类别形态非法"))?;
    if !required_duties
        .iter()
        .any(|need| duty_classes.contains(need))
    {
        return Err(forbidden("职责类别不满足该端点要求"));
    }

    let roles: Vec<RoleCode> = header_str(headers, "x-ep-roles")
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(RoleCode::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| forbidden("角色码形态非法"))?;

    let device_id = header_str(headers, "x-ep-device-id")
        .and_then(|v| DeviceId::new(v).ok())
        .unwrap_or_else(|| DeviceId::new("platform-endpoint").expect("固定取值合法"));
    let request_id = header_str(headers, "x-ep-request-id")
        .and_then(|v| RequestId::new(v).ok())
        .unwrap_or_else(|| RequestId::new("platform-endpoint").expect("固定取值合法"));
    let trace_id = TraceId::new(trace_str).expect("上方已按合法形态过滤");

    Ok(SecurityContext::human(HumanContextInput {
        user_id: Id::<UserAccount>::from_uuid(user_id),
        session_id: Id::<Session>::from_uuid(session_id),
        legal_entity_id: Id::<LegalEntity>::from_uuid(legal_entity_id),
        device_id,
        client: ClientKind::Ops,
        clearance_level: SecurityLevel::Secret,
        roles: roles.into(),
        duty_classes: duty_classes.into(),
        department_scope: DepartmentScope::All,
        position_ids: Vec::new().into(),
        project_scope: Vec::new().into(),
        customer_scope: Vec::new().into(),
        record_shares: Vec::new().into(),
        data_scope_tags: Vec::new().into(),
        snapshot_version: 0,
        is_breakglass: false,
        request_id,
        trace_id,
    }))
}

/// 重新认证头的存在性校验（A-04/A-05/A-06/A-09）。实质验证待 authn
/// 阶段交付，本阶段只要求头存在且非空（偏离登记）。
pub fn require_reauth_token(headers: &HeaderMap, state: &SystemState) -> Result<(), ApiError> {
    match header_str(headers, "x-reauth-token") {
        Some(v) if !v.is_empty() => Ok(()),
        _ => Err(ApiError::new(
            PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
            state.next_incident_no(),
            DEFAULT_TRACE_ID.to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (k, v) in pairs {
            map.insert(
                axum::http::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                v.parse().unwrap(),
            );
        }
        map
    }

    fn state() -> Arc<SystemState> {
        use ep_platform_obs::log::{JsonLogger, Level};
        use ep_platform_obs::MetricsRegistry;
        use ep_platform_runtime::lifecycle::Lifecycle;
        use ep_platform_runtime::selfcheck::{Outcome, SelfCheckReport};
        use ep_platform_runtime::{BuildInfo, ProcessKind};
        SystemState::new(
            ProcessKind::CoreServer,
            BuildInfo::current(),
            Lifecycle::new(ProcessKind::CoreServer),
            SelfCheckReport {
                process: "core-server",
                version: "0.1.0".into(),
                items: Vec::new(),
                overall: Outcome::Passed,
            },
            Arc::new(MetricsRegistry::new()),
            Arc::new(JsonLogger::new("core-server", "0.1.0", Level::Info)),
        )
    }

    const BASE: [(&str, &str); 4] = [
        ("x-ep-user-id", "11111111-1111-7111-8111-111111111111"),
        (
            "x-ep-legal-entity-id",
            "22222222-2222-7222-8222-222222222222",
        ),
        ("x-ep-session-id", "33333333-3333-7333-8333-333333333333"),
        ("x-ep-duty-classes", "SECURITY"),
    ];

    #[test]
    fn a_complete_header_set_yields_a_human_context() {
        let ctx = extract_context(&headers(&BASE), &state(), &[DutyClass::Security])
            .expect("齐备头必须推导出上下文");
        assert_eq!(ctx.duty_classes.len(), 1);
        assert_eq!(ctx.legal_entity_id.as_uuid().to_string(), BASE[1].1);
    }

    // 负样例断言的是职责类别门禁这条规则本身：不命中即 403。
    #[test]
    fn a_missing_duty_class_is_forbidden() {
        let err = extract_context(&headers(&BASE), &state(), &[DutyClass::System])
            .expect_err("SECURITY 上下文不得通过 SYSTEM 端点");
        assert_eq!(err.code, PLATFORM_AUTHZ_OBJECT_FORBIDDEN);
    }

    #[test]
    fn a_missing_user_header_is_forbidden() {
        let err = extract_context(&headers(&BASE[1..]), &state(), &[DutyClass::Security])
            .expect_err("缺用户标识必须拒绝");
        assert_eq!(err.code, PLATFORM_AUTHZ_OBJECT_FORBIDDEN);
    }

    #[test]
    fn reauth_token_must_be_present_and_non_empty() {
        assert!(require_reauth_token(&headers(&[]), &state()).is_err());
        assert!(require_reauth_token(&headers(&[("x-reauth-token", "")]), &state()).is_err());
        assert!(require_reauth_token(&headers(&[("x-reauth-token", "t-1")]), &state()).is_ok());
    }
}
