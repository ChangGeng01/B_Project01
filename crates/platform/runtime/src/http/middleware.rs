//! 中间件栈：访问日志与指标、并发闸门、同步等待上限、panic 捕获。
//!
//! 四层都做成 tower 层而不是写进每个处理器：横切关注点一旦下放到处理器，
//! 新增一条路由就多一次漏掉的机会。

use std::future::{poll_fn, Future};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::task::Poll;
use std::time::{Duration, Instant};

use axum::extract::{MatchedPath, Request, State};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use ep_foundation::error::codes::{
    PLATFORM_CAPACITY_CONCURRENCY_LIMIT, PLATFORM_SYSTEM_INTERNAL_ERROR,
    PLATFORM_SYSTEM_SYNC_TIMEOUT,
};
use ep_platform_obs::log::{AccessLog, Level, LogFields};
use tokio::sync::Semaphore;

use super::envelope::ApiError;
use super::request_meta::ensure_request_meta;
use super::state::SystemState;

/// 并发闸门的许可与等待上限。
pub struct Gate {
    semaphore: Semaphore,
    wait: Duration,
    system: Arc<SystemState>,
}

impl Gate {
    pub fn new(permits: u16, wait_ms: u32, system: Arc<SystemState>) -> Arc<Self> {
        Arc::new(Self {
            semaphore: Semaphore::new(usize::from(permits)),
            wait: Duration::from_millis(u64::from(wait_ms)),
            system,
        })
    }

    pub fn available(&self) -> usize {
        self.semaphore.available_permits()
    }
}

fn route_of(req: &Request) -> String {
    // 模板路径而不是实例路径：实例路径进标签就是时序爆炸。
    req.extensions()
        .get::<MatchedPath>()
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "<unmatched>".to_string())
}

/// 从请求头取规范化的 X-Client 标签。缺失或不在 CLIENT_KINDS 闭集时回落 `ops`，
/// 不引入闭集外的新标签取值。
fn client_label(req: &Request) -> &'static str {
    use super::headers::CLIENT_KINDS;
    let raw = req
        .headers()
        .get("x-client")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    CLIENT_KINDS
        .iter()
        .copied()
        .find(|k| *k == raw)
        .unwrap_or("ops")
}

fn status_class(status: StatusCode) -> &'static str {
    match status.as_u16() / 100 {
        2 => "2xx",
        3 => "3xx",
        4 => "4xx",
        _ => "5xx",
    }
}

/// 指标 method 标签只允许有界闭集。HTTP token 本身允许任意扩展方法，
/// 不能把攻击者可控原文直接变成 Prometheus series。
fn method_label(method: &Method) -> &'static str {
    match method.as_str() {
        "GET" => "GET",
        "POST" => "POST",
        "PUT" => "PUT",
        "PATCH" => "PATCH",
        "DELETE" => "DELETE",
        "OPTIONS" => "OPTIONS",
        "HEAD" => "HEAD",
        _ => "OTHER",
    }
}

/// 每请求一条访问日志，并填 `ep_http_request_duration_seconds`。
pub async fn observe(State(st): State<Arc<SystemState>>, mut req: Request, next: Next) -> Response {
    let started = Instant::now();
    let meta = ensure_request_meta(&mut req);
    let method = req.method().to_string();
    let metric_method = method_label(req.method());
    let trace_id = meta.trace_id.as_str().to_string();
    let request_id = Some(meta.request_id.as_str().to_string());
    let route = route_of(&req);
    // X-Client 在 req 被移动前取出。它已被 header_guard 强制校验为
    // CLIENT_KINDS 六值之一，此处再兜一次：缺失或不在闭集则回落 ops，不臆造。
    let client = client_label(&req);
    let mut response = next.run(req).await;
    let elapsed = started.elapsed();

    let entry = AccessLog {
        route: route.clone(),
        method: method.clone(),
        status: response.status().as_u16(),
        duration_ms: elapsed.as_millis() as u64,
        trace_id: trace_id.clone(),
        request_id,
        error_code: None,
        error_category: None,
    };
    st.logger().log(entry.level(), entry.clone().into_fields());

    let labels = [
        ("route", route.as_str()),
        ("method", metric_method),
        ("status_class", status_class(response.status())),
        // F-83：客户端类型取请求头 X-Client（已被 header_guard 校验为六值之一）。
        // 原实现写死 "ops"，使登记的另外六个取值在时序库里永不出现、
        // 「按端拆分」这一维恒为单值；而取值就在同一个请求头里。
        ("client", client),
    ];
    if let Err(e) = st.metrics().observe(
        "ep_http_request_duration_seconds",
        &labels,
        elapsed.as_secs_f64(),
    ) {
        st.logger().log(
            Level::Error,
            LogFields::msg("metrics", format!("指标写入失败：{e}")),
        );
    }
    // 框架级拒绝（例如 DefaultBodyLimit）未必有 JSON 封套；统一响应头
    // 使它们仍与访问日志、指标和同请求 RequestMeta 精确关联。
    response.headers_mut().insert(
        "x-ep-trace-id",
        HeaderValue::from_str(&trace_id).expect("服务端 TraceId 形态固定可作头值"),
    );
    response
}

