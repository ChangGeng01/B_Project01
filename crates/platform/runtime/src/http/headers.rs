//! 固定请求头的格式校验，以及幂等键中间件。
//!
//! 本阶段只校验存在性与格式，不做任何真实校验：真实校验与其端口由阶段 4
//! 在交付第一条判定时同批引入。这一点必须在文档里写明，否则会被误认为
//! 已具备鉴权。这里也因此不定义 `AuthnPort`，不注入任何空实现。

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use ep_foundation::error::codes::{PLATFORM_IDEMPOTENCY_KEY_REQUIRED, PLATFORM_REQUEST_HEADER_MISSING};
use std::sync::Arc;

use super::envelope::{ApiError, Detail};
use super::state::SystemState;

/// 四个固定头。系统端点豁免这四个头，豁免清单见 [`EXEMPT_PREFIXES`]。
pub const REQUIRED_HEADERS: [&str; 4] =
    ["x-legal-entity-id", "x-device-id", "x-client", "authorization"];

/// 豁免路径前缀。新增豁免必须改这张表并触发 CODEOWNERS 的安全审查。
pub const EXEMPT_PREFIXES: [&str; 2] = ["/api/v1/system/", "/portal/v1/system/"];

/// `X-Client` 的六个取值，与 `ClientKind` 的六个变体一一对应。
pub const CLIENT_KINDS: [&str; 6] = ["win", "mac", "ios", "android", "portal", "ops"];

pub fn is_exempt(path: &str) -> bool {
    EXEMPT_PREFIXES.iter().any(|p| path.starts_with(p))
}

fn is_uuid(v: &str) -> bool {
    let bytes = v.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    bytes.iter().enumerate().all(|(i, b)| match i {
        8 | 13 | 18 | 23 => *b == b'-',
        _ => b.is_ascii_hexdigit(),
    })
}

/// UUIDv7 另判版本位与变体位。幂等键取 UUIDv7 是为了让键自带时间序。
pub fn is_uuid_v7(v: &str) -> bool {
    if !is_uuid(v) {
        return false;
    }
    let b = v.as_bytes();
    b[14] == b'7' && matches!(b[19], b'8' | b'9' | b'a' | b'b' | b'A' | b'B')
}

