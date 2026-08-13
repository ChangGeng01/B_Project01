//! 门户 Cookie → 核心会话令牌转发（阶段 4 任务 #23，04 计划 §2.2）。
//!
//! 门户进程不持有数据库连接：会话令牌由 core-server 的门户登录端点
//! 签发，浏览器以门户 Cookie 持有；网关转发上游时把 Cookie 换回
//! `Authorization: Bearer <令牌>`，供核心认证层按既有形态消费。
//! 呈现层由门户阶段承担，此处只落转发换算逻辑。
//!
//! 规则：请求已带 `Authorization` 时一律不覆盖（核心认证层以该头
//! 为准，覆盖等于替客户端改凭据）；Cookie 缺门户会话项或取值形态
//! 非法时不注入，请求按未认证形态落到核心侧处置。令牌取值不进日志。

use axum::extract::Request;
use axum::http::header::{AUTHORIZATION, COOKIE};
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

/// 门户会话 Cookie 的固定名。
pub const PORTAL_SESSION_COOKIE: &str = "ep-portal-session";

/// 从 `Cookie` 头取值中解析门户会话令牌：按 RFC 6265 的 Cookie
/// 头形态取固定名项的取值（止于下一个分号），再整体校验取值
/// 必须是可直接进请求头的形态。
pub fn session_token_from_cookies(cookie_header: &str) -> Option<String> {
    let prefix = format!("{PORTAL_SESSION_COOKIE}=");
    let value = find_pair_value(cookie_header, &prefix)?;
    header_safe(value).then(|| value.to_string())
}

/// 在分号分隔的取值串里找 `名=` 前缀项，返回其后到下一个分号
/// 之前的段；同时要求该项之前是串首或分号边界，避免同名后缀撞入。
fn find_pair_value<'a>(header: &'a str, prefix: &str) -> Option<&'a str> {
    let mut cursor = 0;
    while let Some(pos) = header[cursor..].find(prefix) {
        let start = cursor + pos;
        let at_boundary = start == 0
            || header
                .as_bytes()
                .get(start - 1)
                .is_some_and(|b| matches!(b, b';' | b' '));
        if at_boundary {
            let value_start = start + prefix.len();
            let end = header[value_start..]
                .find(';')
                .map_or(header.len(), |p| value_start + p);
            return Some(header[value_start..end].trim());
        }
        cursor = start + prefix.len();
    }
    None
}

/// 令牌形态必须是可直接进请求头的：可打印 ASCII，不含空格、
/// 引号、逗号与分号（后三者会歧义 Cookie 与头的取值边界）。
fn header_safe(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|b| (0x21..=0x7E).contains(&b) && !matches!(b, b'"' | b',' | b';'))
}

/// 令牌到 `Authorization` 头取值：与核心认证层消费的 Bearer 形态一致。
pub fn bearer_value(token: &str) -> String {
    format!("Bearer {token}")
}

/// 转发换算：Cookie 里有门户会话令牌且请求未自带 `Authorization`
/// 时注入 Bearer 头；返回是否注入。
pub fn inject_authorization(req: &mut Request) -> bool {
    if req.headers().contains_key(AUTHORIZATION) {
        return false;
    }
    let Some(cookies) = req
        .headers()
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(session_token_from_cookies)
    else {
        return false;
    };
    let Ok(value) = HeaderValue::from_str(&bearer_value(&cookies)) else {
        return false;
    };
    req.headers_mut().insert(AUTHORIZATION, value);
    true
}

/// 转发中间件：在每个请求进入门户路由前完成 Cookie → Bearer 换算。
/// 门户业务路由（门户阶段）对核心的调用一律经此拿到会话令牌。
pub async fn forward_session(mut req: Request, next: Next) -> Response {
    inject_authorization(&mut req);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request as HttpRequest;

    fn req_with(headers: &[(&str, &str)]) -> Request {
        let mut builder = HttpRequest::builder().uri("/portal/v1/system/upstream");
        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }
        builder.body(axum::body::Body::empty()).expect("构造合法")
    }

    #[test]
    fn token_is_found_among_multiple_cookies() {
        let header = "theme=dark; ep-portal-session=abc123; lang=zh";
        assert_eq!(
            session_token_from_cookies(header),
            Some("abc123".to_string())
        );
    }

    #[test]
    fn absent_or_empty_cookie_yields_none() {
        assert_eq!(session_token_from_cookies("theme=dark"), None);
        assert_eq!(session_token_from_cookies(""), None);
        assert_eq!(session_token_from_cookies("ep-portal-session="), None);
    }

    #[test]
    fn header_unsafe_value_is_rejected() {
        assert_eq!(
            session_token_from_cookies("ep-portal-session=a b"),
            None,
            "空格不得进入请求头取值"
        );
        assert_eq!(
            session_token_from_cookies("ep-portal-session=a;b"),
            Some("a".to_string()),
            "按 RFC 6265 取值止于分号，后段是另一个 Cookie"
        );
        assert_eq!(
            session_token_from_cookies("x-ep-portal-session=evil"),
            None,
            "同名后缀不得撞入"
        );
    }

    #[test]
    fn injection_maps_cookie_to_bearer() {
        let mut req = req_with(&[("cookie", "ep-portal-session=tok42")]);
        assert!(inject_authorization(&mut req));
        assert_eq!(
            req.headers().get(AUTHORIZATION).expect("已注入"),
            "Bearer tok42"
        );
    }

    #[test]
    fn an_existing_authorization_header_is_never_overwritten() {
        let mut req = req_with(&[
            ("cookie", "ep-portal-session=tok42"),
            ("authorization", "Bearer caller-token"),
        ]);
        assert!(!inject_authorization(&mut req));
        assert_eq!(
            req.headers().get(AUTHORIZATION).expect("原值保留"),
            "Bearer caller-token"
        );
    }

    #[test]
    fn no_cookie_means_no_injection() {
        let mut req = req_with(&[]);
        assert!(!inject_authorization(&mut req));
        assert!(req.headers().get(AUTHORIZATION).is_none());
    }
}
