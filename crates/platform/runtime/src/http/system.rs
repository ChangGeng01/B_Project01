//! 系统端点。全部只监听回环地址，不承载业务数据。
//!
//! 两套形态：core-server 与 portal-gateway 走统一封套，其余进程走
//! `/healthz`、`/readyz`、`/metrics` 的最小形态。形态差别来自阶段 1 计划
//! 第 6.1 与 6.2 节，不是随手取的。

use std::sync::Arc;

use axum::extract::{Extension, FromRef, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use ep_foundation::error::codes::PLATFORM_SYSTEM_NOT_READY;
use serde::Serialize;

use super::envelope::{ApiError, Envelope};
use super::request_meta::RequestMeta;
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

fn trace_id(meta: Option<Extension<RequestMeta>>) -> String {
    meta.map(|Extension(meta)| meta.trace_id.as_str().to_string())
        // 正常装配由最外层 observe 确保 RequestMeta 存在；固定零串只是
        // 防御未经该栈直接调用 handler 时的失败关闭值，不再制造第二个 trace。
        .unwrap_or_else(|| "00000000000000000000000000000000".to_string())
}

fn not_ready(st: &SystemState, trace_id: &str) -> ApiError {
    ApiError::new(
        PLATFORM_SYSTEM_NOT_READY,
        st.next_incident_no(),
        trace_id.to_string(),
    )
}

async fn health(State(st): State<Shared>, meta: Option<Extension<RequestMeta>>) -> Response {
    let trace_id = trace_id(meta);
    let data = HealthData {
        status: "UP",
        process: st.process().name(),
        version: st.build().version,
        started_at: st.started_at().to_string(),
    };
    axum::Json(Envelope::ok(data, trace_id)).into_response()
}

async fn ready(State(st): State<Shared>, meta: Option<Extension<RequestMeta>>) -> Response {
    let trace_id = trace_id(meta);
    if !st.is_serving() {
        return not_ready(&st, &trace_id).into_response();
    }
    let data = ReadyData {
        // 降级状态必须在就绪端点上显形，PRD 11.9 要求用户可见。
        state: if st.state() == crate::lifecycle::State::Degraded {
            "DEGRADED"
        } else {
            "READY"
        },
        pending_items: st.report().pending_items(),
    };
    axum::Json(Envelope::ok(data, trace_id)).into_response()
}

async fn version(State(st): State<Shared>, meta: Option<Extension<RequestMeta>>) -> Response {
    let trace_id = trace_id(meta);
    let b = st.build();
    let data = VersionData {
        version: b.version,
        git_commit: b.git_commit,
        source_date_epoch: b.source_date_epoch,
        migration_manifest_sha256: b.migration_manifest_sha256,
        api_major: 1,
    };
    axum::Json(Envelope::ok(data, trace_id)).into_response()
}

async fn self_check(State(st): State<Shared>, meta: Option<Extension<RequestMeta>>) -> Response {
    let trace_id = trace_id(meta);
    let report = st.report();
    let body = serde_json::to_value(report).unwrap_or(serde_json::Value::Null);
    if report.overall == crate::selfcheck::Outcome::Passed {
        return axum::Json(Envelope::ok(body, trace_id)).into_response();
    }
    // 未通过时仍要把报告交出去：拿不到报告的运维只能去翻日志。
    let err = not_ready(&st, &trace_id);
    let mut envelope = err.body();
    envelope.meta = Some(body);
    (err.status(), axum::Json(envelope)).into_response()
}

pub async fn metrics_text(State(st): State<Shared>) -> Response {
    let text = st.metrics().encode_text();
    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        text,
    )
        .into_response()
}

async fn healthz(State(st): State<Shared>) -> Response {
    (
        StatusCode::OK,
        format!("UP {} {}\n", st.process().name(), st.build().version),
    )
        .into_response()
}

async fn readyz(State(st): State<Shared>) -> Response {
    if st.is_serving() {
        (StatusCode::OK, format!("{}\n", st.state().as_str())).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("{}\n", st.state().as_str()),
        )
            .into_response()
    }
}

/// 路由不存在时统一返回 404 与 `PLATFORM.ROUTE.NOT_FOUND`，
/// 不用框架默认的空 404，否则四端拿不到可读的错误形态。
pub async fn fallback(State(st): State<Shared>, meta: Option<Extension<RequestMeta>>) -> Response {
    ApiError::new(
        ep_foundation::error::codes::PLATFORM_ROUTE_NOT_FOUND,
        st.next_incident_no(),
        trace_id(meta),
    )
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
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::middleware::from_fn_with_state;
    use ep_foundation::security::context::{RequestId, TraceId};
    use ep_platform_obs::log::{JsonLogger, Level};
    use ep_platform_obs::MetricsRegistry;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::http::middleware::observe;
    use crate::http::RequestMeta;
    use crate::lifecycle::Lifecycle;
    use crate::process::{BuildInfo, ProcessKind};
    use crate::selfcheck::{Outcome, SelfCheckReport};

    const TEST_TRACE: &str = "abcdef0123456789abcdef0123456789";

    fn system() -> Shared {
        SystemState::new(
            ProcessKind::CoreServer,
            BuildInfo::current(),
            Lifecycle::new(ProcessKind::CoreServer),
            SelfCheckReport {
                process: "core-server",
                version: "test".into(),
                items: Vec::new(),
                overall: Outcome::Passed,
            },
            Arc::new(MetricsRegistry::new()),
            Arc::new(JsonLogger::new("core-server", "test", Level::Info)),
        )
    }

    fn request(uri: &str) -> Request {
        let mut request = Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("构造请求");
        request.extensions_mut().insert(RequestMeta {
            client_ip: "192.0.2.11".parse().expect("固定 IP 合法"),
            request_id: RequestId::new("system-http-test").expect("固定请求标识合法"),
            trace_id: TraceId::new(TEST_TRACE).expect("固定追踪标识合法"),
        });
        request
    }

    async fn trace_of_response(response: Response) -> String {
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("响应体可读")
            .to_bytes();
        serde_json::from_slice::<serde_json::Value>(&bytes).expect("系统封套必须是 JSON")
            ["trace_id"]
            .as_str()
            .expect("系统封套必须有 trace_id")
            .to_string()
    }

    #[tokio::test]
    async fn system_and_fallback_envelopes_reuse_request_meta_and_each_record_once() {
        let st = system();
        let app = core_system_router()
            .fallback(fallback)
            .with_state(st.clone())
            .layer(from_fn_with_state(st.clone(), observe));

        let health = app
            .clone()
            .oneshot(request("/api/v1/system/health"))
            .await
            .expect("健康端点应返回响应");
        assert_eq!(health.status(), StatusCode::OK);
        assert_eq!(trace_of_response(health).await, TEST_TRACE);

        let missing = app
            .oneshot(request("/api/v1/not-real/customer-123"))
            .await
            .expect("404 应返回统一响应");
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
        assert_eq!(trace_of_response(missing).await, TEST_TRACE);

        let metrics = st.metrics().encode_text();
        assert!(metrics.contains(
            "ep_http_request_duration_seconds_count{route=\"/api/v1/system/health\",method=\"GET\",status_class=\"2xx\",client=\"ops\"} 1"
        ));
        assert!(metrics.contains(
            "ep_http_request_duration_seconds_count{route=\"<unmatched>\",method=\"GET\",status_class=\"4xx\",client=\"ops\"} 1"
        ));
        assert!(
            !metrics.contains("customer-123"),
            "404 实例路径不得进入指标标签"
        );
    }
}
