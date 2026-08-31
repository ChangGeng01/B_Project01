//! core-server 两层安全中间件（阶段 4 任务 #23，04 计划 §4.4/§5）。
//!
//! 一、认证层：`Authorization` Bearer 令牌取 SHA-256 摘要查 sessions，
//!     校 expires/idle_expires，滑动续期按 60 秒粒度合并写入（读请求
//!     不逐次写库），消费 ep-platform-identity 既有会话端口。
//! 二、法人校验层（与认证同事务执行，逻辑分两段）：请求的
//!     X-Legal-Entity-Id 对照 user_legal_entity_grants 授权集合，
//!     再与设备 restricted_legal_entity_id 取交集；校验通过后把
//!     核验后的身份写进请求扩展，并注入 `x-ep-*` 头面供既有
//!     extract_context 交接面消费（签名不动）。
//!
//! 会话变量 `app.legal_entity_id` 等四条的写入与连接归还清除由
//! db-pg transact 的 SessionContext 机制承担，本层不拼 SET 语句。
//!
//! PRE_AUTH 白名单（sign-in/complete-mfa/legal-entities 列表/门户
//! sign-in）豁免 Authorization 与 X-Legal-Entity-Id，补偿是登录名
//! +来源地址双维度速率限制（`PLATFORM.AUTHN.RATE_LIMITED`）。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use ep_foundation::error::codes::{
    PLATFORM_AUTHN_ACCOUNT_INACTIVE, PLATFORM_AUTHN_CREDENTIAL_INVALID,
    PLATFORM_AUTHN_DEVICE_NOT_REGISTERED, PLATFORM_AUTHN_RATE_LIMITED,
    PLATFORM_AUTHZ_LEGAL_ENTITY_NOT_GRANTED, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::{AppError, ErrorCode};
use ep_foundation::id::marker::{LegalEntity, Session};
use ep_foundation::id::Id;
use ep_foundation::port::tx::UnitOfWork;
use ep_foundation::security::context::{
    ClientKind, DataScopeTag, DepartmentScope, DeviceId, DutyClass, HumanContextInput, RecordShare,
    RecordShareGrant, RequestId, RoleCode, TraceId,
};
use ep_foundation::security::level::SecurityLevel;
use ep_foundation::security::SecurityContext;
use ep_platform_identity::ports::{
    AccountStore, DeviceStore, SessionStore, UserAuthzQuery, UserAuthzSet,
};
use ep_platform_identity::session::{
    is_session_live, should_write_sliding_extension, token_digest,
};
use ep_platform_identity::types::{AccountStatus, DeviceRow, SessionRow, UserAccountRow};
use ep_platform_obs::MetricsRegistry;
use ep_platform_runtime::http::headers::{is_exempt, is_pre_auth};
use ep_platform_runtime::http::{ApiError, Detail};

use super::{trace_of, PlatformState, ZERO_TRACE};

/// 速率窗口长度（秒）：两维度共用一分钟滑窗。
const RATE_WINDOW_SECONDS: u64 = 60;
/// 登录名维度窗口上限（U-B 临时取值）：防单账号口令爆破。
const LOGIN_NAME_WINDOW_MAX: u32 = 10;
/// 来源地址维度窗口上限（U-B 临时取值）：防单源扫号。
const SOURCE_ADDR_WINDOW_MAX: u32 = 60;
/// PRE_AUTH 请求体读取上限：只为取登录名，超限即拒。
const PRE_AUTH_BODY_LIMIT: usize = 64 * 1024;
/// 中间件侧固定请求标识。
const MW_REQUEST_ID: &str = "core-auth-mw";

/// 登录前速率限制：登录名与来源地址两个维度各自一分钟滑窗计数，
/// 任一维超限即拒。
///
/// F-83：键取自攻击者可控的登录名与 `X-Forwarded-For`，而这是唯一一个**不需要
/// 凭据**就能触达的写路径。原实现只在键被再次访问到时重置其计数，过期后不再出现
/// 的键**永久留在 map 里**——匿名调用方每换一个取值就永久新增一条（每条最坏
/// 数十 KiB 键），进程内存单调涨，机器只有 32GB。现在每次 `allow` 顺带清掉两张表里
/// 所有已过窗口的键，表的驻留量因此被一分钟内的**活跃**键数上界所限，而不是历史
/// 全集。它本身就是防爆破用的，不能自己变成一条内存耗尽路径。
pub struct PreAuthRateLimiter {
    inner: Mutex<RateWindow>,
}

