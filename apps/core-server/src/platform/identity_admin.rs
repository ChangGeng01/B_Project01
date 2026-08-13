//! 身份域管理面与自助面端点：MFA 登记、设备凭据、账号生命周期、
//! 应急账号（规格 §6.2，任务 #21）。
//!
//! 职责门禁（临时头阶段经 extract_context 逐项校验）：
//! 自助事务六类职责任一；账号生命周期与应急提交/关闭仅 SECURITY；
//! 应急批准 SECURITY 或 AUDIT 命中（批准人判据本体在用例内校验）。
//! A-20：能力元组与路由注册逐行同行。

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use axum::{Json, Router};
use chrono::Utc;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_OBJECT_FORBIDDEN, PLATFORM_REQUEST_INVALID_PAYLOAD,
};
use ep_foundation::id::marker::{LegalEntity, UserAccount};
use ep_foundation::id::Id;
use ep_foundation::security::context::{ClientKind, DutyClass};
use ep_platform_identity::account_admin::{partial_failed_error, ImportAccountRow};
use ep_platform_identity::breakglass::BreakglassSubmit;
use ep_platform_identity::lifecycle::DeviceRegisterInput;
use ep_platform_identity::types::{AccountKind, BreakglassAction};
use ep_platform_runtime::http::headers::idempotency_key_guard;
use ep_platform_runtime::http::{ApiError, Envelope};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use super::{not_provisioned, ok_response, to_api_error, trace_of, PlatformState, RouteEntry};
use crate::wiring::context::extract_context;
use crate::wiring::identity::IdentityAssembly;

/// 自助端点职责门禁：六类职责任一命中即放行。
const ALL_DUTIES: &[DutyClass] = &[
    DutyClass::System,
    DutyClass::Data,
    DutyClass::Security,
    DutyClass::Audit,
    DutyClass::Key,
    DutyClass::Config,
];
/// 账号生命周期与应急提交/关闭：仅 SECURITY。
const SECURITY_DUTY: &[DutyClass] = &[DutyClass::Security];
/// 应急批准：SECURITY 或 AUDIT 命中（approved_by≠requested_by 与
/// duty_class 持据判据在 BreakglassService::approve 用例内校验）。
const SECURITY_OR_AUDIT: &[DutyClass] = &[DutyClass::Security, DutyClass::Audit];

fn identity_of(state: &PlatformState, trace: &str) -> Result<Arc<IdentityAssembly>, ApiError> {
    state
        .identity
        .clone()
        .ok_or_else(|| not_provisioned(state, trace))
}

fn invalid_payload(state: &PlatformState, trace: &str) -> ApiError {
    ApiError::new(
        PLATFORM_REQUEST_INVALID_PAYLOAD,
        state.system.next_incident_no(),
        trace.to_string(),
    )
}

// ───────────────────────────── MFA 登记（自助） ─────────────────────────────

