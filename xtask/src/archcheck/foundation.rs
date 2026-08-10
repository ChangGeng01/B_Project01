//! 禁止项第六条的机检面。
//!
//! 按裁定 F-03，必要性一条降为评审判据，不由本工具判定；机检面不留空洞，
//! 由五条替身接盘。本模块实现其中两条新增替身，另两条见 [`super::deps`] 的
//! `foundation-no-business` 与 [`super::frozen`] 的 `foundation-frozen-items`。

use std::fs;
use std::path::{Path, PathBuf};

use super::deps::Violation;

/// ep-foundation 的顶层模块登记表，出处为技术基线第 1.4 节。
///
/// 新增顶层模块必须先改本表，这正是本规则要制造的摩擦：准入判据既然不由
/// 工具判，就由「改工具才能加模块」把动作暴露到评审面上。
const REGISTERED_MODULES: [&str; 7] =
    ["capability", "error", "id", "module", "port", "principal", "security"];

/// 唯一例外面。标记类型按第 1.3 节准入，其名字必然带模块码词元。
const SINGLE_OWNER_EXEMPT: &str = "crates/foundation/src/id/marker.rs";

fn violation(rule: &'static str, package: &str, detail: impl Into<String>) -> Violation {
    Violation { rule, package: package.to_string(), detail: detail.into() }
}

/// 替身三：顶层模块清单与登记表逐行相等，多一个少一个都判违反。
pub fn module_registry(root: &Path) -> Vec<Violation> {
    const RULE: &str = "foundation-module-registry";
    let path = root.join("crates/foundation/src/lib.rs");
    let Ok(text) = fs::read_to_string(&path) else {
        return vec![violation(RULE, "crates/foundation/src/lib.rs", "读不到 lib.rs")];
    };
    let mut actual: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter_map(|l| l.strip_prefix("pub mod ").and_then(|r| r.strip_suffix(';')))
        .map(str::to_string)
        .collect();
    actual.sort();
    let mut want: Vec<String> = REGISTERED_MODULES.iter().map(|s| s.to_string()).collect();
    want.sort();
    if actual == want {
        return Vec::new();
    }
    let extra: Vec<&String> = actual.iter().filter(|m| !want.contains(m)).collect();
    let missing: Vec<&String> = want.iter().filter(|m| !actual.contains(m)).collect();
    let mut detail = String::from("顶层模块与基线第 1.4 节登记表不符：");
    if !extra.is_empty() {
        detail.push_str(&format!("多出 {:?}；", extra));
    }
    if !missing.is_empty() {
        detail.push_str(&format!("缺少 {:?}；", missing));
    }
    detail.push_str("新增顶层模块须先改基线第 1.4 节与本规则的登记表。");
    vec![violation(RULE, "crates/foundation/src/lib.rs", detail)]
}

/// 替身四：foundation 内的声明不得带任何模块码词元。
///
/// 这是必要条件而非充要条件：不带模块码词元的业务形状（例如 `OrderDto`）
/// 仍能通过，该缺口由评审判据承接。
///
/// 扫描面刻意不含 `pub const`、`pub static`、`pub fn`：错误码常量的段名本来
/// 就是模块码，纳入即大批误报。
pub fn no_single_owner(root: &Path) -> Vec<Violation> {
    const RULE: &str = "foundation-no-single-owner";
    let base = root.join("crates/foundation/src");
    let Some(codes) = module_code_tokens(&base.join("module.rs")) else {
        return vec![violation(RULE, "module.rs", "读不到 ModuleCode 枚举体，词元表无法建立")];
    };
    let exempt = root.join(SINGLE_OWNER_EXEMPT);

    let mut found = Vec::new();
    for file in rust_files(&base) {
        if file == exempt {
            continue;
        }
        let rel = file.strip_prefix(root).unwrap_or(&file).display().to_string();
        let path_hit: Vec<&String> = file
            .strip_prefix(&base)
            .map(|p| p.components().filter_map(|c| c.as_os_str().to_str()).map(str::to_string).collect::<Vec<_>>())
            .unwrap_or_default()
            .iter()
            .map(|s| s.trim_end_matches(".rs").to_string())
            .filter_map(|seg| codes.iter().find(|c| **c == seg))
            .collect();
        for hit in path_hit {
            found.push(violation(RULE, &rel, format!("路径含模块码词元「{hit}」")));
        }

        let Ok(text) = fs::read_to_string(&file) else { continue };
        for (no, name) in declarations(&text) {
            for token in camel_tokens(&name) {
                if codes.contains(&token) {
                    found.push(violation(
                        RULE,
                        &rel,
                        format!("第 {no} 行声明 {name} 含模块码词元「{token}」，业务形状应定义在拥有它的 ep-contract-*"),
                    ));
                }
            }
        }
    }
    found
}

