//! 自检探针的装配适配（02 计划 §7.1、E-11）。
//!
//! 判定逻辑在 ep-platform-runtime，取数在 ep-adapter-db-pg；两侧互不依赖，
//! 本文件把 `DataFoundationCheck` 的结构化取值逐项映射为运行期 `SqlProbe`
//! 端口形态，并实现 `SecretsProbe` 的两段判定。
//!
//! 迁移历史表无 schema 列：schema 归属由二进制内嵌清单按
//! `runtime::migrations::schema_of` 回填；清单之外的行如实报错，
//! 由 `migration-version-matched` 判失败——读不到被测对象绝不判通过。

use std::path::PathBuf;
use std::sync::Arc;

use ep_adapter_db_pg::foundation_check::DataFoundationCheck;
use ep_adapter_db_pg::{PgKeyDomainStore, PgLegalEntityDirectory};
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_runtime::config::{KmsCfg, SecretRef};
use ep_platform_runtime::migrations::schema_of;
use ep_platform_runtime::selfcheck::probe::{
    MigrationRow, ProbeError, RlsState, RolePrivileges, ServerSettings, SqlProbe, TableRls,
};
use ep_platform_runtime::selfcheck::SecretsProbe;
use ep_platform_tenancy::directory::LegalEntityDirectory;

use super::db::resolve_secret;
use super::kms::build_kms_backend;

/// 自检探针的固定追踪标识：32 位十六进制零串。
const PROBE_TRACE_ID: &str = "00000000000000000000000000000000";

/// 把 `DataFoundationCheck` 适配为运行期 `SqlProbe`。
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

/// `secrets-resolvable` 的两段探针。
///
/// 第一段（Blocking）：数据库口令可解引用且密钥后端可构造。
/// 第二段（Degrading）：逐法人切换会话上下文核验密钥域存在——
/// `key_domains` 挂法人行级策略，跨法人枚举只能逐法人取数。
pub struct CoreSecretsProbe {
    secrets_dir: PathBuf,
    db_password_ref: SecretRef,
    kms: KmsCfg,
    directory: Arc<PgLegalEntityDirectory>,
    key_domains: Arc<PgKeyDomainStore>,
}

impl CoreSecretsProbe {
    pub fn new(
        secrets_dir: PathBuf,
        db_password_ref: SecretRef,
        kms: KmsCfg,
        directory: Arc<PgLegalEntityDirectory>,
        key_domains: Arc<PgKeyDomainStore>,
    ) -> Self {
        Self {
            secrets_dir,
            db_password_ref,
            kms,
            directory,
            key_domains,
        }
    }

    /// 逐法人核验用的系统上下文：法人取自目录行，request/trace 固定。
    fn le_ctx(legal_entity_id: Id<LegalEntity>) -> SecurityContext {
        SecurityContext::system(
            legal_entity_id,
            RequestId::new("secrets-probe").expect("固定取值长度合法"),
            TraceId::new(PROBE_TRACE_ID).expect("固定取值形态合法"),
        )
    }
}

#[async_trait::async_trait]
impl SecretsProbe for CoreSecretsProbe {
    async fn backend_available(&self) -> Result<(), ProbeError> {
        resolve_secret(&self.secrets_dir, &self.db_password_ref).map_err(probe_err)?;
        build_kms_backend(&self.kms, &self.secrets_dir)
            .map(|_| ())
            .map_err(probe_err)
    }

    async fn legal_entities_missing_key_domain(&self) -> Result<Vec<Id<LegalEntity>>, ProbeError> {
        let entities = self.directory.list_active().await.map_err(probe_err)?;
        let mut missing = Vec::new();
        for entity in entities {
            let ctx = Self::le_ctx(entity.id);
            let has = self
                .key_domains
                .has_any_domain(&ctx)
                .await
                .map_err(probe_err)?;
            if !has {
                missing.push(entity.id);
            }
        }
        Ok(missing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_id_constant_satisfies_the_frozen_shape() {
        // 32 位小写十六进制是 TraceId 的唯一合法形态，改动即编译期可见的失败。
        assert!(TraceId::new(PROBE_TRACE_ID).is_ok());
        assert!(RequestId::new("secrets-probe").is_ok());
    }
}
