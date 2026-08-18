//! core-server 的九个平台端点（02 计划 §5，A-01~A-09）。
//!
//! 全部挂在 `/api/v1/platform/` 下。写请求经运行期
//! `IdempotencyKeyHeaderGuard` 层校验请求头存在性与 UUIDv7 形态；
//! 判等与重放存储属阶段 3a，本阶段不实现（C-07 三段分工）。
//!
//! 能力常量逐对引用 `ep-platform-tenancy` 的 capability 登记，
//! 不另起字面量（A-20，configdoc 断言路由可解析）。

mod events;
pub mod identity;
pub mod identity_admin;
mod key_domain;
pub mod middleware;
mod migration;
mod sensitive;
mod windows;

use std::sync::Arc;

use axum::http::{HeaderMap, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, MethodRouter};
use axum::{Json, Router};
use ep_adapter_kms::BuiltinKmsBackend;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::error::AppError;
use ep_platform_runtime::http::headers::idempotency_key_guard;
use ep_platform_runtime::http::{ApiError, Envelope, SystemState};

use crate::wiring::authz::AuthzAssembly;
use crate::wiring::identity::IdentityAssembly;
use crate::wiring::DbAssembly;

/// A-20 路由登记条目：（路径、路由、能力元组）逐行同行，
/// 定长数组逐条登记，能力元组缺失即编译不过。
pub type RouteEntry = (
    &'static str,
    MethodRouter<Arc<PlatformState>>,
    (CapabilityDomain, ActionClass),
);

/// 缺省追踪标识：32 位十六进制零串，形态与 TraceId 冻结口径一致。
pub const ZERO_TRACE: &str = "00000000000000000000000000000000";

/// 平台端点的共享状态。db 或 kms 缺位时按 unwired-absent 纪律
/// 返回 503 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`，不以空实现顶位。
pub struct PlatformState {
    pub system: Arc<SystemState>,
    pub db: Option<Arc<DbAssembly>>,
    pub kms: Option<Arc<BuiltinKmsBackend>>,
    /// 身份域装配（阶段 4 任务 #21）：缺位时身份端点按
    /// 503 NOT_PROVISIONED 处置（unwired-absent）。
    pub identity: Option<Arc<IdentityAssembly>>,
    /// 认证中间件载体（阶段 4 任务 #23）：缺位时中间件按未装配
    /// 形态放行，端点仍按 503 NOT_PROVISIONED 处置（unwired-absent）。
    pub authn: Option<Arc<middleware::AuthnAssembly>>,
    /// 授权域装配（阶段 4 任务 #23）：快照持有者、轮询器与指标
    /// 桥接；判定面接入时（阶段 5+）经此消费（unwired-absent）。
    pub authz: Option<Arc<AuthzAssembly>>,
    /// `EP__MIGRATION__WINDOW_TTL_MAX_MIN`，A-09 开窗上限。
    pub window_ttl_max_min: u32,
}

/// 从请求头取追踪标识：形态不合法即缺省零串。
pub fn trace_of(headers: &HeaderMap) -> String {
    use ep_foundation::security::context::TraceId;
    headers
        .get("x-ep-trace-id")
        .and_then(|v| v.to_str().ok())
        .filter(|v| TraceId::new(v).is_ok())
        .map(str::to_string)
        .unwrap_or_else(|| ZERO_TRACE.to_string())
}

