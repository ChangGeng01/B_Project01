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
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use ep_foundation::error::codes::{
    PLATFORM_CAPACITY_CONCURRENCY_LIMIT, PLATFORM_SYSTEM_INTERNAL_ERROR,
    PLATFORM_SYSTEM_SYNC_TIMEOUT,
};
use ep_platform_obs::log::{AccessLog, Level, LogFields};
use tokio::sync::Semaphore;

use super::envelope::ApiError;
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

/// 每请求一条访问日志，并填 `ep_http_request_duration_seconds`。
pub async fn observe(State(st): State<Arc<SystemState>>, req: Request, next: Next) -> Response {
    let started = Instant::now();
    let method = req.method().to_string();
    let trace = ep_platform_obs::TraceContext::new();
    let trace_id = trace.trace_id().to_string();
    let route = route_of(&req);
    // X-Client 在 req 被移动前取出。它已被 header_guard 强制校验为
    // CLIENT_KINDS 六值之一，此处再兜一次：缺失或不在闭集则回落 ops，不臆造。
    let client = client_label(&req);
    let response = next.run(req).await;
    let elapsed = started.elapsed();

    let entry = AccessLog {
        route: route.clone(),
        method: method.clone(),
        status: response.status().as_u16(),
        duration_ms: elapsed.as_millis() as u64,
        trace_id,
        request_id: None,
        error_code: None,
        error_category: None,
    };
    st.logger().log(entry.level(), entry.clone().into_fields());

    let labels = [
        ("route", route.as_str()),
        ("method", method.as_str()),
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
    response
}

/// 并发闸门。等待超过上限返回 503 与 `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`，
/// 已获得许可的在途请求不受影响，不做静默降级。
pub async fn concurrency_gate(State(gate): State<Arc<Gate>>, req: Request, next: Next) -> Response {
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
            reject(&gate.system, PLATFORM_CAPACITY_CONCURRENCY_LIMIT)
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
            reject(&gate.system, PLATFORM_CAPACITY_CONCURRENCY_LIMIT)
        }
    }
}

fn reject(st: &SystemState, code: ep_foundation::ErrorCode) -> Response {
    ApiError::new(
        code,
        st.next_incident_no(),
        ep_platform_obs::TraceContext::new().trace_id().to_string(),
    )
    .into_response()
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
pub async fn sync_timeout(State(sl): State<Arc<SyncLimit>>, req: Request, next: Next) -> Response {
    match tokio::time::timeout(sl.limit, next.run(req)).await {
        Ok(response) => response,
        Err(_) => reject(&sl.system, PLATFORM_SYSTEM_SYNC_TIMEOUT),
    }
}

/// panic 捕获。先写一条含 trace_id 的 ERROR 日志，再返回
/// `PLATFORM.SYSTEM.INTERNAL_ERROR`，进程不中止。
pub async fn catch_panic(State(st): State<Arc<SystemState>>, req: Request, next: Next) -> Response {
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
            let trace_id = ep_platform_obs::TraceContext::new().trace_id().to_string();
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
}