struct RateWindow {
    logins: HashMap<String, WindowCount>,
    addrs: HashMap<String, WindowCount>,
}

struct WindowCount {
    start: Instant,
    count: u32,
}

impl PreAuthRateLimiter {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(RateWindow {
                logins: HashMap::new(),
                addrs: HashMap::new(),
            }),
        }
    }

    /// 两个维度都未超限才放行；放行即各计一次。
    pub fn allow(&self, login_name: Option<&str>, source_addr: &str, now: Instant) -> bool {
        let mut w = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        // 先清过期键，再计数：驻留量 = 当前窗口内的活跃键数，不随历史全集增长。
        evict_expired(&mut w.addrs, now);
        evict_expired(&mut w.logins, now);
        let addr_ok = bump(&mut w.addrs, source_addr, SOURCE_ADDR_WINDOW_MAX, now);
        let login_ok = match login_name {
            Some(name) => bump(&mut w.logins, name, LOGIN_NAME_WINDOW_MAX, now),
            None => true,
        };
        addr_ok && login_ok
    }
}

/// 清掉所有已过滑窗的键。过期键的计数无论如何都会在下次访问时归零，
/// 留着它们只占内存不改变判定，因此可以安全整体删除。
fn evict_expired(map: &mut HashMap<String, WindowCount>, now: Instant) {
    let window = Duration::from_secs(RATE_WINDOW_SECONDS);
    map.retain(|_, w| now.duration_since(w.start) < window);
}

fn bump(map: &mut HashMap<String, WindowCount>, key: &str, max: u32, now: Instant) -> bool {
    let entry = map.entry(key.to_string()).or_insert(WindowCount {
        start: now,
        count: 0,
    });
    if now.duration_since(entry.start) >= Duration::from_secs(RATE_WINDOW_SECONDS) {
        entry.start = now;
        entry.count = 0;
    }
    if entry.count >= max {
        return false;
    }
    entry.count += 1;
    true
}

/// 活跃会话台账：按空闲窗口内核验过的会话登记在案，两项 gauge
/// （ep_authn_active_sessions 与 ep_breakglass_active_sessions）据此刷新。
pub struct SessionTracker {
    inner: Mutex<HashMap<uuid::Uuid, (Instant, bool)>>,
    idle_window: Duration,
    registry: Arc<MetricsRegistry>,
}

impl SessionTracker {
    pub fn new(idle_timeout_seconds: u64, registry: Arc<MetricsRegistry>) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            idle_window: Duration::from_secs(idle_timeout_seconds),
            registry,
        }
    }

    /// 一次会话核验成功后的登记与 gauge 刷新。
    pub fn observe(&self, session_id: uuid::Uuid, is_breakglass: bool) {
        let now = Instant::now();
        let mut seen = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        seen.retain(|_, (t, _)| now.duration_since(*t) < self.idle_window);
        seen.insert(session_id, (now, is_breakglass));
        let total = seen.len();
        let breakglass = seen.values().filter(|(_, b)| *b).count();
        let _ = self
            .registry
            .set_gauge("ep_authn_active_sessions", &[], total as f64);
        let _ = self
            .registry
            .set_gauge("ep_breakglass_active_sessions", &[], breakglass as f64);
    }
}

/// 中间件运行时载体：全部经端口消费身份域，不触碰 crate 内部。
pub struct AuthnAssembly {
    pub uow: Arc<ep_adapter_db_pg::PgUnitOfWork>,
    pub sessions: Arc<dyn SessionStore>,
    pub accounts: Arc<dyn AccountStore>,
    pub devices: Arc<dyn DeviceStore>,
    pub authz_query: Arc<dyn UserAuthzQuery>,
    pub limiter: Arc<PreAuthRateLimiter>,
    pub tracker: Arc<SessionTracker>,
    /// 指标注册表引用：登录尝试计数（ep_authn_login_attempts_total）
    /// 的填充面在端点层，经此写入。
    pub metrics: Arc<MetricsRegistry>,
    /// 滑动续期合并粒度（秒），取 auth.session.sliding_write_granularity_seconds。
    pub sliding_granularity_seconds: u64,
    /// 空闲超时（秒），续期写入的新到期时刻 = now + 本值。
    pub idle_timeout_seconds: u64,
}

