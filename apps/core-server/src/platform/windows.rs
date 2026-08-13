//! A-09 迁移窗口开闭（02 计划 §5）。
//!
//! 开窗与关窗共用一对能力常量（同一用例「迁移窗口控制」）。
//! 职责类别门禁：SYSTEM；两端点均要求重新认证头。
//! 双人审批判定经端口调用阶段 4，未装配前 `approval_ref` 只做
//! 存在性校验（偏离登记），开窗与关窗的审计写入待审计阶段端口。

use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::security::context::DutyClass;
use ep_platform_runtime::http::{ApiError, Envelope};
use ep_platform_tenancy::capability as cap;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::{events, not_provisioned, ok_response, to_api_error, trace_of, PlatformState};
use crate::wiring::context::{extract_context, require_reauth_token};

/// 引用 capability 登记（A-20）：两条路由共用一对常量。
#[allow(dead_code)]
const CAPABILITY_BINDING: (CapabilityDomain, ActionClass) = (
    cap::MIGRATION_WINDOW_CONTROL_DOMAIN,
    cap::MIGRATION_WINDOW_CONTROL_ACTION,
);

fn invalid_payload(state: &PlatformState, trace: &str, field: &str, reason: &str) -> ApiError {
    ApiError::new(
        ep_foundation::error::codes::PLATFORM_REQUEST_INVALID_PAYLOAD,
        state.system.next_incident_no(),
        trace.to_string(),
    )
    .with_details(vec![ep_platform_runtime::http::Detail {
        field: field.into(),
        reason: reason.into(),
        value: None,
    }])
}

#[derive(Deserialize)]
pub struct OpenWindowBody {
    approval_ref: String,
    reason: String,
    ttl_minutes: u32,
}

pub async fn open_window(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<OpenWindowBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::System])?;
        require_reauth_token(&headers, &state.system)?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        if body.approval_ref.trim().is_empty() {
            return Err(invalid_payload(
                &state,
                &trace,
                "approval_ref",
                "审批引用不得为空",
            ));
        }
        if body.reason.trim().is_empty() {
            return Err(invalid_payload(
                &state,
                &trace,
                "reason",
                "开窗理由不得为空",
            ));
        }
        if body.ttl_minutes == 0 || body.ttl_minutes > state.window_ttl_max_min {
            return Err(invalid_payload(
                &state,
                &trace,
                "ttl_minutes",
                "存续时长须在 1 与配置上限之间",
            ));
        }
        let opened = db
            .windows
            .open(&ctx, body.approval_ref, body.reason, body.ttl_minutes)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        events::record_pending_emit(
            &state,
            events::MIGRATION_WINDOW_OPENED,
            &opened.id.to_string(),
        );
        Ok(ok_response(
            axum::http::StatusCode::CREATED,
            Envelope::ok(
                json!({
                    "window_id": opened.id.to_string(),
                    "expires_at": opened.expires_at.to_rfc3339(),
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
pub struct CloseWindowBody {
    window_id: Uuid,
}

pub async fn close_window(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<CloseWindowBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::System])?;
        require_reauth_token(&headers, &state.system)?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        db.windows
            .close(&ctx, body.window_id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        Ok(ok_response(
            axum::http::StatusCode::OK,
            Envelope::ok(
                json!({"window_id": body.window_id.to_string()}),
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
