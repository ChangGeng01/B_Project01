//! 受保护平台端点的安全上下文提取。
//!
//! 认证中间件核验会话、法人、设备与权限快照后，把完整
//! [`SecurityContext`] 写入请求扩展。本模块只消费该扩展并校验端点所需
//! 职责；任何请求头都不是身份权威。扩展缺失或职责不符一律稳定返回
//! 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`，不会落入 Axum 缺扩展的 500。

use axum::http::HeaderMap;
use ep_foundation::error::codes::PLATFORM_AUTHZ_OBJECT_FORBIDDEN;
use ep_foundation::security::context::{DutyClass, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_runtime::http::{ApiError, SystemState};

/// 缺省追踪标识：32 位十六进制零串，形态与 TraceId 冻结口径一致。
const DEFAULT_TRACE_ID: &str = "00000000000000000000000000000000";

/// 从认证中间件写入的请求扩展提取安全上下文。
/// `required_duties` 为该端点允许的职责类别，上下文缺失或一项都不命中即 403。
pub fn extract_context(
    context: Option<&SecurityContext>,
    state: &SystemState,
    required_duties: &[DutyClass],
) -> Result<SecurityContext, ApiError> {
    let trace_str = context
        .map(|ctx| ctx.trace_id.as_str())
        .unwrap_or(DEFAULT_TRACE_ID);
    let forbidden = |reason: &str| {
        ApiError::new(
            PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
            state.next_incident_no(),
            trace_str.to_string(),
        )
        .with_details(vec![ep_platform_runtime::http::Detail {
            field: "security-context".into(),
            reason: reason.into(),
            value: None,
        }])
    };

    let context = context.ok_or_else(|| forbidden("缺安全上下文扩展"))?;
    if !required_duties
        .iter()
        .any(|need| context.duty_classes.contains(need))
    {
        return Err(forbidden("职责类别不满足该端点要求"));
    }
    Ok(context.clone())
}

/// 重新认证证明尚未实现：客户端头永远不构成已验证权威。
/// 交付绑定用户/会话、时效与防重放校验的服务端证明前，敏感操作统一失败关闭。
pub fn require_reauth_token(
    _headers: &HeaderMap,
    state: &SystemState,
    trace_id: &TraceId,
) -> Result<(), ApiError> {
    Err(ApiError::new(
        PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
        state.next_incident_no(),
        trace_id.as_str().to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_foundation::id::Id;
    use ep_foundation::security::context::{RequestId, TraceId};
    use std::sync::Arc;
    use uuid::Uuid;

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (k, v) in pairs {
            map.insert(
                axum::http::HeaderName::from_bytes(k.as_bytes()).unwrap(),
                v.parse().unwrap(),
            );
        }
        map
    }

    fn state() -> Arc<SystemState> {
        use ep_platform_obs::log::{JsonLogger, Level};
        use ep_platform_obs::MetricsRegistry;
        use ep_platform_runtime::lifecycle::Lifecycle;
        use ep_platform_runtime::selfcheck::{Outcome, SelfCheckReport};
        use ep_platform_runtime::{BuildInfo, ProcessKind};
        SystemState::new(
            ProcessKind::CoreServer,
            BuildInfo::current(),
            Lifecycle::new(ProcessKind::CoreServer),
            SelfCheckReport {
                process: "core-server",
                version: "0.1.0".into(),
                items: Vec::new(),
                overall: Outcome::Passed,
            },
            Arc::new(MetricsRegistry::new()),
            Arc::new(JsonLogger::new("core-server", "0.1.0", Level::Info)),
        )
    }

    #[test]
    fn forged_identity_headers_cannot_supply_a_missing_security_context_extension() {
        let _forged = headers(&[
            ("x-ep-user-id", "11111111-1111-7111-8111-111111111111"),
            (
                "x-ep-legal-entity-id",
                "22222222-2222-7222-8222-222222222222",
            ),
            ("x-ep-session-id", "33333333-3333-7333-8333-333333333333"),
            ("x-ep-duty-classes", "SECURITY"),
        ]);
        let err = extract_context(None, &state(), &[DutyClass::Security])
            .expect_err("没有中间件注入的扩展时，伪造身份头也必须稳定拒绝");
        assert_eq!(err.code, PLATFORM_AUTHZ_OBJECT_FORBIDDEN);
    }

    #[test]
    fn a_present_security_context_extension_is_the_identity_authority() {
        let context = SecurityContext::system(
            Id::from_uuid(Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap()),
            RequestId::new("context-test").unwrap(),
            TraceId::new(DEFAULT_TRACE_ID).unwrap(),
        );

        let extracted = extract_context(Some(&context), &state(), &[DutyClass::System])
            .expect("匹配职责的中间件扩展必须被接受");

        assert_eq!(extracted.user_id, context.user_id);
        assert_eq!(extracted.legal_entity_id, context.legal_entity_id);
    }

    // 负样例断言的是职责类别门禁这条规则本身：不命中即 403。
    #[test]
    fn a_missing_duty_class_is_forbidden() {
        let ctx = SecurityContext::system(
            Id::from_uuid(Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap()),
            RequestId::new("context-test").unwrap(),
            TraceId::new(DEFAULT_TRACE_ID).unwrap(),
        );
        let err = extract_context(Some(&ctx), &state(), &[DutyClass::Security])
            .expect_err("SYSTEM 上下文不得通过 SECURITY 端点");
        assert_eq!(err.code, PLATFORM_AUTHZ_OBJECT_FORBIDDEN);
    }

    #[test]
    fn unverified_reauth_headers_never_supply_step_up_authority() {
        for pairs in [
            vec![],
            vec![("x-reauth-token", "")],
            vec![("x-reauth-token", "t-1")],
        ] {
            let error = require_reauth_token(
                &headers(&pairs),
                &state(),
                &TraceId::new(DEFAULT_TRACE_ID).unwrap(),
            )
            .expect_err("客户端头的存在性不是已验证的重新认证凭据");
            assert_eq!(error.code, PLATFORM_AUTHZ_OBJECT_FORBIDDEN);
        }
    }

    #[tokio::test]
    async fn privileged_handlers_reject_unverified_reauth_before_dependency_access() {
        use crate::platform::{reauth_handler_tests, PlatformState};
        use axum::{
            body::to_bytes,
            extract::{Extension, Path, State},
            Json,
        };
        let platform = Arc::new(PlatformState {
            system: state(),
            db: None,
            kms: None,
            identity: None,
            authn: None,
            authz: None,
            trusted_proxy_cidrs: Arc::from([]),
            window_ttl_max_min: 60,
        });
        let mut context = SecurityContext::system(
            Id::from_uuid(Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap()),
            RequestId::new("reauth-test").unwrap(),
            TraceId::new(DEFAULT_TRACE_ID).unwrap(),
        );
        context.duty_classes = Arc::from([DutyClass::System, DutyClass::Security]);
        let forged = headers(&[("x-reauth-token", "attacker-controlled")]);
        let responses = [
            reauth_handler_tests::rotate_key_domain(State(platform.clone()), Some(Extension(context.clone())),
                forged.clone(), Path(Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap()),
                Json(serde_json::from_value(serde_json::json!({"purpose":"data"})).unwrap())).await,
            reauth_handler_tests::open_window(State(platform), Some(Extension(context)), forged,
                Json(serde_json::from_value(serde_json::json!({"approval_ref":"approved", "reason":"maintenance", "ttl_minutes":10})).unwrap())).await,
        ];
        for response in responses {
            assert_eq!(
                response.status(),
                axum::http::StatusCode::FORBIDDEN,
                "必须先拒绝重新认证，不能进入缺数据库/KMS 的 503 分支"
            );
            let body = to_bytes(response.into_body(), 4096).await.unwrap();
            assert!(std::str::from_utf8(&body)
                .unwrap()
                .contains("PLATFORM.AUTHZ.OBJECT_FORBIDDEN"));
        }
    }

    #[tokio::test]
    async fn denied_step_up_envelope_matches_server_trace_header() {
        use crate::platform::{middleware, reauth_handler_tests, PlatformState};
        use axum::{
            extract::Request,
            middleware::{from_fn, from_fn_with_state, Next},
            routing::post,
            Router,
        };
        use ep_platform_runtime::http::{middleware::observe, RequestMeta, TrustedProxyNet};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let system = state();
        let platform = Arc::new(PlatformState {
            system: system.clone(),
            db: None,
            kms: None,
            identity: None,
            authn: None,
            authz: None,
            trusted_proxy_cidrs: Arc::from([]),
            window_ttl_max_min: 60,
        });
        let app = Router::new()
            .route(
                "/rotate/{id}",
                post(reauth_handler_tests::rotate_key_domain),
            )
            .route("/open", post(reauth_handler_tests::open_window))
            .with_state(platform)
            .layer(from_fn(|mut request: Request, next: Next| async move {
                let meta = request.extensions().get::<RequestMeta>().unwrap();
                let mut context = SecurityContext::system(
                    Id::from_uuid(Uuid::parse_str("22222222-2222-7222-8222-222222222222").unwrap()),
                    meta.request_id.clone(),
                    meta.trace_id.clone(),
                );
                context.duty_classes = Arc::from([DutyClass::System, DutyClass::Security]);
                request.extensions_mut().insert(context);
                next.run(request).await
            }))
            .layer(from_fn_with_state(system, observe))
            .layer(from_fn_with_state(
                Arc::<[TrustedProxyNet]>::from([]),
                middleware::request_metadata,
            ));
        let (listener, address) = ep_platform_runtime::http::bind("127.0.0.1:0".parse().unwrap())
            .await
            .unwrap();
        let (stop, stopped) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(ep_platform_runtime::http::serve_on(listener, app, async {
            let _ = stopped.await;
        }));
        for (path, body) in [
            (
                "/rotate/22222222-2222-7222-8222-222222222222",
                r#"{"purpose":"data"}"#,
            ),
            (
                "/open",
                r#"{"approval_ref":"approved","reason":"maintenance","ttl_minutes":10}"#,
            ),
        ] {
            let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
            let request = format!("POST {path} HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nx-reauth-token: attacker-controlled\r\nx-ep-trace-id: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\n\r\n{body}", body.len());
            stream.write_all(request.as_bytes()).await.unwrap();
            let mut bytes = Vec::new();
            tokio::time::timeout(
                std::time::Duration::from_secs(2),
                stream.read_to_end(&mut bytes),
            )
            .await
            .unwrap()
            .unwrap();
            let response = std::str::from_utf8(&bytes).unwrap();
            let (headers, body) = response.split_once("\r\n\r\n").unwrap();
            assert!(headers.starts_with("HTTP/1.1 403"), "{headers}");
            let trace = headers
                .lines()
                .find_map(|line| line.strip_prefix("x-ep-trace-id: "))
                .unwrap();
            assert_ne!(trace, DEFAULT_TRACE_ID);
            assert_ne!(trace, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
            let envelope: serde_json::Value = serde_json::from_str(body).unwrap();
            assert_eq!(envelope["error"]["code"], "PLATFORM.AUTHZ.OBJECT_FORBIDDEN");
            assert_eq!(
                envelope["trace_id"], trace,
                "拒绝封套必须与服务端响应头、访问日志共用同一 trace"
            );
        }
        stop.send(()).unwrap();
        server.await.unwrap().unwrap();
    }
}