/// 认证通过的主体：会话行、账号行、授权集合、设备行与活动法人。
pub struct SessionPrincipal {
    pub session: SessionRow,
    pub account: UserAccountRow,
    pub authz: UserAuthzSet,
    pub device: DeviceRow,
    pub legal_entity_id: Id<LegalEntity>,
}

fn api_err(state: &PlatformState, code: ErrorCode, trace: &str) -> ApiError {
    ApiError::new(code, state.system.next_incident_no(), trace.to_string())
}

fn bearer_of(req: &Request) -> Option<String> {
    req.headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_string)
}

fn header_of(req: &Request, name: &str) -> Option<String> {
    req.headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}

fn source_addr_of(req: &Request) -> String {
    header_of(req, "x-forwarded-for").unwrap_or_else(|| "unknown".to_string())
}

/// 两层中间件的外层入口。系统端点整体豁免；PRE_AUTH 白名单先过
/// 速率限制，携带令牌者照常认证，未携带者按登录前端点放行。
pub async fn authenticate(
    State(state): State<Arc<PlatformState>>,
    mut req: Request,
    next: Next,
) -> Response {
    // 先剥再判：`x-ep-*` 头面只由 apply_principal 在核验成功后注入，
    // 客户端送进来的一律是伪造。不剥的话，任何走到 next.run 而未经
    // apply_principal 的路径（系统豁免、PRE_AUTH 未携带令牌、认证面
    // 未装配）都会把伪造头原样交给处理器的 extract_context（F-78）。
    strip_injected_identity_headers(&mut req);
    let path = req.uri().path().to_string();
    if is_exempt(&path) {
        return next.run(req).await;
    }
    let Some(authn) = state.authn.as_ref() else {
        // 数据库未装配即认证面未注入（unwired-absent）：平台端点
        // 随后在各自处理器按 503 NOT_PROVISIONED 处置。
        return next.run(req).await;
    };
    let trace = trace_of(req.headers());
    if is_pre_auth(&path) {
        match pre_auth_entry(&state, authn, &mut req, &trace).await {
            PreAuthOutcome::Proceed => return next.run(req).await,
            PreAuthOutcome::Reject(resp) => return resp,
            PreAuthOutcome::Authenticated(principal) => {
                apply_principal(&mut req, &principal);
                return next.run(req).await;
            }
        }
    }
    match protected_entry(&state, &req, &trace) {
        Ok(input) => match verify_session(&state, authn, input, &trace).await {
            Ok(principal) => {
                apply_principal(&mut req, &principal);
                next.run(req).await
            }
            Err(e) => e.into_response(),
        },
        Err(e) => e.into_response(),
    }
}

/// PRE_AUTH 白名单里**仍然要求携带令牌**的路径。
///
/// 白名单本身只表示「豁免 `X-Legal-Entity-Id` 与幂等守卫」；是否豁免
/// `Authorization` 是另一件事，必须逐条列明，不能与前者共用一个布尔量。
fn requires_authenticated_caller(path: &str) -> bool {
    path == "/api/v1/platform/identity/me/legal-entities"
}

/// 剥离客户端可能伪造的全部身份头面。
///
/// 这五个头名与 `apply_principal` 注入的一一对应；那里加一个，这里必须
/// 同批加一个，否则新加的那个就是一条新的伪造通道。
fn strip_injected_identity_headers(req: &mut Request) {
    const INJECTED: [&str; 5] = [
        "x-ep-user-id",
        "x-ep-legal-entity-id",
        "x-ep-session-id",
        "x-ep-duty-classes",
        "x-ep-roles",
    ];
    let headers = req.headers_mut();
    for name in INJECTED {
        headers.remove(name);
    }
}

enum PreAuthOutcome {
    Proceed,
    Reject(Response),
    /// 装箱避免大枚举变体：SessionPrincipal 含授权集合，与单元变体
    /// 体积差距过大。
    Authenticated(Box<SessionPrincipal>),
}