fn is_device_id(v: &str) -> bool {
    let len = v.chars().count();
    (1..=64).contains(&len) && v.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn is_bearer(v: &str) -> bool {
    let Some(token) = v.strip_prefix("Bearer ") else { return false };
    token.len() == 43 && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// 逐头校验，返回全部不合规项而不是遇到第一个就停：一次告诉客户端全部问题，
/// 比让它改一个试一次快得多。
pub fn validate(headers: &[(String, String)]) -> Vec<Detail> {
    let find = |name: &str| {
        headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    };
    let mut problems = Vec::new();
    let mut check = |name: &str, ok: bool, reason: &str| {
        if !ok {
            problems.push(Detail { field: name.to_string(), reason: reason.to_string(), value: None });
        }
    };
    match find("x-legal-entity-id") {
        None => check("X-Legal-Entity-Id", false, "MISSING"),
        Some(v) => check("X-Legal-Entity-Id", is_uuid(v), "NOT_UUID"),
    }
    match find("x-device-id") {
        None => check("X-Device-Id", false, "MISSING"),
        Some(v) => check("X-Device-Id", is_device_id(v), "BAD_FORMAT"),
    }
    match find("x-client") {
        None => check("X-Client", false, "MISSING"),
        Some(v) => check("X-Client", CLIENT_KINDS.contains(&v), "NOT_IN_ENUM"),
    }
    match find("authorization") {
        None => check("Authorization", false, "MISSING"),
        Some(v) => check("Authorization", is_bearer(v), "BAD_FORMAT"),
    }
    problems
}

/// 幂等键中间件。名字固定为 `IdempotencyKeyHeaderGuard`（裁定 C-07 第一段）。
/// 本阶段只校验存在与格式，不做判等与重放存储。
pub struct IdempotencyKeyHeaderGuard;

impl IdempotencyKeyHeaderGuard {
    pub fn check(value: Option<&str>) -> Result<(), &'static str> {
        match value {
            None => Err("MISSING"),
            Some(v) if !is_uuid_v7(v) => Err("NOT_UUID_V7"),
            Some(_) => Ok(()),
        }
    }
}

/// 写请求的幂等键校验层。
pub async fn idempotency_key_guard(
    State(st): State<Arc<SystemState>>,
    req: Request,
    next: Next,
) -> Response {
    if req.method().is_safe() {
        return next.run(req).await;
    }
    let key = req.headers().get("idempotency-key").and_then(|v| v.to_str().ok());
    match IdempotencyKeyHeaderGuard::check(key) {
        Ok(()) => next.run(req).await,
        Err(reason) => ApiError::new(
            PLATFORM_IDEMPOTENCY_KEY_REQUIRED,
            st.next_incident_no(),
            ep_platform_obs::TraceContext::new().trace_id().to_string(),
        )
        .with_details(vec![Detail {
            field: "Idempotency-Key".into(),
            reason: reason.into(),
            value: None,
        }])
        .into_response(),
    }
}

/// 固定请求头校验层。系统端点豁免。
pub async fn header_guard(State(st): State<Arc<SystemState>>, req: Request, next: Next) -> Response {
    if is_exempt(req.uri().path()) {
        return next.run(req).await;
    }
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|s| (k.as_str().to_string(), s.to_string())))
        .collect();
    let problems = validate(&headers);
    if problems.is_empty() {
        return next.run(req).await;
    }
    ApiError::new(
        PLATFORM_REQUEST_HEADER_MISSING,
        st.next_incident_no(),
        ep_platform_obs::TraceContext::new().trace_id().to_string(),
    )
    .with_details(problems)
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full() -> Vec<(String, String)> {
        [
            ("X-Legal-Entity-Id", "0192f3a1-7b2c-7def-8000-0123456789ab"),
            ("X-Device-Id", "DESK-001"),
            ("X-Client", "win"),
            ("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    #[test]
    fn all_four_headers_present_and_well_formed_passes() {
        assert!(validate(&full()).is_empty());
    }

    // 负样例断言的是四个头这条规则本身：逐个缺失与逐个格式错误各一次。
    #[test]
    fn each_missing_header_is_reported_by_name() {
        for name in REQUIRED_HEADERS {
            let headers: Vec<(String, String)> =
                full().into_iter().filter(|(k, _)| !k.eq_ignore_ascii_case(name)).collect();
            let problems = validate(&headers);
            assert_eq!(problems.len(), 1, "缺 {name} 时应只报一项");
            assert_eq!(problems[0].reason, "MISSING");
        }
    }

    #[test]
    fn each_malformed_header_is_rejected() {
        let cases = [
            ("X-Legal-Entity-Id", "not-a-uuid", "NOT_UUID"),
            ("X-Device-Id", "含中文", "BAD_FORMAT"),
            ("X-Client", "linux", "NOT_IN_ENUM"),
            ("Authorization", "Basic abc", "BAD_FORMAT"),
        ];
        for (name, bad, reason) in cases {
            let headers: Vec<(String, String)> = full()
                .into_iter()
                .map(|(k, v)| if k.eq_ignore_ascii_case(name) { (k, bad.to_string()) } else { (k, v) })
                .collect();
            let problems = validate(&headers);
            assert_eq!(problems.len(), 1, "{name} 应只报一项");
            assert_eq!(problems[0].reason, reason);
        }
    }

    #[test]
    fn system_prefixes_are_exempt_and_nothing_else_is() {
        assert!(is_exempt("/api/v1/system/health"));
        assert!(is_exempt("/portal/v1/system/metrics"));
        assert!(!is_exempt("/api/v1/sales/sales-orders"));
        assert!(!is_exempt("/api/v1/systemx/health"), "豁免按前缀整段匹配，不做模糊匹配");
    }

    #[test]
    fn idempotency_key_must_be_a_uuid_v7() {
        assert_eq!(IdempotencyKeyHeaderGuard::check(Some("0192f3a1-7b2c-7def-8000-0123456789ab")), Ok(()));
        assert_eq!(IdempotencyKeyHeaderGuard::check(None), Err("MISSING"));
        // v4 的版本位不是 7，必须被拒。
        assert_eq!(
            IdempotencyKeyHeaderGuard::check(Some("0192f3a1-7b2c-4def-8000-0123456789ab")),
            Err("NOT_UUID_V7")
        );
        assert_eq!(IdempotencyKeyHeaderGuard::check(Some("abc")), Err("NOT_UUID_V7"));
    }
}
