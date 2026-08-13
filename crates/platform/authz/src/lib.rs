//! ep-platform-authz：授权判定五构件、快照运行形态、SoD 校验与
//! AUTHZ 类配置项 applier（04 计划 §4.1/§4.2/§4.5/§4.7/§4.8）。
//!
//! 依赖方向：foundation + tenancy + release 端口；不依赖 identity、
//! 不依赖 adapter。SQL 渲染与落库实现体归 ep-adapter-db-pg，
//! 本 crate 只持有谓词构造与端口消费面。

pub mod admission;
pub mod applier;
pub mod decider;
pub mod field;
pub mod metrics;
pub mod reauth;
pub mod scope;
pub mod snapshot;
pub mod sod;
pub mod spec;
pub mod types;

pub use admission::{AdmissionConfig, AdmissionGate, AdmissionPermit};
pub use applier::{
    register_authz_appliers, AuthzConfigWriteStore, AuthzFieldGrantApplier, AuthzPolicyApplier,
    AuthzRoleApplier,
};
pub use decider::{AccessDecider, Verdict};
pub use field::{FieldProjector, NoSensitiveFields, SensitiveFieldInfo, SensitiveFieldLookup};
pub use metrics::{AuthzMetricsSink, SilentMetricsSink};
pub use reauth::{
    canonical_amount, subject_digest, ChallengeRecord, ChallengeStatus, IssuedChallenge,
    ReauthChallengeStore, ReauthGate, ReauthSubject,
};
pub use scope::{CompiledScope, ScopeCompiler, ScopeConfig};
pub use snapshot::{
    AuthzConfigVersionQuery, AuthzSnapshotHolder, DegradationWindowOpener, EffectiveVersion,
    EntityAuthzData, ReloadOutcome, SnapshotReloader,
};
pub use types::{Action, Decision, DenyReason, HighRiskOperation, RecordScope};
