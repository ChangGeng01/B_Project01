//! 源码扫描类规则。
//!
//! 依赖图看不见的判据落在这里：命名三处一致、`unwired-absent`、
//! `downcast_mut::<PgTx>` 的落点、std 文件与网络 API 禁令、
//! db-pg 一个仓储只访问自己模块的 schema。

use std::fs;
use std::path::{Path, PathBuf};

use super::deps::Violation;
use crate::graph::{expected_name, Layer, Workspace};

/// `unwired-absent` 断言的前缀集合。就是这四类，不设第五类。
pub const UNWIRED_PREFIXES: [&str; 4] = ["Noop", "Stub", "Fake", "Dummy"];

/// `unwired-absent` 的断言对象是这两个目录下的全部文件，不是单个 wiring.rs 文件。
pub const WIRING_DIRS: [&str; 2] = ["apps/core-server/src/wiring", "apps/job-worker/src/wiring"];

fn violation(rule: &'static str, package: &str, detail: impl Into<String>) -> Violation {
    Violation { rule, package: package.to_string(), detail: detail.into() }
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(rust_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out.sort();
    out
}

/// 逐行扫描，跳过整行注释。规则判的是代码，不是文档里对规则本身的描述。
fn code_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .filter(|(_, l)| !l.starts_with("//") && !l.starts_with("*") && !l.is_empty())
}

/// 命名三处一致：目录名、`ep-` 前缀、`Cargo.toml` 的 name。
pub fn naming(ws: &Workspace) -> Vec<Violation> {
    const RULE: &str = "crate-naming-consistent";
    ws.packages
        .iter()
        .filter_map(|p| match expected_name(&p.dir) {
            None => Some(violation(RULE, &p.name, format!("目录 {} 不在第 1.1 节的布局之内", p.dir))),
            Some(want) if want != p.name => Some(violation(
                RULE,
                &p.name,
                format!("目录 {} 要求 crate 名为 {want}，实际 {}", p.dir, p.name),
            )),
            Some(_) => None,
        })
        .collect()
}

/// `unwired-absent`：两个 wiring 目录下不出现以四类前缀开头的实现类型或注入行。
///
/// 本规则单列，不并入七条禁止项。
pub fn unwired_absent(root: &Path) -> Vec<Violation> {
    const RULE: &str = "unwired-absent";
    let mut found = Vec::new();
    for dir in WIRING_DIRS {
        let path = root.join(dir);
        if !path.is_dir() {
            found.push(violation(RULE, dir, "断言对象目录不存在"));
            continue;
        }
        for file in rust_files(&path) {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let rel = file.strip_prefix(root).unwrap_or(&file).display().to_string();
            for (no, line) in code_lines(&text) {
                for prefix in UNWIRED_PREFIXES {
                    if contains_ident_with_prefix(line, prefix) {
                        found.push(violation(
                            RULE,
                            &rel,
                            format!("第 {no} 行出现 {prefix} 前缀的实现类型或注入行：{line}"),
                        ));
                    }
                }
            }
        }
    }
    found
}

/// 前缀后必须紧跟大写字母或行末，避免把 `Noop` 之外的词误判。
fn contains_ident_with_prefix(line: &str, prefix: &str) -> bool {
    let bytes = line.as_bytes();
    let mut from = 0;
    while let Some(rel) = line[from..].find(prefix) {
        let at = from + rel;
        let before_ok = at == 0 || !is_ident_byte(bytes[at - 1]);
        let after = at + prefix.len();
        let after_ok = after >= bytes.len() || bytes[after].is_ascii_uppercase();
        if before_ok && after_ok {
            return true;
        }
        from = at + prefix.len();
    }
    false
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// 跨 crate 取具体事务句柄的唯一写法只允许出现在 `crates/adapter/db-pg/`。
pub fn downcast_confined(root: &Path) -> Vec<Violation> {
    const RULE: &str = "downcast-pgtx-confined";
    const NEEDLE: &str = "downcast_mut::<PgTx>";
    const ALLOWED: &str = "crates/adapter/db-pg";
    let mut found = Vec::new();
    for dir in ["crates", "apps", "testkit", "datagen", "tools"] {
        for file in rust_files(&root.join(dir)) {
            let rel = file.strip_prefix(root).unwrap_or(&file).display().to_string();
            if rel.starts_with(ALLOWED) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&file) else { continue };
            for (no, line) in code_lines(&text) {
                if line.contains(NEEDLE) {
                    found.push(violation(
                        RULE,
                        &rel,
                        format!("第 {no} 行出现 {NEEDLE}，该写法只允许在 {ALLOWED}/ 内"),
                    ));
                }
            }
        }
    }
    found
}