/// 十个平台路由，四读六写。能力常量在 capability 登记，路由字面量是唯一引用点。
/// 写路由单独成组并挂阶段 1 的幂等键请求头守卫层；判等与重放存储
/// 属阶段 3a，本阶段不实现（C-07 三段分工）。
///
/// 本函数原先用裸 `.route(path, handler)` 注册这十条，**绕开了同文件定义的
/// [`RouteEntry`]**，于是那一行「能力元组缺失即编译不过」对这十条不生效——
/// 全仓 33 条路由里恰是这十条没有能力元组。改为逐条走 `RouteEntry`，
/// 使该类型的编译期保证覆盖到它们；判据侧另由 `xtask configdoc` 第四段承接。
///
/// 十条的能力域一律取 `PlatformAdminLowcodeOps`，与 `identity_admin.rs` 的
/// 平台管理端点同域；动作类别按读写取 `Read` 与 `Write`。这两项是本次落码所定，
/// 卷内未逐条指定，若日后另有裁定以裁定为准。
pub fn platform_router(state: Arc<PlatformState>) -> Router {
    const READS: usize = 4;
    const WRITES: usize = 6;

    let read_entries: [RouteEntry; READS] = [
        (
            "/api/v1/platform/key-domains",
            get(key_domain::list_key_domains),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/key-domains/{id}",
            get(key_domain::get_key_domain),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/sensitive-fields",
            get(sensitive::list_sensitive_fields),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
        (
            "/api/v1/platform/migrations",
            get(migration::list_migrations),
            (CapabilityDomain::PlatformAdminLowcodeOps, ActionClass::Read),
        ),
    ];
    let write_entries: [RouteEntry; WRITES] = [
        (
            "/api/v1/platform/key-domains/actions/provision",
            post(key_domain::provision_key_domain),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/key-domains/{id}/actions/rotate",
            post(key_domain::rotate_key_domain),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/key-domains/{id}/actions/plan-destroy",
            post(key_domain::plan_destroy_key_domain),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/key-domains/{id}/actions/cancel-destroy",
            post(key_domain::cancel_destroy_key_domain),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/migrations/actions/open-window",
            post(windows::open_window),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
        (
            "/api/v1/platform/migrations/actions/close-window",
            post(windows::close_window),
            (
                CapabilityDomain::PlatformAdminLowcodeOps,
                ActionClass::Write,
            ),
        ),
    ];

    let mut reads = Router::new();
    for (path, handler, _capability) in read_entries {
        reads = reads.route(path, handler);
    }
    let reads = reads.with_state(state.clone());

    let mut writes = Router::new();
    for (path, handler, _capability) in write_entries {
        writes = writes.route(path, handler);
    }
    let writes = writes
        .layer(from_fn_with_state(
            state.system.clone(),
            idempotency_key_guard,
        ))
        .with_state(state.clone());
    reads
        .merge(writes)
        .merge(identity::router(state.clone()).merge(identity_admin::router(state)))
}

/// 成功封套直出响应。
pub fn ok_response(status: StatusCode, env: Envelope<serde_json::Value>) -> Response {
    (status, Json(env)).into_response()
}

/// A-03 幂等重放：200 加 `Idempotent-Replay: true`。
pub fn replay_response(env: Envelope<serde_json::Value>) -> Response {
    (StatusCode::OK, [("idempotent-replay", "true")], Json(env)).into_response()
}

/// 装配缺位（db 或 kms 未注入）：503 NOT_PROVISIONED。
pub fn not_provisioned(state: &PlatformState, trace: &str) -> ApiError {
    ApiError::new(
        ep_foundation::error::codes::PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
        state.system.next_incident_no(),
        trace.to_string(),
    )
}

/// 库侧错误到 HTTP 错误的映射：已登记码原样上抛，
/// 未预期的一律折叠为 SYSTEM_INTERNAL_ERROR，不泄漏内部形态。
pub fn to_api_error(err: AppError, state: &PlatformState, trace: &str) -> ApiError {
    use ep_foundation::error::codes::*;
    let known = [
        PLATFORM_CONCURRENCY_STALE_VERSION,
        PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
        PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
        PLATFORM_DB_MIGRATION_WINDOW_CONFLICT,
        PLATFORM_DB_MIGRATION_WINDOW_CLOSED,
        PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
        PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
        PLATFORM_KEY_DOMAIN_ROTATION_IN_PROGRESS,
        PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED,
        PLATFORM_KEY_DOMAIN_TRANSITION_INVALID,
        PLATFORM_DB_RLS_CONTEXT_MISSING,
        PLATFORM_DB_LEGAL_ENTITY_MISMATCH,
        PLATFORM_DB_POOL_EXHAUSTED,
        PLATFORM_DB_STATEMENT_TIMEOUT,
        PLATFORM_DB_LOCK_TIMEOUT,
        PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED,
        PLATFORM_DB_MIGRATION_VERSION_MISMATCH,
        PLATFORM_DB_WRITE_SCALE_VIOLATION,
        PLATFORM_DB_APPEND_ONLY_VIOLATION,
        PLATFORM_DB_ROW_VERSION_NOT_BUMPED,
        PLATFORM_DB_REFERENCED_ROW_MISSING,
        PLATFORM_REQUEST_INVALID_PAYLOAD,
        PLATFORM_SENSITIVE_FIELD_NOT_REGISTERED,
        // 阶段 4 身份域（任务 #21）：AUTHN 段九码与 USER_ACCOUNT 段三码。
        PLATFORM_AUTHN_CREDENTIAL_INVALID,
        PLATFORM_AUTHN_ACCOUNT_LOCKED,
        PLATFORM_AUTHN_ACCOUNT_INACTIVE,
        PLATFORM_AUTHN_MFA_REQUIRED,
        PLATFORM_AUTHN_MFA_INVALID,
        PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED,
        PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN,
        PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
        PLATFORM_AUTHN_RATE_LIMITED,
        PLATFORM_USER_ACCOUNT_BATCH_PARTIAL_FAILED,
        PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED,
        PLATFORM_USER_ACCOUNT_PENDING_APPROVAL_TASKS,
    ];
    let code = if known.contains(&err.code) {
        err.code
    } else {
        PLATFORM_SYSTEM_INTERNAL_ERROR
    };
    ApiError::new(code, state.system.next_incident_no(), trace.to_string())
}

#[allow(dead_code)]
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<PlatformState>();
};