/// 从 `module.rs` 的 `ModuleCode` 枚举体做文本解析取词元，不 use ep_foundation，
/// 避免给 xtask 加一条对 ep-foundation 的依赖边。
fn module_code_tokens(path: &Path) -> Option<Vec<String>> {
    let text = fs::read_to_string(path).ok()?;
    let start = text.find("pub enum ModuleCode {")? + "pub enum ModuleCode {".len();
    let end = start + text[start..].find('}')?;
    let tokens: Vec<String> = text[start..end]
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && l.ends_with(','))
        .map(|l| l.trim_end_matches(',').to_lowercase())
        .filter(|l| !l.is_empty())
        .collect();
    (!tokens.is_empty()).then_some(tokens)
}

fn declarations(text: &str) -> Vec<(usize, String)> {
    const HEADS: [&str; 5] = ["pub struct ", "pub enum ", "pub trait ", "pub type ", "pub mod "];
    text.lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .filter_map(|(no, l)| {
            HEADS.iter().find_map(|h| l.strip_prefix(h)).map(|rest| {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                (no, name)
            })
        })
        .filter(|(_, n)| !n.is_empty())
        .collect()
}

/// `SalesOrderLine` -> ["sales", "order", "line"]；`snake_case` 也按下划线切。
fn camel_tokens(name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in name.chars() {
        if c == '_' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else if c.is_ascii_uppercase() && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
            cur.push(c.to_ascii_lowercase());
        } else {
            cur.push(c.to_ascii_lowercase());
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
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

#[cfg(test)]
mod negative_samples {
    use super::*;

    #[test]
    fn negative_camel_tokens() {
        assert_eq!(camel_tokens("SalesOrderLine"), ["sales", "order", "line"]);
        assert_eq!(camel_tokens("ledger_entry"), ["ledger", "entry"]);
        assert_eq!(camel_tokens("Id"), ["id"]);
        // 不误切连续大写之外的形态。
        assert_eq!(camel_tokens("PgTx"), ["pg", "tx"]);
    }

    #[test]
    fn negative_declaration_extraction() {
        let src = "pub struct Foo;\n// pub struct Bar;\npub enum Baz {\npub trait Qux: Send {";
        let got: Vec<String> = declarations(src).into_iter().map(|(_, n)| n).collect();
        assert_eq!(got, ["Foo", "Baz", "Qux"], "注释行不算声明");
    }
}

#[cfg(test)]
mod rule_negative_samples {
    use super::*;
    use std::fs;

    /// 造一个最小的 foundation 目录树，供两条规则的规则级负样例使用。
    ///
    /// 两条规则都读文件系统，合成图喂不进去，只能造真目录。
    fn fixture(tag: &str, lib: &str, extra: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ep-archcheck-{tag}"));
        let src = root.join("crates/foundation/src");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(src.join("id")).expect("建夹具目录");
        fs::write(src.join("lib.rs"), lib).expect("写 lib.rs");
        fs::write(
            src.join("module.rs"),
            "pub enum ModuleCode {\n    Mdm,\n    Sales,\n    Ledger,\n    Invoice,\n}\n",
        )
        .expect("写 module.rs");
        fs::write(src.join("id/marker.rs"), "pub struct SalesOrder;\n").expect("写 marker.rs");
        for (rel, body) in extra {
            fs::write(src.join(rel), body).expect("写附加文件");
        }
        root
    }

    const SEVEN: &str = "pub mod capability;\npub mod error;\npub mod id;\npub mod module;\n\
                         pub mod port;\npub mod principal;\npub mod security;\n";

    /// 负样例：新开一个登记表之外的顶层模块。
    #[test]
    fn negative_module_registry() {
        let ok = fixture("reg-ok", SEVEN, &[]);
        assert!(module_registry(&ok).is_empty(), "七项登记模块应通过");

        let bad = fixture("reg-bad", &format!("{SEVEN}pub mod bogus;\n"), &[]);
        let v = module_registry(&bad);
        assert_eq!(v.len(), 1);
        assert!(v[0].detail.contains("bogus"), "须点名多出的模块");

        let missing = fixture("reg-missing", "pub mod capability;\n", &[]);
        assert!(
            module_registry(&missing)[0].detail.contains("缺少"),
            "少一个模块同样违反，登记表是逐行相等而不是子集"
        );
    }

    /// 负样例：在已有模块内塞带模块码词元的业务形状。
    #[test]
    fn negative_no_single_owner() {
        let ok = fixture("own-ok", SEVEN, &[("error.rs", "pub struct AppError;\n")]);
        assert!(no_single_owner(&ok).is_empty(), "不含模块码词元的声明应通过");

        let bad = fixture("own-bad", SEVEN, &[("error.rs", "pub struct SalesOrderDto;\n")]);
        let v = no_single_owner(&bad);
        assert!(v.iter().any(|x| x.detail.contains("sales")), "须点名命中的模块码词元");

        // marker.rs 是唯一例外面：SalesOrder 在那里合法。
        assert!(
            !no_single_owner(&ok).iter().any(|x| x.package.contains("marker.rs")),
            "标记类型模块不受本规则约束"
        );
    }
}
