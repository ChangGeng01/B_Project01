//! `platform_authz` schema 的授权域读写面。archcheck 规则
//! `db-pg-one-schema-per-file` 要求访问某 schema 基表的文件落在
//! 与该 schema 同名的目录之下，本目录即该约束的落点。

pub mod config_store;
pub mod snapshot_query;
pub mod user_grants;
