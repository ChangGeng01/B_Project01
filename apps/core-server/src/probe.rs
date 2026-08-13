//! `ci-probe` feature 下的探针端点。发布构建不包含本文件编译产物。
//!
//! 它存在的唯一理由，是让封套、错误映射、并发闸门、同步等待上限、请求头校验、
//! 追踪与日志七条横切链路在阶段 1 就有端到端用例。没有它，这七条只有单元测试。

use std::sync::Arc;
use std::time::Duration;

use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use ep_platform_runtime::http::headers::idempotency_key_guard;
use ep_platform_runtime::http::{Envelope, SystemState};
use serde::{Deserialize, Serialize};

/// 触发 panic 捕获路径的固定取值。只在探针端点上生效。
pub const PANIC_TRIGGER: &str = "__panic__";

#[derive(Deserialize)]
pub struct EchoRequest {
    pub text: String,
    pub delay_ms: Option<u32>,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub text: String,
    pub received_at: String,
}

async fn api_v1_system_echo(
    State(_st): State<Arc<SystemState>>,
    Json(req): Json<EchoRequest>,
) -> Response {
    if req.text == PANIC_TRIGGER {
        panic!("探针端点按约定取值触发 panic，用于验证捕获路径");
    }
    if let Some(ms) = req.delay_ms {
        tokio::time::sleep(Duration::from_millis(u64::from(ms))).await;
    }
    let body = EchoResponse {
        text: req.text,
        received_at: ep_platform_obs::log::now_rfc3339_micros(),
    };
    Json(Envelope::ok(
        body,
        ep_platform_obs::TraceContext::new().trace_id().to_string(),
    ))
    .into_response()
}

pub fn router(state: Arc<SystemState>) -> Router<Arc<SystemState>> {
    Router::new()
        .route("/api/v1/system/echo", post(api_v1_system_echo))
        .route_layer(from_fn_with_state(state, idempotency_key_guard))
}
