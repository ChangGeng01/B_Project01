//! 迁移文件模型、校验和与历史表——两套执行器共用的唯一出处。
//!
//! ## 结构性冲突记录（逐字保留，供偏离登记引用）
//!
//! 计划 §3.3 指定 refinery 0.8 系列 Runner，但 refinery 0.8.16 的
//! `Migration::unapplied` 把文件名版本号 `parse::<i32>()`，其历史表 DDL 为
//! `version INT4 PRIMARY KEY`，i32::MAX ≈ 2.1e9；而本项目按基线通则第五条与
//! §3.4 采用 `V<YYYYMMDDHHMMSS>` 命名，版本号是 14 位时间戳（≈ 2.0e13），
//! refinery Runner 会对全部迁移文件报 InvalidVersion，无法加载。这是 02 计划
//! 内部的结构性矛盾。经 leader 批准，本工具自建 refinery 语义兼容 Runner：
//! - 历史表四列结构同名：version、name、applied_on、checksum；
//!   version 由 INT4 放宽为 BIGINT（容纳 14 位版本号），其余三列形态与
//!   refinery postgres 驱动逐项一致（见 [`HISTORY_TABLE_COLUMNS`] 与单测锁定）；
//! - 校验和与 refinery 同款算法：SipHasher13 依次喂 name、version、sql
//!   （refinery 喂 i32 版本号，本项目版本号超 i32，按 i64 喂入——除此之外
//!   逐项一致）；事务执行器与 concurrent/ 执行器共用 [`migration_checksum`]，
//!   两套执行器校验和算法严格一致；
//! - applied_on 存 RFC3339 文本，checksum 存 u64 十进制文本，与 refinery
//!   postgres 驱动的存取形态一致；
//! - 每个常规迁移一个事务（refinery 默认行为），concurrent/ 目录走自动提交。
//!
//! 若未来换回 refinery（或其修复版本号宽度），本模块的单测是历史表兼容判据。

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// 一个待执行或已执行的迁移文件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationFile {
    /// 版本号：文件名中 `V` 与 `__` 之间的十进制数（14 位时间戳）。
    pub version: i64,
    /// 历史表 name 列取值：`__` 之后、扩展名之前的部分（与 refinery 同名语义）。
    pub name: String,
    /// 文件全路径。
    pub path: PathBuf,
    /// 是否位于 `concurrent/` 子目录（决定走哪套执行器）。
    pub concurrent: bool,
}

/// 历史表的四列定义，逐项对齐 refinery postgres 驱动形态。
/// 列一 version：refinery 为 INT4，本项目放宽 BIGINT（冲突记录见模块头）。
pub const HISTORY_TABLE_COLUMNS: &str = "(
             version BIGINT PRIMARY KEY,
             name VARCHAR(255),
             applied_on VARCHAR(255),
             checksum VARCHAR(255))";

/// 校验和：与 refinery 0.8.16 同款——SipHash13（即 std 稳定 API
/// `DefaultHasher`，其底层算法就是 SipHash-1-3）依次喂 name、version、sql。
/// refinery 源码用的就是 `std::collections::hash_map::DefaultHasher`，
/// 逐字一致；两套执行器（事务与 concurrent）都必须且只能经本函数算校验和。
pub fn migration_checksum(name: &str, version: i64, sql: &str) -> u64 {
    let mut hasher = DefaultHasher::default();
    name.hash(&mut hasher);
    version.hash(&mut hasher);
    sql.hash(&mut hasher);
    hasher.finish()
}

/// 解析迁移文件名的版本号与名称。文件名形如
/// `V20260901090000__platform_core_create_schema.sql`。
/// 不合法（前缀、双下划线、纯数字段、扩展名任一不符）返回 None。
pub fn parse_migration_filename(file_name: &str) -> Option<(i64, String)> {
    let stem = file_name.strip_suffix(".sql")?;
    let body = stem.strip_prefix('V')?;
    let (digits, name) = body.split_once("__")?;
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if name.is_empty() {
        return None;
    }
    let version = digits.parse::<i64>().ok()?;
    Some((version, name.to_string()))
}

