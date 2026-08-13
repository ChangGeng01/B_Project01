//! 身份域端点：会话身份七端点与门户三端点（规格 §6.2，任务 #21）。
//!
//! sign-in/complete-mfa/门户 sign-in 属 PRE_AUTH 白名单：不经
//! extract_context 推导上下文（登录前无会话），幂等键守卫豁免。
//! 明文会话令牌只在登录响应出现一次（SignInSuccess 直出）。
//! A-20：能力元组与路由注册同行，缺失即编译不过。

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_ACCOUNT_INACTIVE, PLATFORM_AUTHN_ACCOUNT_LOCKED,
    PLATFORM_AUTHN_CREDENTIAL_INVALID, PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
    PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED, PLATFORM_AUTHN_MFA_INVALID,
    PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN, PLATFORM_AUTHN_RATE_LIMITED,
};
use ep_foundation::error::ErrorCode;
use ep_foundation::security::context::DutyClass;
use ep_platform_identity::login::{
    CompleteMfaRequest, SecondFactorProof, SignInOutcome, SignInRequest, SignInSuccess,
};
use ep_platform_identity::types::{AccountKind, SessionRow, UserAccountRow};
use ep_platform_runtime::http::headers::idempotency_key_guard;
use ep_platform_runtime::http::{ApiError, Envelope};
use serde::Deserialize;
use serde_json::{json, Value};

use super::{not_provisioned, ok_response, to_api_error, trace_of, PlatformState, RouteEntry};
use crate::wiring::context::extract_context;
use crate::wiring::identity::IdentityAssembly;

/// 自助端点职责门禁：六类职责任一命中即放行（本人事务不细分）。
const ALL_DUTIES: &[DutyClass] = &[
    DutyClass::System,
    DutyClass::Data,
    DutyClass::Security,
    DutyClass::Audit,
    DutyClass::Key,
    DutyClass::Config,
];

fn identity_of(state: &PlatformState, trace: &str) -> Result<Arc<IdentityAssembly>, ApiError> {
    state
        .identity
        .clone()
        .ok_or_else(|| not_provisioned(state, trace))
}

fn source_addr_of(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

fn request_id_of(headers: &HeaderMap) -> String {
    headers
        .get("x-ep-request-id")
        .and_then(|v| v.to_str().ok())
        .filter(|v| v.len() >= 8)
        .unwrap_or("platform-signin")
        .to_string()
}

fn bearer_of(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_string)
}

fn sign_in_json(s: &SignInSuccess) -> Value {
    json!({
        "session_token": s.session_token,
        "session_id": s.session_id.to_string(),
        "user_id": s.user_id.to_string(),
        "active_legal_entity_id": s.active_legal_entity_id.as_uuid().to_string(),
        "expires_at": s.expires_at.to_rfc3339(),
        "idle_expires_at": s.idle_expires_at.to_rfc3339(),
        "is_breakglass": s.is_breakglass,
    })
}

fn account_json(row: &UserAccountRow) -> Value {
    json!({
        "id": row.id.as_uuid().to_string(),
        "account_kind": row.account_kind.as_str(),
        "login_name": row.login_name,
        "display_name": row.display_name,
        "home_legal_entity_id": row.home_legal_entity_id.as_uuid().to_string(),
        "status": row.status.as_str(),
        "is_mfa_required": row.is_mfa_required,
    })
}

fn session_json(row: &SessionRow) -> Value {
    json!({
        "id": row.id.to_string(),
        "user_device_row_id": row.user_device_row_id.to_string(),
        "active_legal_entity_id": row.active_legal_entity_id.as_uuid().to_string(),
        "issued_at": row.issued_at.to_rfc3339(),
        "expires_at": row.expires_at.to_rfc3339(),
        "idle_expires_at": row.idle_expires_at.to_rfc3339(),
        "last_seen_at": row.last_seen_at.to_rfc3339(),
        "is_breakglass": row.is_breakglass,
    })
}

/// 登录错误码到登录结果八值的指标标签形态映射（小写下划线，
/// 取值集与 `LoginAttemptOutcome` 逐项一致）。挑战过期与末因子
/// 禁入都属第二因子失败，归 mfa_invalid；限流拒入归准入拒绝类。
/// 映射不到的码返回 None，不新造标签取值。
fn login_outcome_label(code: &ErrorCode) -> Option<&'static str> {
    if *code == PLATFORM_AUTHN_CREDENTIAL_INVALID {
        return Some("credential_invalid");
    }
    if *code == PLATFORM_AUTHN_ACCOUNT_LOCKED {
        return Some("account_locked");
    }
    if *code == PLATFORM_AUTHN_ACCOUNT_INACTIVE {
        return Some("account_inactive");
    }
    if *code == PLATFORM_AUTHN_MFA_INVALID
        || *code == PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED
        || *code == PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN
    {
        return Some("mfa_invalid");
    }
    if *code == PLATFORM_AUTHN_DEVICE_NOT_REGISTERED {
        return Some("device_unregistered");
    }
    if *code == PLATFORM_AUTHN_RATE_LIMITED {
        return Some("admission_rejected");
    }
    None
}

