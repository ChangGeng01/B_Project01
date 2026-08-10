//! 系统端点。全部只监听回环地址，不承载业务数据。
//!
//! 两套形态：core-server 与 portal-gateway 走统一封套，其余进程走
//! `/healthz`、`/readyz`、`/metrics` 的最小形态。形态差别来自阶段 1 计划
//! 第 6.1 与 6.2 节，不是随手取的。

use std::sync::Arc;

use axum::extract::{FromRef, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use ep_foundation::error::codes::PLATFORM_SYSTEM_NOT_READY;
use serde::Serialize;

use super::envelope::{ApiError, Envelope};
use super::state::SystemState;

pub type Shared = Arc<SystemState>;

// 路由构造函数对状态类型只有一个要求：能从中取出 [`Shared`]。写成泛型而不是
// 写死 `Shared`，是为了让 portal-gateway 那样另有上游配置的进程把系统端点与
// 自有端点装在同一个 Router 上，而不是为此另起一个监听。

#[derive(Serialize)]
struct HealthData {
    status: &'static str,
    process: &'static str,
    version: &'static str,
    started_at: String,
}

#[derive(Serialize)]
struct ReadyData {
    state: &'static str,
    pending_items: usize,
}

#[derive(Serialize)]
struct VersionData {
    version: &'static str,
    git_commit: &'static str,
    source_date_epoch: &'static str,
    migration_manifest_sha256: &'static str,
    api_major: u8,
}

fn trace_id() -> String {
    ep_platform_obs::TraceContext::new().trace_id().to_string()
}

fn not_ready(st: &SystemState) -> ApiError {
    ApiError::new(PLATFORM_SYSTEM_NOT_READY, st.next_incident_no(), trace_id())
}

async fn health(State(st): State<Shared>) -> Response {
    let data = HealthData {
        status: "UP",
        process: st.process().name(),
        version: st.build().version,
        started_at: st.started_at().to_string(),
    };
    axum::Json(Envelope::ok(data, trace_id())).into_response()
}

async fn ready(State(st): State<Shared>) -> Response {
    if !st.is_serving() {
        return not_ready(&st).into_response();
    }
    let data = ReadyData {
        // 降级状态必须在就绪端点上显形，PRD 11.9 要求用户可见。
        state: if st.state() == crate::lifecycle::State::Degraded { "DEGRADED" } else { "READY" },
        pending_items: st.report().pending_items(),
    };
    axum::Json(Envelope::ok(data, trace_id())).into_response()
}

async fn version(State(st): State<Shared>) -> Response {
    let b = st.build();
    let data = VersionData {
        version: b.version,
        git_commit: b.git_commit,
        source_date_epoch: b.source_date_epoch,
        migration_manifest_sha256: b.migration_manifest_sha256,
        api_major: 1,
    };
    axum::Json(Envelope::ok(data, trace_id())).into_response()
}

async fn self_check(State(st): State<Shared>) -> Response {
    let report = st.report();
    let body = serde_json::to_value(report).unwrap_or(serde_json::Value::Null);
    if report.overall == crate::selfcheck::Outcome::Passed {
        return axum::Json(Envelope::ok(body, trace_id())).into_response();
    }
    // 未通过时仍要把报告交出去：拿不到报告的运维只能去翻日志。
    let err = not_ready(&st);
    let mut envelope = err.body();
    envelope.meta = Some(body);
    (err.status(), axum::Json(envelope)).into_response()
}

pub async fn metrics_text(State(st): State<Shared>) -> Response {
    let text = st.metrics().encode_text();
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")], text).into_response()
}

async fn healthz(State(st): State<Shared>) -> Response {
    (StatusCode::OK, format!("UP {} {}\n", st.process().name(), st.build().version)).into_response()
}

async fn readyz(State(st): State<Shared>) -> Response {
    if st.is_serving() {
        (StatusCode::OK, format!("{}\n", st.state().as_str())).into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, format!("{}\n", st.state().as_str())).into_response()
    }
}

/// 路由不存在时统一返回 404 与 `PLATFORM.ROUTE.NOT_FOUND`，
/// 不用框架默认的空 404，否则四端拿不到可读的错误形态。
pub async fn fallback(State(st): State<Shared>) -> Response {
    ApiError::new(ep_foundation::error::codes::PLATFORM_ROUTE_NOT_FOUND, st.next_incident_no(), trace_id())
        .into_response()
}

/// core-server 的五个系统端点。
///
/// 四个构造函数一律返回未绑定状态的 `Router<Shared>`：状态在 apps 装配完
/// 中间件与 fallback 之后一次绑定，中途绑定会让 fallback 拿不到状态。
pub fn core_system_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Shared: FromRef<S>,
{
    Router::new()
        .route("/api/v1/system/health", get(health))
        .route("/api/v1/system/ready", get(ready))
        .route("/api/v1/system/version", get(version))
        .route("/api/v1/system/self-check", get(self_check))
        .route("/api/v1/system/metrics", get(metrics_text))
}

/// job-worker 与 integration-gateway 的最小形态。
pub fn minimal_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Shared: FromRef<S>,
{
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics_text))
}

/// ops-agent 的健康聚合端口（9102）。指标端口（9101）由 ops-agent 自行装配。
pub fn ops_health_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Shared: FromRef<S>,
{
    Router::new().route("/healthz", get(healthz)).route("/readyz", get(readyz))
}

/// portal-gateway 的两个自有端点；`upstream` 由 portal-gateway 自行装配。
pub fn portal_system_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Shared: FromRef<S>,
{
    Router::new()
        .route("/portal/v1/system/health", get(health))
        .route("/portal/v1/system/metrics", get(metrics_text))
}