/// 扫描迁移目录：`<dir>/<schema>/*.sql` 为事务路径文件，
/// `<dir>/<schema>/concurrent/*.sql` 为非事务路径文件。
/// 返回按版本号升序的全序；版本号重复即报错（sqlcheck SQL-011 的运行时兜底）。
pub fn scan_migrations(dir: &Path) -> Result<Vec<MigrationFile>, String> {
    let schema_dirs =
        std::fs::read_dir(dir).map_err(|e| format!("迁移目录 {} 不可读：{e}", dir.display()))?;
    let mut files: Vec<MigrationFile> = Vec::new();
    for entry in schema_dirs {
        let entry = entry.map_err(|e| format!("迁移目录条目读取失败：{e}"))?;
        let schema_path = entry.path();
        if !schema_path.is_dir() {
            continue;
        }
        collect_sql(&schema_path, false, &mut files)?;
        let concurrent_dir = schema_path.join("concurrent");
        if concurrent_dir.is_dir() {
            collect_sql(&concurrent_dir, true, &mut files)?;
        }
    }
    files.sort_by_key(|f| f.version);
    for pair in files.windows(2) {
        if pair[0].version == pair[1].version {
            return Err(format!(
                "版本号 {} 重复出现（{} 与 {}），迁移目录已损坏",
                pair[0].version,
                pair[0].path.display(),
                pair[1].path.display()
            ));
        }
    }
    Ok(files)
}

fn collect_sql(dir: &Path, concurrent: bool, out: &mut Vec<MigrationFile>) -> Result<(), String> {
    for entry in
        std::fs::read_dir(dir).map_err(|e| format!("目录 {} 不可读：{e}", dir.display()))?
    {
        let entry = entry.map_err(|e| format!("目录条目读取失败：{e}"))?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !file_name.ends_with(".sql") {
            continue;
        }
        let Some((version, name)) = parse_migration_filename(file_name) else {
            return Err(format!(
                "迁移文件名不合 V<版本号>__<名称>.sql 形态：{}",
                path.display()
            ));
        };
        out.push(MigrationFile {
            version,
            name,
            path,
            concurrent,
        });
    }
    Ok(())
}

/// 建历史表语句（幂等）。schema 与表名已经 CLI 标识符闸校验为小写无引号形态，
/// 直接拼接安全。
pub fn create_history_table_sql(schema: &str, table: &str) -> String {
    format!("CREATE TABLE IF NOT EXISTS {schema}.{table} {HISTORY_TABLE_COLUMNS};")
}

/// 历史表属主归位语句（幂等）。历史表由连接账号（ep_migrator）建成，
/// 而授权收口迁移以 `SET ROLE ep_mod_<schema>` 对 schema 内全部表 GRANT，
/// GRANT 要求执行者是表属主；属主留在连接账号会让授权迁移报 permission denied。
/// 归位到 schema 属主角色与迁移文件 SET ROLE 的属主口径一致。
pub fn align_history_owner_sql(schema: &str, table: &str) -> String {
    format!("ALTER TABLE {schema}.{table} OWNER TO ep_mod_{schema};")
}

/// 插入历史行语句。四列 version/name/applied_on/checksum 与 refinery 同结构。
pub fn insert_history_sql(schema: &str, table: &str) -> String {
    format!(
        "INSERT INTO {schema}.{table} (version, name, applied_on, checksum) \
         VALUES ($1, $2, $3, $4)"
    )
}

/// 读取全部已应用迁移。按 version 升序。
pub fn select_history_sql(schema: &str, table: &str) -> String {
    format!("SELECT version, name, applied_on, checksum FROM {schema}.{table} ORDER BY version ASC")
}

