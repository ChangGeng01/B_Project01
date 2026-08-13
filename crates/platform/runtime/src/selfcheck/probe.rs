//! SQL 类自检项的被测对象端口。
//!
//! 四项 SQL 自检的判定逻辑落在 platform，取数落在 adapter：`ep-platform-runtime`
//! 不得依赖任何 `ep-adapter-*`（archcheck 的 platform-no-adapter 会拦），
//! 因此这里只声明端口，实现由 apps 在 `wiring/` 目录下注入。
//!
//! 端口方法按自检项切分而不是按 SQL 语句切分：一个自检项一次取数，
//! 判定逻辑与取数方式解耦，判定的正负两条分支才能不靠数据库跑起来。

use sha2::{Digest, Sha256};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProbeError(pub String);

impl std::fmt::Display for ProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ProbeError {}

/// `database-reachable` 的被测取值。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSettings {
    pub server_version: String,
    pub timezone: String,
    pub max_connections: u32,
    pub max_wal_senders: u32,
    pub max_replication_slots: u32,
}

/// 迁移历史表的一行。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MigrationRow {
    pub schema: String,
    pub version: u64,
    pub name: String,
    pub checksum: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableRls {
    pub schema: String,
    pub table: String,
    pub enabled: bool,
    pub forced: bool,
}

/// `rls-enabled-and-forced` 的被测取值。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RlsState {
    /// 全部带 `legal_entity_id` 列的表。
    pub legal_entity_tables: Vec<TableRls>,
    pub current_role_bypassrls: bool,
    pub current_role_superuser: bool,
}

/// `runtime-role-privileges-bounded` 的被测取值。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RolePrivileges {
    /// 当前角色具备 CREATE 权限的 schema，非空即越界。
    pub schemas_with_create: Vec<String>,
    pub rolcreaterole: bool,
    pub rolcreatedb: bool,
}

#[async_trait::async_trait]
pub trait SqlProbe: Send + Sync {
    async fn server_settings(&self) -> Result<ServerSettings, ProbeError>;
    async fn migration_rows(&self) -> Result<Vec<MigrationRow>, ProbeError>;
    async fn rls_state(&self) -> Result<RlsState, ProbeError>;
    async fn role_privileges(&self) -> Result<RolePrivileges, ProbeError>;
}

/// 迁移清单哈希。与 build.rs 同一拼接规则（四元组按 schema、version 排序，
/// 逐字段 `\u{1F}` 分隔），两处改一处必须改另一处。checksum 列取库内原文，
/// 与 build.rs 按同一算法（迁移工具的 SipHash-1-3）算出的取值对齐。
pub fn manifest_sha256(rows: &[MigrationRow]) -> String {
    let mut sorted: Vec<&MigrationRow> = rows.iter().collect();
    sorted.sort_by(|a, b| (&a.schema, a.version).cmp(&(&b.schema, b.version)));
    let mut hasher = Sha256::new();
    for r in sorted {
        for field in [
            r.schema.as_str(),
            &r.version.to_string(),
            r.name.as_str(),
            r.checksum.as_str(),
        ] {
            hasher.update(field.as_bytes());
            hasher.update([0x1f]);
        }
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(schema: &str, version: u64) -> MigrationRow {
        MigrationRow {
            schema: schema.into(),
            version,
            name: format!("m{version}"),
            checksum: format!("c{version}"),
        }
    }

    #[test]
    fn empty_manifest_hashes_to_the_empty_sha256() {
        // 空集的哈希必须与 build.rs 在没有迁移文件时算出的取值一致。
        assert_eq!(
            manifest_sha256(&[]),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn ordering_of_rows_does_not_change_the_hash() {
        let a = vec![row("platform_core", 1), row("mdm", 2), row("mdm", 1)];
        let b = vec![row("mdm", 1), row("mdm", 2), row("platform_core", 1)];
        assert_eq!(manifest_sha256(&a), manifest_sha256(&b));
    }

    // 负样例断言的是哈希这条规则本身：任何一列变了，清单哈希必须变。
    #[test]
    fn a_changed_checksum_changes_the_manifest_hash() {
        let base = vec![row("mdm", 1)];
        let mut tampered = base.clone();
        tampered[0].checksum = "tampered".into();
        assert_ne!(manifest_sha256(&base), manifest_sha256(&tampered));

        let mut renamed = base.clone();
        renamed[0].name = "other".into();
        assert_ne!(manifest_sha256(&base), manifest_sha256(&renamed));
    }
}