/// 填充 ep_authn_login_attempts_total：认证中间件载体缺位即不填
/// （unwired-absent，指标注册表与认证装配同进同出）。
fn count_login_attempt(state: &PlatformState, outcome: &'static str) {
    if let Some(authn) = state.authn.as_ref() {
        let _ = authn.metrics.inc_counter(
            "ep_authn_login_attempts_total",
            &[("outcome", outcome)],
            1.0,
        );
    }
}

#[derive(Deserialize)]
pub struct SignInBody {
    login_name: String,
    /// 口令仅存在于本请求体，任何日志不得引用。
    password: String,
    device_id: String,
    client: String,
}

async fn do_sign_in(
    state: &Arc<PlatformState>,
    headers: &HeaderMap,
    body: SignInBody,
    expected_kind: Option<AccountKind>,
) -> Result<Response, ApiError> {
    let trace = trace_of(headers);
    let identity = identity_of(state, &trace)?;
    let req = SignInRequest {
        login_name: body.login_name,
        password: body.password,
        device_id: body.device_id,
        client: body.client,
        source_addr: source_addr_of(headers),
        request_id: request_id_of(headers),
        trace_id: trace.clone(),
        expected_kind,
    };
    let outcome = identity.login.sign_in(req, Utc::now()).await.map_err(|e| {
        // 锁定事实已在登录事务内落库；阶段 4 派生事件
        // platform.user_account.locked.v1 在此登记发生，
        // 写出属 3b Outbox 接缝（同阶段 2 纪律）。主体取
        // 固定字样，登录名不进日志。
        if e.code == PLATFORM_AUTHN_ACCOUNT_LOCKED {
            super::events::record_pending_emit(
                state,
                super::events::USER_ACCOUNT_LOCKED,
                "account_lockouts",
            );
        }
        if let Some(label) = login_outcome_label(&e.code) {
            count_login_attempt(state, label);
        }
        to_api_error(e, state, &trace)
    })?;
    match outcome {
        SignInOutcome::Authenticated(s) => {
            count_login_attempt(state, "success");
            Ok(ok_response(
                StatusCode::OK,
                Envelope::ok(sign_in_json(&s), trace.clone()),
            ))
        }
        SignInOutcome::MfaRequired { challenge } => {
            count_login_attempt(state, "mfa_required");
            Ok(ok_response(
                StatusCode::OK,
                Envelope::ok(
                    json!({"mfa_required": true, "challenge": challenge}),
                    trace.clone(),
                ),
            ))
        }
    }
}

