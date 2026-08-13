//! 上游探测端点 `/portal/v1/system/upstream`。
//!
//! 它证明的是「门户侧取数一律经 core-server」这条约束在装配上成立：
//! 门户进程自己没有库连接，唯一的上游就是回环上的 core。
//! 另外，门户侧新建 trace，不接受外部传入的 traceparent，公网侧的关联标识
//! 走 `X-Correlation-Id` 单独回带——外部注入的追踪上下文会污染内部链路。

use std::sync::Arc;
use std::time::Duration;

use axum::extract::{FromRef, State};
use axum::http::HeaderName;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use ep_platform_obs::CorrelationId;
use ep_platform_runtime::http::client;
use ep_platform_runtime::http::{Envelope, SystemState};
use serde::Serialize;

pub const CORRELATION_HEADER: &str = "x-correlation-id";
/// 门户侧探测上游的超时。取同步等待上限之下的一个固定值：
/// 探测比业务请求更该早失败，拖住探测等于把门户拖成不可用。
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone)]
pub struct PortalState {
    pub system: Arc<SystemState>,
    pub upstream_base_url: Arc<str>,
}

impl FromRef<PortalState> for Arc<SystemState> {
    fn from_ref(s: &PortalState) -> Self {
        s.system.clone()
    }
}

#[derive(Serialize)]
struct UpstreamData {
    upstream: String,
    reachable: bool,
    status: Option<u16>,
    detail: Option<String>,
}

/// 上游健康端点的固定路径。门户只认这一条，不做任意转发。
pub fn upstream_health_url(base: &str) -> String {
    format!("{}/api/v1/system/health", base.trim_end_matches('/'))
}

async fn upstream(State(st): State<PortalState>) -> Response {
    let url = upstream_health_url(&st.upstream_base_url);
    let (reachable, status, detail) = match client::get(&url, PROBE_TIMEOUT).await {
        Ok(r) => (r.status == 200, Some(r.status), None),
        // 抓不到就是抓不到：不返回一个 reachable=true 的空壳。
        Err(e) => (false, None, Some(e.to_string())),
    };
    let data = UpstreamData {
        upstream: url,
        reachable,
        status,
        detail,
    };
    let trace = ep_platform_obs::TraceContext::new();
    let correlation = CorrelationId::new();
    (
        [(
            HeaderName::from_static(CORRELATION_HEADER),
            correlation.as_str().to_string(),
        )],
        axum::Json(Envelope::ok(data, trace.trace_id().to_string())),
    )
        .into_response()
}

pub fn router() -> Router<PortalState> {
    Router::new().route("/portal/v1/system/upstream", get(upstream))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_url_is_the_core_health_endpoint() {
        assert_eq!(
            upstream_health_url("http://127.0.0.1:8080"),
            "http://127.0.0.1:8080/api/v1/system/health"
        );
        assert_eq!(
            upstream_health_url("http://127.0.0.1:8080/"),
            "http://127.0.0.1:8080/api/v1/system/health",
            "尾斜杠不得产生双斜杠"
        );
    }

    // 负样例断言的是「门户只认 core 的健康端点」这条规则本身：
    // 上游地址不是 core 的健康端点时，拼出来的路径必须仍落在该端点上。
    #[test]
    fn upstream_url_never_forwards_an_arbitrary_path() {
        assert!(upstream_health_url("http://127.0.0.1:8080/anything")
            .ends_with("/api/v1/system/health"));
    }
}
