//! 启动自检。注册表落点由技术基线第 7.3 节定死在 `selfcheck/registry.rs`。

pub mod items;
pub mod probe;
pub mod registry;
pub mod secrets;

pub use items::{baseline_registry, BASELINE_ITEMS, REGISTERED_THIS_STAGE};
pub use probe::{
    manifest_sha256, MigrationRow, ProbeError, RlsState, RolePrivileges, ServerSettings, SqlProbe,
    TableRls,
};
pub use registry::{
    DuplicateName, ItemReport, Outcome, SelfCheckItem, SelfCheckRegistry, SelfCheckReport,
    SelfCheckRun, Severity, Verdict,
};
pub use secrets::{SecretsProbe, SecretsResolvable, KEY_DOMAIN_WINDOW_SUBJECT};