/// 并发闸门。等待超过上限返回 503 与 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`，
/// 已获得许可的在途请求不受影响，不做静默降级。
pub async fn concurrency_gate(
    State(gate): State<Arc<Gate>>,
    mut req: Request,
    next: Next,
) -> Response {
    let trace_id = ensure_request_meta(&mut req).trace_id.as_str().to_string();
    let route = route_of(&req);
    let permit = tokio::time::timeout(gate.wait, gate.semaphore.acquire()).await;
    match permit {
        Ok(Ok(_permit)) => next.run(req).await,
        Ok(Err(e)) => {
            // 信号量被关闭只可能发生在进程收尾阶段，按未就绪处理而不是放行。
            gate.system.logger().log(
                Level::Error,
                LogFields::msg("gate", format!("并发闸门不可用：{e}")),
            );
            reject(&gate.system, PLATFORM_CAPACITY_CONCURRENCY_LIMIT, &trace_id)
        }
        Err(_) => {
            if let Err(e) = gate.system.metrics().inc_counter(
                "ep_quota_throttled_total",
                &[("route", route.as_str())],
                1.0,
            ) {
                gate.system.logger().log(
                    Level::Error,
                    LogFields::msg("metrics", format!("指标写入失败：{e}")),
                );
            }
            reject(&gate.system, PLATFORM_CAPACITY_CONCURRENCY_LIMIT, &trace_id)
        }
    }
}

fn reject(st: &SystemState, code: ep_foundation::ErrorCode, trace_id: &str) -> Response {
    ApiError::new(code, st.next_incident_no(), trace_id.to_string()).into_response()
}

/// 同步等待上限的参数。上限取 `http.request_timeout_ms`。
pub struct SyncLimit {
    limit: Duration,
    system: Arc<SystemState>,
}

impl SyncLimit {
    pub fn new(request_timeout_ms: u32, system: Arc<SystemState>) -> Arc<Self> {
        Arc::new(Self {
            limit: Duration::from_millis(u64::from(request_timeout_ms)),
            system,
        })
    }
}

/// 同步等待上限。超时返回 `PLATFORM.SYSTEM.SYNC_TIMEOUT`，
/// advice 中写明该请求应改由后台任务表达。
pub async fn sync_timeout(
    State(sl): State<Arc<SyncLimit>>,
    mut req: Request,
    next: Next,
) -> Response {
    let trace_id = ensure_request_meta(&mut req).trace_id.as_str().to_string();
    match tokio::time::timeout(sl.limit, next.run(req)).await {
        Ok(response) => response,
        Err(_) => reject(&sl.system, PLATFORM_SYSTEM_SYNC_TIMEOUT, &trace_id),
    }
}

