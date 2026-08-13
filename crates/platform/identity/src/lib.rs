//! ep-platform-identity — 账号、口令、MFA、会话、设备、应急账号（04 §4.3/§4.6/§5.2-5.4）。
//!
//! 依赖方向：foundation + platform-authz（只消费）；不依赖 adapter，
//! SQL 落库实现体归 ep-adapter-db-pg（platform_core/identity*.rs 与
//! platform_authz/user_grants.rs）。编译期断言由 `xtask archcheck` 承担。
//!
//! 模块分工：
//! - [`types`]：九张身份表行结构与 CHECK 字面量；
//! - [`config`]：EP__AUTH__* 策略参数（U-B-14 临时取值处逐项标注）；
//! - [`ports`]：持久化端口与审计/事件/告警占位调用面；
//! - [`password`]：Argon2id 哈希校验与口令策略；
//! - [`totp`]：RFC 6238 自实现（HMAC-SHA1，skew ±1）；
//! - [`mfa`]：强制判据、无状态登录挑战、TOTP/X509/WebAuthn 三形态；
//! - [`enrollment`]：MFA 登记用例（TOTP begin/complete 与注销）；
//! - [`session`]：不透明令牌、摘要、上限裁剪与续期合并；
//! - [`login`]：sign-in 九步与 complete-mfa；
//! - [`maintenance`]：过期会话与挑战清理（job-worker 入口）；
//! - [`context_build`]：SecurityContext 19 字段装配。

pub mod account_admin;
pub mod breakglass;
pub mod config;
pub mod context_build;
pub mod enrollment;
pub mod lifecycle;
pub mod login;
pub mod maintenance;
pub mod mfa;
pub mod password;
pub mod ports;
pub mod session;
pub mod totp;
pub mod types;

#[cfg(test)]
mod testutil;
#[cfg(test)]
mod testutil_extra;

pub use account_admin::{AccountAdminService, ImportOutcome, IMPORT_BATCH_MAX_ROWS};
pub use breakglass::BreakglassService;
pub use config::IdentityPolicies;
pub use enrollment::{MfaEnrollmentService, MFA_ENROLLMENT_TTL_SECONDS};
pub use lifecycle::LifecycleService;
pub use login::{
    CompleteMfaRequest, LoginService, SecondFactorProof, SignInOutcome, SignInRequest,
    SignInSuccess, MFA_CHALLENGE_TTL_SECONDS,
};
pub use maintenance::HygieneService;
pub use mfa::MfaChallengeService;
pub use password::PasswordService;
