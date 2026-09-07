//! `ep_ops_ro` 运维池的 SQL 基座自检适配。
//!
//! 判定逻辑保持在 ep-platform-runtime，本文件只把取数结果转为
//! `SqlProbe` 形态。生产迁移必须为 `ep_ops_ro` 交付精确的自检取数
//! 权限；权限缺失时查询失败并阻断启动，不伪造通过。

use std::sync::Arc;

use ep_adapter_db_pg::foundation_check::DataFoundationCheck;
use ep_platform_runtime::migrations::schema_of;
use ep_platform_runtime::selfcheck::probe::{
    MigrationRow, ProbeError, RlsState, RolePrivileges, ServerSettings, SqlProbe, TableRls,
};

pub struct FoundationProbeAdapter {
    check: Arc<dyn DataFoundationCheck>,
}

impl FoundationProbeAdapter {
    pub fn new(check: Arc<dyn DataFoundationCheck>) -> Self {
        Self { check }
    }
}

fn probe_err(e: impl std::fmt::Display) -> ProbeError {
    ProbeError(e.to_string())
}

#[async_trait::async_trait]
impl SqlProbe for FoundationProbeAdapter {
    async fn server_settings(&self) -> Result<ServerSettings, ProbeError> {
        let s = self.check.server_settings().await.map_err(probe_err)?;
        Ok(ServerSettings {
            server_version: s.server_version,
            timezone: s.timezone,
            max_connections: s.max_connections,
            max_wal_senders: s.max_wal_senders,
            max_replication_slots: s.max_replication_slots,
        })
    }

    async fn migration_rows(&self) -> Result<Vec<MigrationRow>, ProbeError> {
        let rows = self.check.migration_rows().await.map_err(probe_err)?;
        rows.into_iter()
            .map(|r| {
                let schema = schema_of(r.version, &r.name).ok_or_else(|| {
                    ProbeError(format!("迁移历史行 {} 不在二进制内嵌清单内", r.version))
                })?;
                Ok(MigrationRow {
                    schema: schema.to_string(),
                    version: r.version,
                    name: r.name,
                    checksum: r.checksum,
                })
            })
            .collect()
    }

    async fn rls_state(&self) -> Result<RlsState, ProbeError> {
        let s = self.check.rls_state().await.map_err(probe_err)?;
        Ok(RlsState {
            legal_entity_tables: s
                .legal_entity_tables
                .into_iter()
                .map(|t| TableRls {
                    schema: t.schema,
                    table: t.table,
                    enabled: t.enabled,
                    forced: t.forced,
                })
                .collect(),
            current_role_bypassrls: s.current_role_bypassrls,
            current_role_superuser: s.current_role_superuser,
        })
    }

    async fn role_privileges(&self) -> Result<RolePrivileges, ProbeError> {
        let p = self.check.role_privileges().await.map_err(probe_err)?;
        Ok(RolePrivileges {
            schemas_with_create: p.schemas_with_create,
            rolcreaterole: p.rolcreaterole,
            rolcreatedb: p.rolcreatedb,
        })
    }
}