/// panic 捕获。先写一条含 trace_id 的 ERROR 日志，再返回
/// `PLATFORM.SYSTEM.INTERNAL_ERROR`，进程不中止。
pub async fn catch_panic(
    State(st): State<Arc<SystemState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let request_trace_id = ensure_request_meta(&mut req).trace_id.as_str().to_string();
    // Box::pin 后是 Unpin，可以在安全代码里逐次 poll，不需要手写 pin 投影。
    let mut fut = Box::pin(next.run(req));
    let caught =
        poll_fn(
            |cx| match std::panic::catch_unwind(AssertUnwindSafe(|| fut.as_mut().poll(cx))) {
                Ok(poll) => poll.map(Ok),
                Err(payload) => Poll::Ready(Err(payload)),
            },
        )
        .await;

    match caught {
        Ok(response) => response,
        Err(payload) => {
            let what = panic_message(&payload);
            let trace_id = request_trace_id;
            st.logger().log(
                Level::Error,
                LogFields {
                    target: "http.panic",
                    msg: format!("请求处理 panic：{what}"),
                    trace_id: Some(trace_id.clone()),
                    outcome: Some("error"),
                    error_code: Some(PLATFORM_SYSTEM_INTERNAL_ERROR.0.to_string()),
                    error_category: Some("INFRASTRUCTURE".to_string()),
                    ..LogFields::default()
                },
            );
            ApiError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                st.next_incident_no(),
                trace_id,
            )
            .into_response()
        }
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "非字符串 panic 负载".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::DefaultBodyLimit;
    use axum::http::{header, Method};
    use axum::middleware::from_fn_with_state;
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use ep_foundation::security::context::{RequestId, TraceId};
    use ep_platform_obs::log::{JsonLogger, Level};
    use ep_platform_obs::MetricsRegistry;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::http::RequestMeta;
    use crate::lifecycle::Lifecycle;
    use crate::process::{BuildInfo, ProcessKind};
    use crate::selfcheck::{Outcome, SelfCheckReport};

    const TEST_TRACE: &str = "0123456789abcdef0123456789abcdef";

    fn system() -> Arc<SystemState> {
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

    fn request(method: Method, uri: &str, body: Body) -> Request {
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .expect("构造请求");
        request.extensions_mut().insert(RequestMeta {
            client_ip: "192.0.2.10".parse().expect("固定 IP 合法"),
            request_id: RequestId::new("runtime-http-test").expect("固定请求标识合法"),
            trace_id: TraceId::new(TEST_TRACE).expect("固定追踪标识合法"),
        });
        request
    }

    async fn envelope_trace(response: Response) -> String {
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("响应体可读")
            .to_bytes();
        serde_json::from_slice::<serde_json::Value>(&bytes).expect("统一封套必须是 JSON")
            ["trace_id"]
            .as_str()
            .expect("统一封套必须有 trace_id")
            .to_string()
    }

    fn assert_one_access_sample(st: &SystemState, route: &str, status_class: &str) {
        let want = format!(
            "ep_http_request_duration_seconds_count{{route=\"{route}\",method=\"GET\",status_class=\"{status_class}\",client=\"ops\"}} 1"
        );
        let text = st.metrics().encode_text();
        assert!(
            text.contains(&want),
            "应恰有一条统一访问样本：{want}\n{text}"
        );
    }

    #[test]
    fn status_class_covers_every_bucket() {
        assert_eq!(status_class(StatusCode::OK), "2xx");
        assert_eq!(status_class(StatusCode::FOUND), "3xx");
        assert_eq!(status_class(StatusCode::NOT_FOUND), "4xx");
        assert_eq!(status_class(StatusCode::SERVICE_UNAVAILABLE), "5xx");
    }

    #[test]
    fn panic_payload_is_rendered_without_losing_the_reason() {
        let payload: Box<dyn std::any::Any + Send> = Box::new("探针路径故意 panic");
        assert_eq!(panic_message(&payload), "探针路径故意 panic");
        let payload: Box<dyn std::any::Any + Send> = Box::new(42u8);
        assert_eq!(panic_message(&payload), "非字符串 panic 负载");
    }

    #[tokio::test]
    async fn outer_observer_reuses_request_trace_and_keeps_template_route_cardinality() {
        let st = system();
        let app = Router::new()
            .route("/widgets/{widget_id}", get(|| async { StatusCode::OK }))
            .layer(from_fn_with_state(st.clone(), observe));

        let response = app
            .oneshot(request(Method::GET, "/widgets/customer-123", Body::empty()))
            .await
            .expect("路由调用成功");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("x-ep-trace-id")
                .and_then(|value| value.to_str().ok()),
            Some(TEST_TRACE),
            "包括无封套响应在内，也必须用响应头关联同一 RequestMeta"
        );
        assert_one_access_sample(&st, "/widgets/{widget_id}", "2xx");
        assert!(
            !st.metrics().encode_text().contains("customer-123"),
            "指标不得把实例路径作为标签"
        );
    }

    #[tokio::test]
    async fn shared_http_boundary_removes_all_inbound_internal_headers() {
        let app = Router::new()
            .route(
                "/",
                get(|headers: axum::http::HeaderMap| async move {
                    assert!(
                        !headers
                            .keys()
                            .any(|name| name.as_str().starts_with("x-ep-")),
                        "业务处理器不得看见客户端伪造的内部头"
                    );
                    StatusCode::NO_CONTENT
                }),
            )
            .layer(from_fn_with_state(system(), catch_panic))
            .layer(from_fn_with_state(system(), observe));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .header("x-ep-user-id", "forged-user")
                    .header("x-ep-duty-classes", "SECURITY")
                    .header("x-ep-future-authority", "forged-future")
                    .header("x-ep-trace-id", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let trace = response
            .headers()
            .get("x-ep-trace-id")
            .unwrap()
            .to_str()
            .unwrap();
        assert_ne!(trace, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(trace.len(), 32);
    }

    #[tokio::test]
    async fn unknown_or_overlong_http_methods_collapse_to_the_other_metric_label() {
        let st = system();
        let app = Router::new()
            .route("/widgets/{widget_id}", get(|| async { StatusCode::OK }))
            .layer(from_fn_with_state(st.clone(), observe));
        let attacker_method = "ATTACKER-CONTROLLED-METHOD-WITH-UNBOUNDED-CARDINALITY";
        let method = Method::from_bytes(attacker_method.as_bytes()).expect("合法 HTTP token");

        let response = app
            .oneshot(request(method, "/widgets/customer-123", Body::empty()))
            .await
            .expect("未知 method 也应得到框架响应");
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

        let metrics = st.metrics().encode_text();
        assert!(metrics.contains(
            "ep_http_request_duration_seconds_count{route=\"/widgets/{widget_id}\",method=\"OTHER\",status_class=\"4xx\",client=\"ops\"} 1"
        ));
        assert!(
            !metrics.contains(attacker_method),
            "攻击者控制的原始 method 不得进入指标时序"
        );
    }

    #[tokio::test]
    async fn every_early_rejection_crosses_one_outer_access_trail_with_the_same_trace() {
        // 四头格式守卫在 handler 之前拒绝。
        let header_state = system();
        let header_app = Router::new()
            .route("/header/{id}", get(|| async { StatusCode::NO_CONTENT }))
            .layer(from_fn_with_state(
                header_state.clone(),
                super::super::headers::header_guard,
            ))
            .layer(from_fn_with_state(header_state.clone(), observe));
        let header_response = header_app
            .oneshot(request(Method::GET, "/header/42", Body::empty()))
            .await
            .expect("请求头拒绝应返回响应");
        assert_eq!(envelope_trace(header_response).await, TEST_TRACE);
        assert_one_access_sample(&header_state, "/header/{id}", "4xx");

        // 并发闸门在进入内层之前拒绝。
        let gate_state = system();
        let gate_app = Router::new()
            .route("/gate/{id}", get(|| async { StatusCode::NO_CONTENT }))
            .layer(from_fn_with_state(
                Gate::new(0, 0, gate_state.clone()),
                concurrency_gate,
            ))
            .layer(from_fn_with_state(gate_state.clone(), observe));
        let gate_response = gate_app
            .oneshot(request(Method::GET, "/gate/42", Body::empty()))
            .await
            .expect("闸门拒绝应返回响应");
        assert_eq!(envelope_trace(gate_response).await, TEST_TRACE);
        assert_one_access_sample(&gate_state, "/gate/{id}", "5xx");

        // 同步超时取消内层 future，仍只能留一条外层访问轨迹。
        let sync_state = system();
        let sync_app = Router::new()
            .route(
                "/sync/{id}",
                get(|| async {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    StatusCode::NO_CONTENT
                }),
            )
            .layer(from_fn_with_state(
                SyncLimit::new(1, sync_state.clone()),
                sync_timeout,
            ))
            .layer(from_fn_with_state(sync_state.clone(), observe));
        let sync_response = sync_app
            .oneshot(request(Method::GET, "/sync/42", Body::empty()))
            .await
            .expect("超时拒绝应返回响应");
        assert_eq!(envelope_trace(sync_response).await, TEST_TRACE);
        assert_one_access_sample(&sync_state, "/sync/{id}", "5xx");

        // DefaultBodyLimit 是框架拒绝，不产生业务封套；响应头仍必须关联
        // 同一 trace，且访问轨迹只记一次。
        let body_state = system();
        let body_app = Router::new()
            .route(
                "/body/{id}",
                post(|Json(_body): Json<serde_json::Value>| async { StatusCode::NO_CONTENT }),
            )
            .layer(DefaultBodyLimit::max(4))
            .layer(from_fn_with_state(body_state.clone(), observe));
        let mut body_request = request(Method::POST, "/body/42", Body::from(r#"{"too":"large"}"#));
        body_request
            .headers_mut()
            .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        let body_response = body_app
            .oneshot(body_request)
            .await
            .expect("正文超限应返回响应");
        assert_eq!(body_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(
            body_response
                .headers()
                .get("x-ep-trace-id")
                .and_then(|value| value.to_str().ok()),
            Some(TEST_TRACE)
        );
        let body_metrics = body_state.metrics().encode_text();
        assert!(body_metrics.contains(
            "ep_http_request_duration_seconds_count{route=\"/body/{id}\",method=\"POST\",status_class=\"4xx\",client=\"ops\"} 1"
        ));
    }
}