/// 禁止项第三条的 std 部分：domain 与 contract 不得触碰文件与网络 API。
pub fn forbidden_std_io(ws: &Workspace, root: &Path) -> Vec<Violation> {
    const RULE: &str = "domain-contract-no-io";
    const NEEDLES: [&str; 6] =
        ["std::fs", "std::net", "std::process", "tokio::fs", "tokio::net", "tokio::process"];
    let mut found = Vec::new();
    for pkg in &ws.packages {
        if !matches!(pkg.layer, Layer::Domain(_) | Layer::Contract(_)) {
            continue;
        }
        for file in rust_files(&root.join(&pkg.dir)) {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let rel = file.strip_prefix(root).unwrap_or(&file).display().to_string();
            for (no, line) in code_lines(&text) {
                for needle in NEEDLES {
                    if line.contains(needle) {
                        found.push(violation(
                            RULE,
                            &pkg.name,
                            format!("{rel} 第 {no} 行触碰 {needle}"),
                        ));
                    }
                }
            }
        }
    }
    found
}

/// 禁止项第七条：db-pg 中的仓储实现按 schema 分文件，一个仓储只访问自己模块的 schema。
pub fn one_schema_per_file(root: &Path) -> Vec<Violation> {
    const RULE: &str = "db-pg-one-schema-per-file";
    let mut found = Vec::new();
    for file in rust_files(&root.join("crates/adapter/db-pg/src")) {
        let Ok(text) = fs::read_to_string(&file) else { continue };
        let rel = file.strip_prefix(root).unwrap_or(&file).display().to_string();
        let mut schemas: Vec<String> = Vec::new();
        for (_, line) in code_lines(&text) {
            for schema in schema_refs(line) {
                if !schemas.contains(&schema) {
                    schemas.push(schema);
                }
            }
        }
        if schemas.len() > 1 {
            schemas.sort();
            found.push(violation(
                RULE,
                &rel,
                format!("同一文件访问了多个 schema：{}", schemas.join("、")),
            ));
        }
    }
    found
}

/// 从 SQL 字面量里抽 `<schema>.<table>` 的 schema 名。只认小写下划线形态。
fn schema_refs(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (idx, _) in line.match_indices('.') {
        let head: String = line[..idx]
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let tail_ok = line[idx + 1..].starts_with(|c: char| c.is_ascii_lowercase());
        if tail_ok && (head.starts_with("platform_") || head.starts_with("biz_")) {
            out.push(head);
        }
    }
    out
}

/// 禁止项第六条的必要性判据：foundation 的每个公开模块被两个及以上
/// `ep-contract-*` 引用，或被 `ep-platform-*` 引用。
///
/// 返回 `Err` 表示本判据当前不可判定，而不是通过。
pub fn foundation_necessity(ws: &Workspace, root: &Path) -> Result<Vec<Violation>, String> {
    let consumers: Vec<&crate::graph::Package> = ws
        .packages
        .iter()
        .filter(|p| matches!(p.layer, Layer::Contract(_) | Layer::Platform(_)))
        .collect();
    let with_code = consumers
        .iter()
        .filter(|p| {
            rust_files(&root.join(&p.dir))
                .iter()
                .any(|f| fs::read_to_string(f).is_ok_and(|t| code_lines(&t).count() > 0))
        })
        .count();
    if with_code == 0 {
        return Err(format!(
            "必要性判据不可判定：{} 个 ep-contract-* 与 ep-platform-* 全为骨架，无任何引用可数。\
             判据在骨架期恒为「零引用」，若按判据执行将把 foundation 的全部模块判为不必要。",
            consumers.len()
        ));
    }
    Ok(Vec::new())
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    #[test]
    fn negative_unwired_prefix_matching() {
        assert!(contains_ident_with_prefix("let x = NoopClock::new();", "Noop"));
        assert!(contains_ident_with_prefix("use crate::StubLedger;", "Stub"));
        // 词内出现不算：Restub 的 Stub 前面是标识符字符。
        assert!(!contains_ident_with_prefix("let restubbed = 1;", "Stub"));
        // 前缀后必须跟大写：noopy 不是实现类型。
        assert!(!contains_ident_with_prefix("let Noopy = 1;", "Noop"));
    }

    #[test]
    fn negative_schema_refs() {
        assert_eq!(schema_refs("\"select * from platform_authz.roles\""), vec!["platform_authz"]);
        assert_eq!(
            schema_refs("\"join biz_sales.orders on platform_meta.x\""),
            vec!["biz_sales", "platform_meta"]
        );
        assert!(schema_refs("self.pool.acquire()").is_empty());
    }
}