/// 运维面 sign-in（PRE_AUTH；幂等豁免）。
pub async fn sign_in(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<SignInBody>,
) -> Response {
    match do_sign_in(&state, &headers, body, None).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 门户面 sign-in（PRE_AUTH；强制账号形态 PORTAL）。
pub async fn portal_sign_in(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<SignInBody>,
) -> Response {
    match do_sign_in(&state, &headers, body, Some(AccountKind::Portal)).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct CompleteMfaBody {
    challenge: String,
    /// TOTP 一次性码（第二因子形态首版仅 TOTP）。
    code: String,
}

/// complete-mfa（PRE_AUTH；幂等豁免）。
pub async fn complete_mfa(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<CompleteMfaBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let identity = identity_of(&state, &trace)?;
        let req = CompleteMfaRequest {
            challenge: body.challenge,
            proof: SecondFactorProof::Totp { code: body.code },
            source_addr: source_addr_of(&headers),
            request_id: request_id_of(&headers),
            trace_id: trace.clone(),
        };
        let s = identity
            .login
            .complete_mfa(req, Utc::now())
            .await
            .map_err(|e| {
                if let Some(label) = login_outcome_label(&e.code) {
                    count_login_attempt(&state, label);
                }
                to_api_error(e, &state, &trace)
            })?;
        count_login_attempt(&state, "success");
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(sign_in_json(&s), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

async fn do_sign_out(
    state: &Arc<PlatformState>,
    headers: &HeaderMap,
) -> Result<Response, ApiError> {
    let trace = trace_of(headers);
    let ctx = extract_context(headers, &state.system, ALL_DUTIES)?;
    let identity = identity_of(state, &trace)?;
    let token = bearer_of(headers).ok_or_else(|| {
        ApiError::new(
            ep_foundation::error::codes::PLATFORM_REQUEST_INVALID_PAYLOAD,
            state.system.next_incident_no(),
            trace.clone(),
        )
    })?;
    let revoked = identity
        .lifecycle
        .sign_out(&ctx, &token)
        .await
        .map_err(|e| to_api_error(e, state, &trace))?;
    Ok(ok_response(
        StatusCode::OK,
        Envelope::ok(json!({"revoked": revoked}), trace.clone()),
    ))
}

/// 运维面 sign-out（Bearer 令牌定位会话）。
pub async fn sign_out(State(state): State<Arc<PlatformState>>, headers: HeaderMap) -> Response {
    match do_sign_out(&state, &headers).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 门户面 sign-out。
pub async fn portal_sign_out(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
) -> Response {
    match do_sign_out(&state, &headers).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

async fn do_me(state: &Arc<PlatformState>, headers: &HeaderMap) -> Result<Response, ApiError> {
    let trace = trace_of(headers);
    let ctx = extract_context(headers, &state.system, ALL_DUTIES)?;
    let identity = identity_of(state, &trace)?;
    let row = identity
        .lifecycle
        .me(&ctx)
        .await
        .map_err(|e| to_api_error(e, state, &trace))?;
    Ok(ok_response(
        StatusCode::OK,
        Envelope::ok(account_json(&row), trace.clone()),
    ))
}

/// 运维面 identity/me。
pub async fn me(State(state): State<Arc<PlatformState>>, headers: HeaderMap) -> Response {
    match do_me(&state, &headers).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 门户面 identity/me。
pub async fn portal_me(State(state): State<Arc<PlatformState>>, headers: HeaderMap) -> Response {
    match do_me(&state, &headers).await {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// me/legal-entities：逐法人探测（不 OR 展开），PRE_AUTH 白名单豁免
/// 幂等键（GET 天然豁免）；临时头阶段仍需用户标识头推导上下文。
pub async fn me_legal_entities(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let les = identity
            .lifecycle
            .me_legal_entities(&ctx)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let items: Vec<Value> = les
            .iter()
            .map(|le| Value::String(le.as_uuid().to_string()))
            .collect();
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(Value::Array(items), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 本人会话清单（令牌摘要不外出）。
pub async fn list_sessions(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let rows = identity
            .lifecycle
            .list_my_sessions(&ctx)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let items: Vec<Value> = rows.iter().map(session_json).collect();
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(Value::Array(items), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 撤销本人名下指定会话。
pub async fn revoke_session(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let revoked = identity
            .lifecycle
            .revoke_my_session(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"revoked": revoked}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 会话身份与门户路由。A-20：能力元组与路由注册逐行同行。
pub fn router(state: Arc<PlatformState>) -> Router {
    // PRE_AUTH 白名单：登录三段不经上下文推导与幂等守卫。
    let pre_auth: [RouteEntry; 3] = [
        (
            "/api/v1/platform/sessions/actions/sign-in",
            post(sign_in),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/sessions/actions/complete-mfa",
            post(complete_mfa),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/portal/sessions/actions/sign-in",
            post(portal_sign_in),
            (CapabilityDomain::PortalSupplierWeb, ActionClass::Submit),
        ),
    ];
    let reads: [RouteEntry; 4] = [
        (
            "/api/v1/platform/identity/me",
            get(me),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/identity/me/legal-entities",
            get(me_legal_entities),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/sessions",
            get(list_sessions),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/portal/identity/me",
            get(portal_me),
            (CapabilityDomain::PortalSupplierWeb, ActionClass::Read),
        ),
    ];
    let writes: [RouteEntry; 3] = [
        (
            "/api/v1/platform/sessions/actions/sign-out",
            post(sign_out),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/sessions/{id}/actions/revoke",
            post(revoke_session),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/portal/sessions/actions/sign-out",
            post(portal_sign_out),
            (CapabilityDomain::PortalSupplierWeb, ActionClass::Write),
        ),
    ];
    let mut router = Router::new();
    for (path, handler, _capability) in pre_auth {
        router = router.route(path, handler);
    }
    let mut read_router = Router::new();
    for (path, handler, _capability) in reads {
        read_router = read_router.route(path, handler);
    }
    let mut write_router = Router::new();
    for (path, handler, _capability) in writes {
        write_router = write_router.route(path, handler);
    }
    let write_router = write_router
        .layer(from_fn_with_state(
            state.system.clone(),
            idempotency_key_guard,
        ))
        .with_state(state.clone());
    router
        .merge(read_router.with_state(state.clone()))
        .merge(write_router)
        .with_state(state)
}