/// TOTP 登记开始：返回登记引用与 base32 种子（仅此一次明文外出）。
pub async fn mfa_enrollment_begin(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let r = identity
            .enrollment
            .begin_totp(&ctx, Utc::now())
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(
                json!({
                    "enrollment_ref": r.enrollment_ref,
                    "secret_base32": r.secret_base32,
                }),
                trace.clone(),
            ),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct MfaCompleteBody {
    enrollment_ref: String,
    code: String,
}

/// TOTP 登记完成：验证码核验后落凭据。
pub async fn mfa_enrollment_complete(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<MfaCompleteBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let id = identity
            .enrollment
            .complete_totp(&ctx, &body.enrollment_ref, &body.code, Utc::now())
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"credential_id": id.to_string()}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 注销本人名下指定 MFA 凭据（最后因子禁删判据在用例内）。
pub async fn mfa_enrollment_unenroll(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let done = identity
            .enrollment
            .unenroll(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"unenrolled": done}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// ───────────────────────────── 设备与凭据（自助） ─────────────────────────────

#[derive(Deserialize)]
pub struct DeviceRegisterBody {
    device_id: String,
    /// 取值六形态（win/mac/ios/android/portal/ops），serde 小写形态。
    client: Value,
    public_key: Option<String>,
    attestation_ref: Option<String>,
    restricted_legal_entity_id: Option<Uuid>,
}

/// 登记本人设备（单法人限定可选；上下文取交集语义在用例内）。
pub async fn register_device(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<DeviceRegisterBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let client: ClientKind = serde_json::from_value(body.client.clone())
            .map_err(|_| invalid_payload(&state, &trace))?;
        let input = DeviceRegisterInput {
            device_id: body.device_id,
            client,
            public_key: body.public_key,
            attestation_ref: body.attestation_ref,
            restricted_legal_entity_id: body
                .restricted_legal_entity_id
                .map(Id::<LegalEntity>::from_uuid),
        };
        let id = identity
            .lifecycle
            .register_device(&ctx, input)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::CREATED,
            Envelope::ok(json!({"id": id.to_string()}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 远程注销本人设备（级联撤销该设备上的会话，返回撤销数）。
pub async fn revoke_device(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        let identity = identity_of(&state, &trace)?;
        let revoked = identity
            .lifecycle
            .revoke_device(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"revoked_sessions": revoked}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct ResetPasswordBody {
    /// 新口令仅存在于本请求体，任何日志不得引用。
    new_password: String,
}

/// 重置本人口令：路径账号标识必须与调用上下文一致（本人事务）。
pub async fn reset_password(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<ResetPasswordBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, ALL_DUTIES)?;
        if id != ctx.user_id.as_uuid() {
            return Err(ApiError::new(
                PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
                state.system.next_incident_no(),
                trace.clone(),
            ));
        }
        let identity = identity_of(&state, &trace)?;
        identity
            .lifecycle
            .reset_password(&ctx, &body.new_password, Utc::now())
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"reset": true}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// ───────────────────────────── 账号生命周期（管理面） ─────────────────────────────

#[derive(Deserialize)]
pub struct ImportRowBody {
    account_kind: String,
    login_name: String,
    employee_no: Option<String>,
    display_name: String,
    home_legal_entity_id: Uuid,
    clearance_level: u8,
    is_mfa_required: bool,
    /// 初始口令仅存在于本请求体，任何日志不得引用。
    initial_password: String,
}

#[derive(Deserialize)]
pub struct ImportBatchBody {
    rows: Vec<ImportRowBody>,
}

/// 请求行到用例行的解析：形态非法整批 400（尚未落库）。
fn parse_import_rows(
    body: ImportBatchBody,
    state: &PlatformState,
    trace: &str,
) -> Result<Vec<ImportAccountRow>, ApiError> {
    let mut rows = Vec::with_capacity(body.rows.len());
    for r in body.rows.into_iter() {
        let kind =
            AccountKind::parse(&r.account_kind).ok_or_else(|| invalid_payload(state, trace))?;
        rows.push(ImportAccountRow {
            account_kind: kind,
            login_name: r.login_name,
            employee_no: r.employee_no,
            display_name: r.display_name,
            home_legal_entity_id: Id::<LegalEntity>::from_uuid(r.home_legal_entity_id),
            clearance_level: r.clearance_level,
            is_mfa_required: r.is_mfa_required,
            initial_password: r.initial_password,
        });
    }
    Ok(rows)
}

/// 批量导入账号（200 行上限；逐行独立事务，失败行退回 409 明细）。
pub async fn import_batch(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<ImportBatchBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        let rows = parse_import_rows(body, &state, &trace)?;
        let outcome = identity
            .account_admin
            .import_batch(&ctx, rows)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        if let Some(err) = partial_failed_error(&outcome) {
            return Err(to_api_error(err, &state, &trace));
        }
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"imported": outcome.imported}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 激活账号（UNACTIVATED→ACTIVE；SoD 与待办判据在用例内）。
pub async fn activate_account(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        let done = identity
            .lifecycle
            .activate_account(&ctx, Id::<UserAccount>::from_uuid(id))
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"activated": done}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct TransferBody {
    to_user_id: Uuid,
}

/// 账号移交（职责归属迁移；SoD 纯函数与未结审批待办校验在用例内）。
pub async fn transfer_account(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<TransferBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        identity
            .lifecycle
            .transfer_account(
                &ctx,
                Id::<UserAccount>::from_uuid(id),
                Id::<UserAccount>::from_uuid(body.to_user_id),
            )
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        // 阶段 4 派生事件 platform.user_account.transferred.v1：
        // 移交事务已提交，写出属 3b Outbox 接缝（同阶段 2 纪律）。
        super::events::record_pending_emit(
            &state,
            super::events::USER_ACCOUNT_TRANSFERRED,
            &id.to_string(),
        );
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"transferred": true}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 停用账号（即时撤全部会话与设备凭据，登记 deactivated 事件）。
pub async fn deactivate_account(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        identity
            .lifecycle
            .deactivate(&ctx, Id::<UserAccount>::from_uuid(id))
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"deactivated": true}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// ───────────────────────────── 应急账号 ─────────────────────────────

#[derive(Deserialize)]
pub struct BreakglassSubmitBody {
    user_id: Uuid,
    reason: String,
    allowed_action_set: Vec<String>,
}

/// 提交应急启用申请（三类允许动作枚举形态在用例外先解析）。
pub async fn breakglass_submit(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<BreakglassSubmitBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        let mut actions = Vec::with_capacity(body.allowed_action_set.len());
        for raw in &body.allowed_action_set {
            actions
                .push(BreakglassAction::parse(raw).ok_or_else(|| invalid_payload(&state, &trace))?);
        }
        let input = BreakglassSubmit {
            user_id: Id::<UserAccount>::from_uuid(body.user_id),
            reason: body.reason,
            allowed_action_set: actions,
        };
        let id = identity
            .breakglass
            .submit(&ctx, input)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::CREATED,
            Envelope::ok(json!({"id": id.to_string()}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

#[derive(Deserialize)]
pub struct BreakglassApproveBody {
    approval_ref: String,
}

/// 批准应急启用（approved_by≠requested_by 与 duty_class 判据在用例内）。
pub async fn breakglass_approve(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<BreakglassApproveBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_OR_AUDIT)?;
        let identity = identity_of(&state, &trace)?;
        identity
            .breakglass
            .approve(&ctx, id, &body.approval_ref)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"approved": true}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

/// 关闭应急窗口（提前收口；到期失效由 job-worker 后台承接）。
pub async fn breakglass_close(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, SECURITY_DUTY)?;
        let identity = identity_of(&state, &trace)?;
        identity
            .breakglass
            .close(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        // 阶段 4 派生事件 platform.breakglass_activation.closed.v1：
        // 关闭与凭据轮换事务已提交，写出属 3b Outbox 接缝。
        super::events::record_pending_emit(
            &state,
            super::events::BREAKGLASS_ACTIVATION_CLOSED,
            &id.to_string(),
        );
        Ok(ok_response(
            StatusCode::OK,
            Envelope::ok(json!({"closed": true}), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// ───────────────────────────── 路由 ─────────────────────────────

/// 管理面与自助面路由：全部为写动作，统一挂幂等键守卫层。
/// A-20：能力元组与路由注册逐行同行。
pub fn router(state: Arc<PlatformState>) -> Router {
    let writes: [RouteEntry; 13] = [
        (
            "/api/v1/platform/mfa-enrollments/actions/begin",
            post(mfa_enrollment_begin),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/mfa-enrollments/actions/complete",
            post(mfa_enrollment_complete),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/mfa-enrollments/{id}",
            delete(mfa_enrollment_unenroll),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/devices",
            post(register_device),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/devices/{id}/actions/revoke",
            post(revoke_device),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/user-accounts/{id}/actions/reset-password",
            post(reset_password),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/user-accounts/actions/import-batch",
            post(import_batch),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/user-accounts/{id}/actions/activate",
            post(activate_account),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/user-accounts/{id}/actions/transfer",
            post(transfer_account),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/user-accounts/{id}/actions/deactivate",
            post(deactivate_account),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/breakglass-activations",
            post(breakglass_submit),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Submit,
            ),
        ),
        (
            "/api/v1/platform/breakglass-activations/{id}/actions/approve",
            post(breakglass_approve),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Approve,
            ),
        ),
        (
            "/api/v1/platform/breakglass-activations/{id}/actions/close",
            post(breakglass_close),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
    ];
    let mut router = Router::new();
    for (path, handler, _capability) in writes {
        router = router.route(path, handler);
    }
    router
        .layer(from_fn_with_state(
            state.system.clone(),
            idempotency_key_guard,
        ))
        .with_state(state)
}
