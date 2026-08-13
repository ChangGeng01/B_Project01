//! `platform_core` schema 的仓储实现。archcheck 规则
//! `db-pg-one-schema-per-file` 要求访问某 schema 基表的文件必须落在
//! 与该 schema 同名的目录或文件之下，本目录即该约束的落点。

pub mod guard;
pub mod identity_accounts;
pub mod identity_breakglass;
pub mod identity_sessions;
pub mod key_domain;
pub mod sensitive_fields;
pub mod tenancy;
pub mod windows;