/// 历史表单一版本（最大版本号）。
pub fn select_max_version_sql(schema: &str, table: &str) -> String {
    format!("SELECT max(version) FROM {schema}.{table}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_real_stage2_filename() {
        let (v, name) =
            parse_migration_filename("V20260901090000__platform_core_create_schema.sql")
                .expect("阶段 2 真实文件名必须可解析");
        assert_eq!(v, 20260901090000);
        assert_eq!(name, "platform_core_create_schema");
    }

    #[test]
    fn parse_rejects_bad_forms() {
        assert!(parse_migration_filename("V__x.sql").is_none());
        assert!(parse_migration_filename("V123__").is_none());
        assert!(parse_migration_filename("V12x__y.sql").is_none());
        assert!(parse_migration_filename("U1__y.sql").is_none());
        assert!(parse_migration_filename("V1__y.txt").is_none());
    }

    /// refinery 0.8 版本号是 i32，装不下 14 位时间戳——冲突的逐字证据。
    /// 若哪天本断言翻绿（i32 能装下），说明可以换回 refinery，需重审本模块。
    #[test]
    fn refinery_i32_cannot_hold_stage2_versions() {
        assert!(20260901090000_i64 > i32::MAX as i64);
    }

    /// 历史表四列结构锁定：与 refinery postgres 驱动逐项对齐，
    /// 仅 version 因版本号宽度由 INT4 放宽 BIGINT。
    #[test]
    fn history_table_shape_matches_refinery() {
        let cols = HISTORY_TABLE_COLUMNS;
        assert!(cols.contains("version BIGINT PRIMARY KEY"), "version 主键");
        assert!(cols.contains("name VARCHAR(255)"), "name 与 refinery 同宽");
        assert!(
            cols.contains("applied_on VARCHAR(255)"),
            "applied_on 与 refinery 同宽同文本形态"
        );
        assert!(
            cols.contains("checksum VARCHAR(255)"),
            "checksum 与 refinery 同为十进制文本列"
        );
    }

    /// 校验和算法锁定：SipHash13(DefaultHasher) 喂 name、version、sql 三段，
    /// 顺序固定，换实现即失败——两套执行器据此保证严格一致。
    #[test]
    fn checksum_is_siphash13_over_name_version_sql() {
        let mut h = DefaultHasher::default();
        "platform_core_create_schema".hash(&mut h);
        20260901090000_i64.hash(&mut h);
        "select 1;".hash(&mut h);
        assert_eq!(
            migration_checksum("platform_core_create_schema", 20260901090000, "select 1;"),
            h.finish()
        );
    }

    #[test]
    fn checksum_differs_on_any_input_change() {
        let a = migration_checksum("n", 1, "sql");
        assert_ne!(a, migration_checksum("n2", 1, "sql"));
        assert_ne!(a, migration_checksum("n", 2, "sql"));
        assert_ne!(a, migration_checksum("n", 1, "sql2"));
    }

    #[test]
    fn scan_orders_by_version_and_flags_concurrent() {
        let root = std::env::temp_dir().join(format!(
            "ep-migrate-scan-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let schema = root.join("platform_core");
        let concurrent = schema.join("concurrent");
        std::fs::create_dir_all(&concurrent).expect("建探针目录");
        std::fs::write(schema.join("V0020__a_b.sql"), "select 1;").unwrap();
        std::fs::write(schema.join("V0010__a_a.sql"), "select 1;").unwrap();
        std::fs::write(
            concurrent.join("V0015__a_idx.sql"),
            "create index concurrently i on s.t(c);",
        )
        .unwrap();
        let files = scan_migrations(&root).expect("可扫描");
        let versions: Vec<i64> = files.iter().map(|f| f.version).collect();
        assert_eq!(versions, vec![10, 15, 20]);
        assert!(files[1].concurrent, "concurrent/ 下文件必须标记非事务路径");
        assert!(!files[0].concurrent && !files[2].concurrent);
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn scan_rejects_duplicate_versions() {
        let root = std::env::temp_dir().join(format!(
            "ep-migrate-dup-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let a = root.join("sa");
        let b = root.join("sb");
        std::fs::create_dir_all(&a).unwrap();
        std::fs::create_dir_all(&b).unwrap();
        std::fs::write(a.join("V0001__x.sql"), "select 1;").unwrap();
        std::fs::write(b.join("V0001__y.sql"), "select 1;").unwrap();
        let err = scan_migrations(&root).expect_err("重复版本号必须拒绝");
        assert!(err.contains("重复"));
        std::fs::remove_dir_all(&root).ok();
    }
}
