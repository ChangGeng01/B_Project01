//! 退出条件 21：ep-foundation 的跨阶段冻结项齐备且逐项与裁定一致。
//!
//! 判据是数量与落点，不是「我写对了」。任何阶段增删冻结项都会在这里变红。

use std::fs;
use std::path::Path;

use super::deps::Violation;

fn violation(package: &str, detail: impl Into<String>) -> Violation {
    Violation { rule: "foundation-frozen-items", package: package.to_string(), detail: detail.into() }
}

/// 冻结项的期望数量。取值出处为技术基线第 1.4 节。
const EXPECTED: [(&str, &str, usize); 7] = [
    ("id/marker.rs", "跨模块引用标记类型", 22),
    ("security/context.rs::SecurityContext", "安全上下文字段", 19),
    ("security/context.rs::ClientKind", "X-Client 取值", 6),
    ("security/context.rs::DutyClass", "职责类别", 6),
    ("module.rs::ModuleCode", "模块码", 15),
    ("capability.rs::CapabilityDomain", "能力域码", 18),
    ("capability.rs::ActionClass", "动作类别", 5),
];

pub fn check(root: &Path) -> Vec<Violation> {
    let base = root.join("crates/foundation/src");
    let mut found = Vec::new();

    let actual = [
        count_unit_structs(&base.join("id/marker.rs")),
        count_block_items(&base.join("security/context.rs"), "pub struct SecurityContext {"),
        count_block_items(&base.join("security/context.rs"), "pub enum ClientKind {"),
        count_block_items(&base.join("security/context.rs"), "pub enum DutyClass {"),
        count_block_items(&base.join("module.rs"), "pub enum ModuleCode {"),
        count_block_items(&base.join("capability.rs"), "pub enum CapabilityDomain {"),
        count_block_items(&base.join("capability.rs"), "pub enum ActionClass {"),
    ];

    for ((what, label, want), got) in EXPECTED.iter().zip(actual) {
        match got {
            None => found.push(violation(what, format!("{label}的落点读不到，无法计数"))),
            Some(n) if n != *want => {
                found.push(violation(what, format!("{label}应为 {want} 项，实际 {n} 项")))
            }
            Some(_) => {}
        }
    }

    // 两个端口模块必须存在，且阶段 1 内不得有任何条目。
    for (rel, owner) in [
        ("port/search.rs", "阶段 3b"),
        ("port/doc.rs", "阶段 5"),
        ("port/db.rs", "阶段 2 与阶段 11"),
    ] {
        let path = base.join(rel);
        match fs::read_to_string(&path) {
            Err(_) => found.push(violation(rel, "空端口模块文件不存在")),
            Ok(text) => {
                let items = text
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.starts_with("//") && !l.is_empty())
                    .count();
                if items != 0 {
                    found.push(violation(
                        rel,
                        format!("本文件应为空模块，内容由{owner}补齐，实际有 {items} 行非注释内容"),
                    ));
                }
            }
        }
    }

    found.extend(check_marker_shape(&base.join("id/marker.rs")));
    found.extend(check_marker_names(&base.join("id/marker.rs")));
    found
}

/// 按裁定 F-03 第三段，`id::marker` 为冻结清单制：22 项按名逐项断言。
///
/// 只数数量挡不住改名——把 SalesOrder 改成 Foo 仍是 22 个、静默通过。
/// 清单出处为技术基线第 1.4 节与裁定 A-01，两处逐字一致。
const MARKER_NAMES: [&str; 22] = [
    "LegalEntity", "UserAccount", "Session", "Department", "Position", "Project",
    "Customer", "Supplier", "Material", "Product", "Warehouse", "Contract",
    "ContractLine", "SalesOrder", "SalesOrderLine", "DeliveryConfirmation",
    "DeliveryConfirmationLine", "PurchaseOrder", "GoodsReceiptLine",
    "PurchaseInvoice", "PurchaseInvoiceLine", "AccountingPeriod",
];

fn check_marker_names(path: &Path) -> Vec<Violation> {
    let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
    let actual: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| is_unit_struct(l))
        .filter_map(|l| l.trim_start_matches("pub struct ").strip_suffix(';'))
        .collect();
    let mut found = Vec::new();
    for want in MARKER_NAMES {
        if !actual.contains(&want) {
            found.push(violation("id/marker.rs", format!("冻结清单缺少标记类型 {want}")));
        }
    }
    for got in &actual {
        if !MARKER_NAMES.contains(got) {
            found.push(violation(
                "id/marker.rs",
                format!("出现清单外的标记类型 {got}；本清单冻结 22 项，任何阶段不得增删"),
            ));
        }
    }
    found
}

/// 标记类型必须是无字段、无方法、无 trait 实现的单元结构体。
fn check_marker_shape(path: &Path) -> Vec<Violation> {
    let Ok(text) = fs::read_to_string(path) else { return Vec::new() };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && !l.is_empty())
        .filter(|l| !is_unit_struct(l))
        .map(|l| {
            violation(
                "id/marker.rs",
                format!("标记类型模块只允许单元结构体，出现了：{l}"),
            )
        })
        .collect()
}

fn is_unit_struct(line: &str) -> bool {
    line.starts_with("pub struct ") && line.ends_with(';') && !line.contains('(')
}

fn count_unit_structs(path: &Path) -> Option<usize> {
    let text = fs::read_to_string(path).ok()?;
    Some(text.lines().map(str::trim).filter(|l| is_unit_struct(l)).count())
}

/// 数 `header` 之后到配对右花括号之间的顶层条目数。
///
/// 枚举变体按逗号结尾计，结构体字段按逗号结尾计，两者形态一致。
fn count_block_items(path: &Path, header: &str) -> Option<usize> {
    let text = fs::read_to_string(path).ok()?;
    let start = text.find(header)? + header.len();
    let mut depth = 1usize;
    let mut end = start;
    for (i, c) in text[start..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }
    Some(
        text[start..end]
            .lines()
            .map(str::trim)
            .filter(|l| !l.starts_with("//") && !l.starts_with("#[") && !l.is_empty())
            .filter(|l| l.ends_with(','))
            .count(),
    )
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    #[test]
    fn negative_unit_struct_shape() {
        assert!(is_unit_struct("pub struct LegalEntity;"));
        // 带字段不是标记类型。
        assert!(!is_unit_struct("pub struct LegalEntity(pub u32);"));
        // 有 derive 就有 trait 实现，该行本身不是单元结构体行，会被单独报出。
        assert!(!is_unit_struct("#[derive(Clone)]"));
    }
}
