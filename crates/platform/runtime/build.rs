//! 迁移清单哈希与内嵌迁移文件清单的编译期常量。
//!
//! 自检项 `migration-version-matched` 比对的是「二进制期望的迁移清单」与
//! 「数据库历史表里实际记着的迁移清单」。前者必须在构建时定死，运行期再算
//! 就等于让被测者自证，因此落在 build.rs。
//!
//! 两侧的被哈希对象都是 `(schema, version, name, checksum)` 四元组序列，
//! 按 schema 升序再按 version 升序排序，逐条以 `\u{1F}` 分隔拼接后取 SHA-256。
//!
//! 条目里的 `checksum` 逐字取迁移工具写入历史表的取值：SipHash-1-3
//! （std 稳定 API `DefaultHasher`）依次喂 name、version、sql 原文三段后的
//! u64 十进制文本（tools/migrate 的 `migration_checksum`）。阶段 1 曾在此处
//! 另算归一化正文的 SHA-256，与库内取值永不相等，自检在活库上必然失败，
//! 阶段 2 按「两侧同一算法」修正；文件读取与迁移工具一致取原文不归一化。
//!
//! 另按 A-08 输出 `EP_MIGRATION_FILE_LIST`：逐行 `schema\u{1F}version\u{1F}name\u{1F}T|C`，
//! 行间以 `\u{1E}` 分隔，供运行期推导每条历史记录的执行路径
//! （TRANSACTIONAL/CONCURRENT）与二进制期望版本号。`concurrent/` 子目录下的
//! 文件归其父 schema 并标记 C，其余标记 T。

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

struct Entry {
    schema: String,
    version: u64,
    name: String,
    checksum: String,
    concurrent: bool,
}

fn main() {
    let root = workspace_root();
    let migrations = root.join("db/migrations");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", migrations.display());

    let mut files = Vec::new();
    collect_sql(&migrations, &mut files);
    files.sort();

    let mut entries: Vec<Entry> = files
        .iter()
        .filter_map(|f| entry_of(&migrations, f))
        .collect();
    entries.sort_by(|a, b| (&a.schema, a.version).cmp(&(&b.schema, b.version)));

    let mut hasher = Sha256::new();
    for e in &entries {
        for field in [
            e.schema.as_str(),
            &e.version.to_string(),
            e.name.as_str(),
            e.checksum.as_str(),
        ] {
            hasher.update(field.as_bytes());
            hasher.update([0x1f]);
        }
    }
    println!(
        "cargo:rustc-env=EP_MIGRATION_MANIFEST_SHA256={:x}",
        hasher.finalize()
    );
    println!(
        "cargo:rustc-env=EP_MIGRATION_MANIFEST_ENTRIES={}",
        entries.len()
    );
    // 目录缺席是一个事实，不是可以静默当成空集的默认值：运行期据此在自检报告
    // 的 detail 里写明「目录不存在」，避免把「没读到」讲成「读到了空集」。
    println!(
        "cargo:rustc-env=EP_MIGRATION_DIR_PRESENT={}",
        u8::from(migrations.is_dir())
    );

    // A-08 的内嵌文件清单按版本号全序输出（迁移工具保证版本号全局唯一）。
    let mut by_version: Vec<&Entry> = entries.iter().collect();
    by_version.sort_by_key(|e| e.version);
    let list = by_version
        .iter()
        .map(|e| {
            format!(
                "{}\u{1F}{}\u{1F}{}\u{1F}{}",
                e.schema,
                e.version,
                e.name,
                if e.concurrent { "C" } else { "T" }
            )
        })
        .collect::<Vec<_>>()
        .join("\u{1E}");
    println!("cargo:rustc-env=EP_MIGRATION_FILE_LIST={list}");
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR 是 <root>/crates/platform/runtime。
    let manifest = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("cargo 必须提供 CARGO_MANIFEST_DIR"),
    );
    manifest
        .ancestors()
        .nth(3)
        .expect("ep-platform-runtime 必须位于 <root>/crates/platform/ 之下")
        .to_path_buf()
}

fn collect_sql(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sql(&path, out);
        } else if path.extension().is_some_and(|e| e == "sql") {
            out.push(path);
        }
    }
}

/// `db/migrations/<schema>/V<version>__<name>.sql` 或
/// `db/migrations/<schema>/concurrent/V<version>__<name>.sql`。
/// 不合形态的文件不进清单，由 `xtask sqlcheck` 负责拦，这里不重复实现一套命名判定。
fn entry_of(root: &Path, file: &Path) -> Option<Entry> {
    let rel = file.strip_prefix(root).ok()?;
    let parent = rel.parent()?;
    // concurrent/ 子目录的文件归属父 schema 并标记非事务路径。
    let (schema_dir, concurrent) = if parent.file_name()?.to_string_lossy() == "concurrent" {
        (parent.parent()?, true)
    } else {
        (parent, false)
    };
    let schema = schema_dir.file_name()?.to_string_lossy().to_string();
    let stem = file.file_stem()?.to_string_lossy().to_string();
    let rest = stem.strip_prefix('V')?;
    let (version, name) = rest.split_once("__")?;
    let version: u64 = version.parse().ok()?;
    let sql = std::fs::read_to_string(file).ok()?;
    // 与 tools/migrate 的 migration_checksum 逐字一致：SipHash-1-3 依次喂
    // name、version、sql 原文。u64 与 i64 的 Hash 表示同宽，取值互通。
    let mut h = DefaultHasher::default();
    name.hash(&mut h);
    (version as i64).hash(&mut h);
    sql.hash(&mut h);
    Some(Entry {
        schema,
        version,
        name: name.to_string(),
        checksum: h.finish().to_string(),
        concurrent,
    })
}
