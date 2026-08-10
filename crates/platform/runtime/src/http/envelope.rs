//! 统一封套与错误映射。字段集合与顺序取技术基线第 5.2 节。
//!
//! 文案不内联在调用点：调用点只给错误码，文案由本文件的占位表取。
//! 文案定稿（U-A-06）时只改这张表与 `docs/error-codes.md`，调用点一处不动。

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use ep_foundation::error::codes::{self, Category, Registered};
use ep_foundation::ErrorCode;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Detail {
    pub field: String,
    pub reason: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub category: &'static str,
    pub message: &'static str,
    pub details: Vec<Detail>,
    pub retryable: bool,
    pub incident_no: String,
    pub occurred_at: String,
    pub advice: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct Envelope<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorBody>,
    pub meta: Option<serde_json::Value>,
    pub trace_id: String,
}

impl<T: Serialize> Envelope<T> {
    pub fn ok(data: T, trace_id: impl Into<String>) -> Self {
        Self { success: true, data: Some(data), error: None, meta: None, trace_id: trace_id.into() }
    }
}

/// 占位文案表，与 `docs/error-codes.md` 第 3 节逐行一致。
/// 只登记本阶段会返回的码；只登记不返回的码没有文案调用点，
/// 留空表比留一张永远取不到的行更诚实。
const TEXTS: [(&str, &str, &str); 8] = [
    (
        codes::PLATFORM_SYSTEM_NOT_READY.0,
        "系统尚未就绪，暂时无法处理该请求。",
        "请稍后重试；持续未就绪时联系管理员查看启动自检报告。",
    ),
    (
        codes::PLATFORM_SYSTEM_SYNC_TIMEOUT.0,
        "该请求处理时间超过同步等待上限。",
        "请改用后台任务方式提交该操作，或缩小单次处理范围后重试。",
    ),
    (
        codes::PLATFORM_SYSTEM_INTERNAL_ERROR.0,
        "系统内部错误，本次操作未生效。",
        "请记录关联编号后重试；重复出现时联系管理员。",
    ),
    (
        codes::PLATFORM_REQUEST_INVALID_PAYLOAD.0,
        "请求内容不符合要求。",
        "请按提示修正标出的字段后重新提交。",
    ),
    (
        codes::PLATFORM_REQUEST_HEADER_MISSING.0,
        "请求缺少必需的标识信息，或其格式不正确。",
        "请更新客户端到受支持的版本后重试。",
    ),
    (
        codes::PLATFORM_ROUTE_NOT_FOUND.0,
        "请求的地址不存在。",
        "请检查地址是否正确，或确认客户端版本与服务端一致。",
    ),
    (
        codes::PLATFORM_IDEMPOTENCY_KEY_REQUIRED.0,
        "该写入请求缺少幂等标识，或标识格式不正确。",
        "请由客户端为每次写入生成一个幂等标识后重试。",
    ),
    (
        codes::PLATFORM_CAPACITY_CONCURRENCY_LIMIT.0,
        "当前并发请求已达上限，本次请求未被受理。",
        "请稍后重试；高峰期持续出现时联系管理员调整并发上限。",
    ),
];

fn registered_of(code: ErrorCode) -> &'static Registered {
    codes::REGISTERED
        .iter()
        .find(|r| r.code == code)
        // 传进来的码只能来自 foundation 的常量表，取不到即为编码错误，
        // 此时宁可在测试里炸掉，也不能回落到一个含混的默认码。
        .expect("错误码必须先登记在 ep-foundation 的常量表中")
}

fn text_of(code: ErrorCode) -> (&'static str, &'static str) {
    TEXTS
        .iter()
        .find(|(c, _, _)| *c == code.0)
        .map(|(_, m, a)| (*m, *a))
        .expect("本阶段会返回的错误码必须在占位文案表中有一行")
}

const fn category_str(c: Category) -> &'static str {
    c.as_str()
}

/// 一个可直接转成 HTTP 响应的错误。
#[derive(Clone, Debug)]
pub struct ApiError {
    pub code: ErrorCode,
    pub details: Vec<Detail>,
    pub incident_no: String,
    pub occurred_at: String,
    pub trace_id: String,
}

impl ApiError {
    pub fn new(code: ErrorCode, incident_no: String, trace_id: String) -> Self {
        Self {
            code,
            details: Vec::new(),
            incident_no,
            occurred_at: ep_platform_obs::log::now_rfc3339_micros(),
            trace_id,
        }
    }

    pub fn with_details(mut self, details: Vec<Detail>) -> Self {
        self.details = details;
        self
    }

    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(registered_of(self.code).http).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn category(&self) -> &'static str {
        category_str(registered_of(self.code).category)
    }

    pub fn body(&self) -> Envelope<serde_json::Value> {
        let reg = registered_of(self.code);
        let (message, advice) = text_of(self.code);
        Envelope {
            success: false,
            data: None,
            error: Some(ErrorBody {
                code: self.code.0,
                category: category_str(reg.category),
                message,
                details: self.details.clone(),
                retryable: reg.retryable,
                incident_no: self.incident_no.clone(),
                occurred_at: self.occurred_at.clone(),
                advice,
            }),
            meta: None,
            trace_id: self.trace_id.clone(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status(), axum::Json(self.body())).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn err(code: ErrorCode) -> ApiError {
        ApiError::new(code, "ERR-20260811-100000".into(), "4bf92f3577b34da6a3ce929d0e0e4736".into())
    }

    #[test]
    fn http_status_and_retryable_come_from_the_frozen_table() {
        let e = err(codes::PLATFORM_CAPACITY_CONCURRENCY_LIMIT);
        assert_eq!(e.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(e.category(), "INFRASTRUCTURE");
        assert!(e.body().error.expect("有错误体").retryable);
    }

    #[test]
    fn envelope_has_exactly_the_five_top_level_keys() {
        let v = serde_json::to_value(err(codes::PLATFORM_ROUTE_NOT_FOUND).body()).unwrap();
        let obj = v.as_object().unwrap();
        let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(keys, ["data", "error", "meta", "success", "trace_id"]);
    }

    #[test]
    fn error_body_carries_the_four_mandatory_elements() {
        let body = err(codes::PLATFORM_SYSTEM_INTERNAL_ERROR).body();
        let e = body.error.expect("有错误体");
        assert!(!e.incident_no.is_empty());
        assert!(e.occurred_at.ends_with('Z'));
        assert!(e.retryable);
        assert!(!e.advice.is_empty());
    }

    // 负样例断言的是文案纪律这条规则本身：占位文案里不得出现内部信息。
    #[test]
    fn placeholder_texts_leak_no_internal_terms() {
        const FORBIDDEN: [&str; 8] =
            ["select", "SELECT", "panic", "core-server", "job-worker", "schema", "table", "secret"];
        for (code, message, advice) in TEXTS {
            for bad in FORBIDDEN {
                assert!(!message.contains(bad), "{code} 的 message 出现 {bad}");
                assert!(!advice.contains(bad), "{code} 的 advice 出现 {bad}");
            }
        }
    }

    #[test]
    fn success_envelope_has_no_error() {
        let v = Envelope::ok(serde_json::json!({"status": "UP"}), "t");
        assert!(v.error.is_none());
        assert!(v.success);
    }
}