/// PRE_AUTH 路径处置：sign-in 两段先过双维度限流；携带令牌者
/// （legal-entities 列表的已登录形态）照常认证。
///
/// `Request<Body>` 非 Sync，`&Request` 不得跨 await 持有：头面取值
/// 在进入异步核验前全部落为自有值。
async fn pre_auth_entry(
    state: &Arc<PlatformState>,
    authn: &Arc<AuthnAssembly>,
    req: &mut Request,
    trace: &str,
) -> PreAuthOutcome {
    let path_owned = req.uri().path().to_string();
    if path_owned != "/api/v1/platform/identity/me/legal-entities" {
        let login = take_login_name(req).await;
        if !authn
            .limiter
            .allow(login.as_deref(), &source_addr_of(req), Instant::now())
        {
            // 限流拒入即一次登录尝试被拒：填充面取八值中语义最近的
            // admission_rejected（准入拒绝类），不新造标签取值。
            let _ = authn.metrics.inc_counter(
                "ep_authn_login_attempts_total",
                &[("outcome", "admission_rejected")],
                1.0,
            );
            return PreAuthOutcome::Reject(
                api_err(state, PLATFORM_AUTHN_RATE_LIMITED, trace).into_response(),
            );
        }
    }
    let Some(token) = bearer_of(req) else {
        // 无令牌时只有真正的登录前端点可以放行。`me/legal-entities`
        // 进白名单的本意是豁免 `X-Legal-Entity-Id`（此时调用方还不知道
        // 自己属哪个法人），**不是**豁免 `Authorization`——注释自己称它
        // 是「已登录形态」。用同一个布尔量豁免两件事，会让它变成匿名端点
        // （F-78）。
        if requires_authenticated_caller(path_owned.as_str()) {
            return PreAuthOutcome::Reject(
                api_err(state, PLATFORM_AUTHN_CREDENTIAL_INVALID, trace).into_response(),
            );
        }
        return PreAuthOutcome::Proceed;
    };
    let input = verify_input_of(req, token);
    match verify_session(state, authn, input, trace).await {
        Ok(principal) => PreAuthOutcome::Authenticated(Box::new(principal)),
        Err(e) => PreAuthOutcome::Reject(e.into_response()),
    }
}

/// 受保护路径处置：令牌必须存在。同步取齐核验入参，异步核验不
/// 再持有请求引用。
fn protected_entry(
    state: &Arc<PlatformState>,
    req: &Request,
    trace: &str,
) -> Result<VerifyInput, ApiError> {
    let token = bearer_of(req).ok_or_else(|| {
        api_err(state, PLATFORM_AUTHN_CREDENTIAL_INVALID, trace).with_details(vec![Detail {
            field: "Authorization".into(),
            reason: "MISSING".into(),
            value: None,
        }])
    })?;
    Ok(verify_input_of(req, token))
}

/// 核验入参：从请求头面一次性取齐自有值，随后异步链路不再触碰
/// 请求本体。
struct VerifyInput {
    token: String,
    requested: Option<Id<LegalEntity>>,
    device_id: String,
}

fn verify_input_of(req: &Request, token: String) -> VerifyInput {
    VerifyInput {
        token,
        requested: requested_le_of(req),
        device_id: header_of(req, "x-device-id").unwrap_or_default(),
    }
}

fn requested_le_of(req: &Request) -> Option<Id<LegalEntity>> {
    header_of(req, "x-legal-entity-id")
        .and_then(|v| uuid::Uuid::parse_str(&v).ok())
        .map(Id::<LegalEntity>::from_uuid)
}

/// 取 sign-in 请求体里的登录名：缓冲请求体后解析，解析失败不
/// 阻断登录流程本身（限流退化为仅来源地址维度），令牌摘要与
/// 口令一律不进日志。
async fn take_login_name(req: &mut Request) -> Option<String> {
    let (parts, body) = std::mem::replace(req, Request::new(Body::empty())).into_parts();
    let bytes = match axum::body::to_bytes(body, PRE_AUTH_BODY_LIMIT).await {
        Ok(b) => b,
        Err(_) => {
            *req = Request::from_parts(parts, Body::from(Vec::new()));
            return None;
        }
    };
    let login = serde_json::from_slice::<serde_json::Value>(&bytes)
        .ok()
        .and_then(|v| {
            v.get("login_name")
                .and_then(|n| n.as_str())
                .map(str::to_string)
        });
    *req = Request::from_parts(parts, Body::from(bytes));
    login
}

