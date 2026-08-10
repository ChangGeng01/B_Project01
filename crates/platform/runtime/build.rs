//! 迁移清单哈希的编译期常量。
//!
//! 自检项 `migration-version-matched` 比对的是「二进制期望的迁移清单」与
//! 「数据库历史表里实际记着的迁移清单」。前者必须在构建时定死，运行期再算
//! 就等于让被测者自证，因此落在 build.rs。
//!
//! 两侧的被哈希对象都是 `(schema, version, name, checksum)` 四元组序列，
//! 按 schema 升序再按 version 升序排序，逐条以 `\u{1F}` 分隔拼接后取 SHA-256。
//! 文件一侧的 checksum 取归一化正文的 SHA-256，归一化规则是统一 LF、去行尾空白。

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

struct Entry {
    schema: String,
    version: u64,
    name: String,
    checksum: String,
}

fn main() {
    let root = workspace_root();
    let migrations = root.join("db/migrations");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", migrations.display());

    let mut files = Vec::new();
    collect_sql(&migrations, &mut files);
    files.sort();

    let mut entries: Vec<Entry> = files.iter().filter_map(|f| entry_of(&migrations, f)).collect();
    entries.sort_by(|a, b| (&a.schema, a.version).cmp(&(&b.schema, b.version)));

    let mut hasher = Sha256::new();
    for e in &entries {
        for field in [e.schema.as_str(), &e.version.to_string(), e.name.as_str(), e.checksum.as_str()] {
            hasher.update(field.as_bytes());
            hasher.update([0x1f]);
        }
    }
    println!("cargo:rustc-env=EP_MIGRATION_MANIFEST_SHA256={:x}", hasher.finalize());
    println!("cargo:rustc-env=EP_MIGRATION_MANIFEST_ENTRIES={}", entries.len());
    // 目录缺席是一个事实，不是可以静默当成空集的默认值：运行期据此在自检报告
    // 的 detail 里写明「目录不存在」，避免把「没读到」讲成「读到了空集」。
    println!("cargo:rustc-env=EP_MIGRATION_DIR_PRESENT={}", u8::from(migrations.is_dir()));
}

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR 是 <root>/crates/platform/runtime。
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("cargo 必须提供 CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .nth(3)
        .expect("ep-platform-runtime 必须位于 <root>/crates/platform/ 之下")
        .to_path_buf()
}

fn collect_sql(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sql(&path, out);
        } else if path.extension().is_some_and(|e| e == "sql") {
            out.push(path);
        }
    }
}

/// `db/migrations/<schema>/V<version>__<name>.sql`。不合形态的文件不进清单，
/// 由 `xtask sqlcheck` 负责拦，这里不重复实现一套命名判定。
fn entry_of(root: &Path, file: &Path) -> Option<Entry> {
    let rel = file.strip_prefix(root).ok()?;
    let schema = rel.parent()?.file_name()?.to_string_lossy().to_string();
    let stem = file.file_stem()?.to_string_lossy().to_string();
    let rest = stem.strip_prefix('V')?;
    let (version, name) = rest.split_once("__")?;
    let body = std::fs::read_to_string(file).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(normalize(&body).as_bytes());
    Some(Entry {
        schema,
        version: version.parse().ok()?,
        name: name.to_string(),
        checksum: format!("{:x}", hasher.finalize()),
    })
}

fn normalize(body: &str) -> String {
    body.replace("\r\n", "\n").lines().map(str::trim_end).collect::<Vec<_>>().join("\n")
}