/// 认证与法人校验的合并事务：令牌摘要查会话、有效期判定、账号与
/// 设备核验、授权集合读取、法人交集判定、滑动续期一次完成。
async fn verify_session(
    state: &Arc<PlatformState>,
    authn: &Arc<AuthnAssembly>,
    input: VerifyInput,
    trace: &str,
) -> Result<SessionPrincipal, ApiError> {
    let digest = token_digest(&input.token);
    let now = Utc::now();
    let requested = input.requested;
    // 中间件自身事务的法人上下文：sessions 无 RLS 策略，取请求
    // 法人写会话变量；结果法人以授权判定为准。
    let ctx = middleware_ctx(requested, trace);
    let sessions = authn.sessions.clone();
    let accounts = authn.accounts.clone();
    let devices = authn.devices.clone();
    let authz_query = authn.authz_query.clone();
    let granularity = authn.sliding_granularity_seconds;
    let idle_timeout = authn.idle_timeout_seconds;
    let outcome = authn
        .uow
        .transact(&ctx, move |tx| {
            Box::pin(principal_tx_body(
                tx,
                sessions,
                accounts,
                devices,
                authz_query,
                digest,
                now,
                input.device_id,
                requested,
                granularity,
                idle_timeout,
            ))
        })
        .await
        .map_err(|e| map_verify_error(e, state, trace))?;
    authn
        .tracker
        .observe(outcome.session.id, outcome.session.is_breakglass);
    Ok(outcome)
}

/// 事务执行体：认证五步与法人交集、滑动续期。
#[allow(clippy::too_many_arguments)]
async fn principal_tx_body(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    sessions: Arc<dyn SessionStore>,
    accounts: Arc<dyn AccountStore>,
    devices: Arc<dyn DeviceStore>,
    authz_query: Arc<dyn UserAuthzQuery>,
    digest: [u8; 32],
    now: chrono::DateTime<Utc>,
    device_id_header: String,
    requested: Option<Id<LegalEntity>>,
    granularity: u64,
    idle_timeout: u64,
) -> Result<SessionPrincipal, AppError> {
    let row = sessions
        .find_active_by_digest(tx, &digest)
        .await?
        .ok_or_else(not_granted_err)?;
    if !is_session_live(&row, now) {
        return Err(not_granted_err());
    }
    let account = accounts
        .get(tx, row.user_id)
        .await?
        .ok_or_else(not_granted_err)?;
    if account.status != AccountStatus::Active {
        return Err(inactive_err());
    }
    let device = check_device(tx, &*devices, &row, &device_id_header).await?;
    let set = authz_query
        .load_user_authz(tx, row.user_id, account.home_legal_entity_id)
        .await?;
    let le = pick_legal_entity(requested, &row, &set, &device)?;
    if should_write_sliding_extension(row.last_seen_at, now, granularity) {
        let secs = i64::try_from(idle_timeout).unwrap_or(i64::MAX);
        sessions
            .extend_idle(tx, &[row.id], now + chrono::Duration::seconds(secs))
            .await?;
    }
    Ok(SessionPrincipal {
        session: row,
        account,
        authz: set,
        device,
        legal_entity_id: le,
    })
}

fn middleware_ctx(requested: Option<Id<LegalEntity>>, trace: &str) -> SecurityContext {
    let le = requested.unwrap_or_else(|| Id::<LegalEntity>::from_uuid(uuid::Uuid::nil()));
    let request = RequestId::new(MW_REQUEST_ID)
        .unwrap_or_else(|_| RequestId::new("platform-endpoint").expect("固定取值合法"));
    let trace_id =
        TraceId::new(trace).unwrap_or_else(|_| TraceId::new(ZERO_TRACE).expect("零串合法"));
    SecurityContext::system(le, request, trace_id)
}

fn not_granted_err() -> AppError {
    AppError::new(PLATFORM_AUTHN_CREDENTIAL_INVALID, "会话令牌无效或已过期")
}

fn inactive_err() -> AppError {
    AppError::new(PLATFORM_AUTHN_ACCOUNT_INACTIVE, "账号非启用状态")
}

/// 设备核验：请求声称的设备必须在该用户名下 ACTIVE，且与会话
/// 绑定的设备行一致。
async fn check_device(
    tx: &mut dyn ep_foundation::port::tx::Tx,
    devices: &dyn DeviceStore,
    session: &SessionRow,
    device_id_header: &str,
) -> Result<DeviceRow, AppError> {
    let device = devices
        .find_active(tx, session.user_id, device_id_header)
        .await?
        .ok_or_else(|| AppError::new(PLATFORM_AUTHN_DEVICE_NOT_REGISTERED, "设备未登记"))?;
    if device.id != session.user_device_row_id {
        return Err(AppError::new(
            PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
            "设备与会话绑定不符",
        ));
    }
    Ok(device)
}

/// 法人交集判定：请求法人（缺省取会话活动法人）必须在授权集合内，
/// 且与设备限定法人一致（设备限定单法人时取交集仅此一值）。
fn pick_legal_entity(
    requested: Option<Id<LegalEntity>>,
    session: &SessionRow,
    set: &UserAuthzSet,
    device: &DeviceRow,
) -> Result<Id<LegalEntity>, AppError> {
    let le = requested.unwrap_or(session.active_legal_entity_id);
    let granted = set.legal_entity_ids.contains(&le);
    let device_ok = match device.restricted_legal_entity_id {
        Some(restricted) => restricted == le,
        None => true,
    };
    if granted && device_ok {
        Ok(le)
    } else {
        Err(AppError::new(
            PLATFORM_AUTHZ_LEGAL_ENTITY_NOT_GRANTED,
            "请求法人不在授权集合与设备交集内",
        ))
    }
}

/// 库侧错误到中间件错误的映射：已登记码原样上抛，其余折叠为
/// SYSTEM_INTERNAL_ERROR，不泄漏内部形态。
fn map_verify_error(err: AppError, state: &PlatformState, trace: &str) -> ApiError {
    let known = [
        PLATFORM_AUTHN_CREDENTIAL_INVALID,
        PLATFORM_AUTHN_ACCOUNT_INACTIVE,
        PLATFORM_AUTHN_DEVICE_NOT_REGISTERED,
        PLATFORM_AUTHZ_LEGAL_ENTITY_NOT_GRANTED,
    ];
    let code = if known.contains(&err.code) {
        err.code
    } else {
        PLATFORM_SYSTEM_INTERNAL_ERROR
    };
    api_err(state, code, trace)
}

/// 核验结果写进请求：扩展供需要的处理器直接取用，`x-ep-*` 头面
/// 供既有 extract_context 交接面消费（签名不动）。
fn apply_principal(req: &mut Request, p: &SessionPrincipal) {
    let headers = req.headers_mut();
    let insert = |headers: &mut axum::http::HeaderMap, name: &str, value: &str| {
        if let (Ok(n), Ok(v)) = (
            axum::http::HeaderName::from_bytes(name.as_bytes()),
            value.parse(),
        ) {
            headers.insert(n, v);
        }
    };
    insert(headers, "x-ep-user-id", &p.account.id.as_uuid().to_string());
    insert(
        headers,
        "x-ep-legal-entity-id",
        &p.legal_entity_id.as_uuid().to_string(),
    );
    insert(headers, "x-ep-session-id", &p.session.id.to_string());
    insert(headers, "x-ep-duty-classes", &duty_list_of(&p.authz));
    insert(headers, "x-ep-roles", &p.authz.role_codes.join(","));
    let ctx = build_context(req, p);
    if let Some(ctx) = ctx {
        req.extensions_mut().insert(ctx);
    }
}

fn duty_list_of(set: &UserAuthzSet) -> String {
    set.duty_classes
        .iter()
        .map(|d| match d {
            DutyClass::System => "SYSTEM",
            DutyClass::Data => "DATA",
            DutyClass::Security => "SECURITY",
            DutyClass::Audit => "AUDIT",
            DutyClass::Key => "KEY",
            DutyClass::Config => "CONFIG",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn client_of(req: &Request) -> ClientKind {
    match header_of(req, "x-client").as_deref() {
        Some("win") => ClientKind::Win,
        Some("mac") => ClientKind::Mac,
        Some("ios") => ClientKind::Ios,
        Some("android") => ClientKind::Android,
        Some("portal") => ClientKind::Portal,
        _ => ClientKind::Ops,
    }
}

/// 由核验结果构造冻结的安全上下文；形态非法的头面取值退化为
/// 固定合法常量，不因旁路头面阻断已认证的请求。
fn build_context(req: &Request, p: &SessionPrincipal) -> Option<SecurityContext> {
    let device_id = DeviceId::new(&p.device.device_id)
        .unwrap_or_else(|_| DeviceId::new("platform-endpoint").expect("固定取值合法"));
    let request_id = header_of(req, "x-ep-request-id")
        .and_then(|v| RequestId::new(&v).ok())
        .unwrap_or_else(|| RequestId::new(MW_REQUEST_ID).expect("固定取值合法"));
    let trace = trace_of(req.headers());
    let trace_id =
        TraceId::new(&trace).unwrap_or_else(|_| TraceId::new(ZERO_TRACE).expect("零串合法"));
    let roles: Vec<RoleCode> = p
        .authz
        .role_codes
        .iter()
        .filter_map(|r| RoleCode::new(r).ok())
        .collect();
    let tags: Vec<DataScopeTag> = p
        .authz
        .data_scope_tags
        .iter()
        .filter_map(|t| DataScopeTag::new(t).ok())
        .collect();
    let shares: Vec<RecordShare> = p
        .authz
        .record_shares
        .iter()
        .map(|(object_type, object_id)| RecordShare {
            object_type: Arc::from(object_type.as_str()),
            object_id: *object_id,
            grant: RecordShareGrant::Read,
        })
        .collect();
    Some(SecurityContext::human(HumanContextInput {
        user_id: p.account.id,
        session_id: Id::<Session>::from_uuid(p.session.id),
        legal_entity_id: p.legal_entity_id,
        device_id,
        client: client_of(req),
        clearance_level: SecurityLevel::from_code(p.account.clearance_level)
            .unwrap_or(SecurityLevel::Internal),
        roles: roles.into(),
        duty_classes: p.authz.duty_classes.clone().into(),
        department_scope: DepartmentScope::Explicit(p.authz.department_ids.clone().into()),
        position_ids: p.authz.position_ids.clone().into(),
        project_scope: p.authz.project_ids.clone().into(),
        customer_scope: p.authz.customer_ids.clone().into(),
        record_shares: shares.into(),
        data_scope_tags: tags.into(),
        snapshot_version: p.authz.snapshot_version,
        is_breakglass: p.session.is_breakglass,
        request_id,
        trace_id,
    }))
}

/// 编译期断言：中间件 future 必须 Send + 'static（axum from_fn 要求；
/// `Request<Body>` 非 Sync，`&Request` 不得跨 await 持有）。
#[allow(dead_code)]
fn _assert_authenticate_future_is_send(
    state: Arc<PlatformState>,
    req: Request,
    next: Next,
) -> impl std::future::Future<Output = Response> + Send + 'static {
    authenticate(State(state), req, next)
}

#[allow(dead_code)]
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AuthnAssembly>();
    assert_send_sync::<SessionPrincipal>();
    assert_send_sync::<PreAuthRateLimiter>();
    assert_send_sync::<SessionTracker>();
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limiter_blocks_after_window_max() {
        let limiter = PreAuthRateLimiter::new();
        let now = Instant::now();
        for _ in 0..LOGIN_NAME_WINDOW_MAX {
            assert!(limiter.allow(Some("alice"), "10.0.0.1", now));
        }
        assert!(!limiter.allow(Some("alice"), "10.0.0.2", now));
        // 另一登录名不受影响（地址维度仍在窗口内计数）。
        assert!(limiter.allow(Some("bob"), "10.0.0.3", now));
    }

    /// 过期键必须被清出，否则匿名调用方换键即可让内存单调涨。
    #[test]
    fn rate_limiter_evicts_expired_keys_instead_of_growing_unbounded() {
        let limiter = PreAuthRateLimiter::new();
        let t0 = Instant::now();
        // 5000 个各不相同的来源地址，每个只出现一次。
        for i in 0..5000 {
            assert!(limiter.allow(None, &format!("10.0.{}.{}", i / 256, i % 256), t0));
        }
        {
            let w = limiter.inner.lock().unwrap();
            assert_eq!(w.addrs.len(), 5000, "同一窗口内的键都在");
        }
        // 越过窗口后再来一个键：一次 allow 就应把此前 5000 个过期键全清掉。
        let later = t0 + Duration::from_secs(RATE_WINDOW_SECONDS + 1);
        assert!(limiter.allow(None, "10.9.9.9", later));
        {
            let w = limiter.inner.lock().unwrap();
            assert_eq!(w.addrs.len(), 1, "过期键必须被清出，只剩当前这一个");
        }
    }

    #[test]
    fn rate_limiter_window_resets_after_expiry() {
        let limiter = PreAuthRateLimiter::new();
        let now = Instant::now();
        for _ in 0..SOURCE_ADDR_WINDOW_MAX {
            assert!(limiter.allow(None, "10.9.9.9", now));
        }
        assert!(!limiter.allow(None, "10.9.9.9", now));
        let later = now + Duration::from_secs(RATE_WINDOW_SECONDS + 1);
        assert!(limiter.allow(None, "10.9.9.9", later));
    }

    /// 伪造的 `x-ep-*` 必须在最外层被剥掉。
    ///
    /// 反例是这条修复的全部理由：不剥的话，任何走到 `next.run` 而未经
    /// `apply_principal` 的路径都会把客户端送来的身份头原样交给处理器。
    #[test]
    fn injected_identity_headers_are_stripped_from_inbound_requests() {
        let mut req = Request::builder()
            .uri("/api/v1/platform/identity/me/legal-entities")
            .header("x-ep-user-id", "00000000-0000-7000-8000-000000000009")
            .header("x-ep-legal-entity-id", "00000000-0000-7000-8000-00000000000a")
            .header("x-ep-session-id", "forged")
            .header("x-ep-duty-classes", "SECURITY")
            .header("x-ep-roles", "SYSTEM")
            .header("x-client", "ops")
            .body(Body::empty())
            .expect("构造请求");

        strip_injected_identity_headers(&mut req);

        for name in [
            "x-ep-user-id",
            "x-ep-legal-entity-id",
            "x-ep-session-id",
            "x-ep-duty-classes",
            "x-ep-roles",
        ] {
            assert!(
                req.headers().get(name).is_none(),
                "{name} 未被剥离，伪造头可直达处理器"
            );
        }
        // 非身份头不受影响。
        assert_eq!(
            req.headers().get("x-client").map(|v| v.as_bytes()),
            Some(&b"ops"[..])
        );
    }

    /// 剥离清单必须与 `apply_principal` 注入的头名逐一对应。
    ///
    /// 那边加一个这边不加，新加的那个就是一条新的伪造通道；此断言让
    /// 这种漏改当场红。
    #[test]
    fn strip_list_covers_every_header_apply_principal_injects() {
        let src = include_str!("middleware.rs");
        let injected: std::collections::BTreeSet<&str> = src
            .lines()
            .filter_map(|l| l.trim().strip_prefix("insert(headers, \""))
            .filter_map(|r| r.split('"').next())
            .collect();
        let stripped: std::collections::BTreeSet<&str> = [
            "x-ep-user-id",
            "x-ep-legal-entity-id",
            "x-ep-session-id",
            "x-ep-duty-classes",
            "x-ep-roles",
        ]
        .into_iter()
        .collect();
        assert!(
            !injected.is_empty(),
            "取不到 apply_principal 的注入头名，判定未做出"
        );
        assert!(
            injected.is_subset(&stripped),
            "apply_principal 注入了未被剥离的头：{:?}",
            injected.difference(&stripped).collect::<Vec<_>>()
        );
    }

    /// 白名单里的 `me/legal-entities` 仍要求携带令牌。
    #[test]
    fn legal_entities_is_whitelisted_but_still_requires_a_token() {
        assert!(is_pre_auth("/api/v1/platform/identity/me/legal-entities"));
        assert!(requires_authenticated_caller(
            "/api/v1/platform/identity/me/legal-entities"
        ));
        // 真正的登录前三段不要求令牌。
        for p in [
            "/api/v1/platform/sessions/actions/sign-in",
            "/api/v1/platform/sessions/actions/complete-mfa",
            "/api/v1/platform/portal/sessions/actions/sign-in",
        ] {
            assert!(is_pre_auth(p));
            assert!(!requires_authenticated_caller(p), "{p} 不应要求令牌");
        }
    }

    #[test]
    fn duty_list_serializes_in_uppercase_form() {
        let set = UserAuthzSet {
            duty_classes: vec![DutyClass::Security, DutyClass::Audit],
            ..Default::default()
        };
        assert_eq!(duty_list_of(&set), "SECURITY,AUDIT");
    }
}
