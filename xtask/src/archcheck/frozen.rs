//! 退出条件 21：ep-foundation 的跨阶段冻结项齐备且逐项与裁定一致。
//!
//! 判据是数量与落点，不是「我写对了」。任何阶段增删冻结项都会在这里变红。

use std::fs;
use std::path::Path;

use super::deps::Violation;

fn violation(package: &str, detail: impl Into<String>) -> Violation {
    Violation {
        rule: "foundation-frozen-items",
        package: package.to_string(),
        detail: detail.into(),
    }
}

/// 冻结项的期望数量。
///
/// **F-81 如实登记：这张表今天没有现行权威可依。** 原注称「取值出处为技术基线
/// 第 1.4 节」，而该文件（`00b-technical-baseline.md`）横幅逐字为
/// `SUPERSEDED_DO_NOT_EXECUTE`；同款要求所在的 `01-engineering-baseline.md` 是
/// `HISTORICAL_DO_NOT_EXECUTE`，`f51-development-readiness-freeze.md` 是
/// `PARTIALLY_SUPERSEDED`；而 F-57 现行文件里 `system_purpose`／`SystemPurpose`
/// 命中 0，也没有任何一处规定 `SecurityContext` 的字段数。
///
/// 也就是说：**下面每个数今天都只是「代码现状的转抄」，不是「权威的转抄」**。
/// 同目录的 `foundation-module-registry` 已经把这个反模式写在自己的注释里
/// （逐字「早先的实现只比对下面这个常量却声称权威在基线，等于两个都在工具里、
/// 文档改了工具不知道」），本项至今是那种写法。
///
/// 顺带证明这些数确实不能照旧基线取：旧基线同段把 `ClientKind` 冻结为八个变体
/// （含 `ServerAdmin`／`Mcp`），而代码与本表都是 6——若照旧基线改，会一次改错两处。
///
/// 转绿路径：G0 生成式再基线为这些闭集建立现行权威后，本表改为从该权威读取，
/// 而不是继续在工具里写死。在那之前本项只能证明「代码没有自我漂移」，
/// **不能证明「代码与权威一致」**——读它的结论时必须按这个射程读。
const EXPECTED: [(&str, &str, usize); 8] = [
    ("id/marker.rs", "跨模块引用标记类型", 22),
    (
        "security/context.rs::HumanContextInput",
        "human 入参字段",
        18,
    ),
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
        count_block_items(
            &base.join("security/context.rs"),
            "pub struct HumanContextInput {",
        ),
        count_block_items(
            &base.join("security/context.rs"),
            "pub struct SecurityContext {",
        ),
        count_block_items(&base.join("security/context.rs"), "pub enum ClientKind {"),
        count_block_items(&base.join("security/context.rs"), "pub enum DutyClass {"),
        count_block_items(&base.join("module.rs"), "pub enum ModuleCode {"),
        count_block_items(&base.join("capability.rs"), "pub enum CapabilityDomain {"),
        count_block_items(&base.join("capability.rs"), "pub enum ActionClass {"),
    ];

    for ((what, label, want), got) in EXPECTED.iter().zip(actual) {
        match got {
            None => found.push(violation(what, format!("{label}的落点读不到，无法计数"))),
            Some(n) if n != *want => found.push(violation(
                what,
                format!("{label}应为 {want} 项，实际 {n} 项"),
            )),
            Some(_) => {}
        }
    }

    // 尚未到归属阶段的端口模块必须保持空，内容由属主阶段补齐。
    for (rel, owner) in [("port/search.rs", "阶段 3b"), ("port/doc.rs", "阶段 5")] {
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

    // 阶段 2 已补齐的两个端口模块：判据从「必须为空」演进为「冻结项逐名在场」。
    // db.rs 只断言 C-07/B-03 四项，`ReadOnlyTx` 归阶段 11，不在本断言面；
    // kms.rs 断言 F-04 端口面九项 = KmsBackend trait 加八个词汇类型。
    const PORT_STAGE2: [(&str, &[&str]); 2] = [
        (
            "port/db.rs",
            &[
                "IdempotencyScope",
                "IdempotencyOutcome",
                "IdempotencyStore",
                "MigrationWindowGuard",
            ],
        ),
        (
            "port/kms.rs",
            &[
                "KmsBackend",
                "CipherText",
                "KeyDomainId",
                "BlindIndex",
                "Aad",
                "KeyRef",
                "Signature",
                "CipherEnvelope",
                "KeyPurpose",
            ],
        ),
    ];
    for (rel, items) in PORT_STAGE2 {
        let path = base.join(rel);
        let Ok(text) = fs::read_to_string(&path) else {
            found.push(violation(rel, "阶段 2 端口模块文件不存在"));
            continue;
        };
        for name in items {
            if !declares(&text, name) {
                found.push(violation(
                    rel,
                    format!("缺少阶段 2 冻结项 {name} 的声明；端口语汇不得增删改名"),
                ));
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
    "LegalEntity",
    "UserAccount",
    "Session",
    "Department",
    "Position",
    "Project",
    "Customer",
    "Supplier",
    "Material",
    "Product",
    "Warehouse",
    "Contract",
    "ContractLine",
    "SalesOrder",
    "SalesOrderLine",
    "DeliveryConfirmation",
    "DeliveryConfirmationLine",
    "PurchaseOrder",
    "GoodsReceiptLine",
    "PurchaseInvoice",
    "PurchaseInvoiceLine",
    "AccountingPeriod",
];

fn check_marker_names(path: &Path) -> Vec<Violation> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let actual: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| is_unit_struct(l))
        .filter_map(|l| l.trim_start_matches("pub struct ").strip_suffix(';'))
        .collect();
    let mut found = Vec::new();
    for want in MARKER_NAMES {
        if !actual.contains(&want) {
            found.push(violation(
                "id/marker.rs",
                format!("冻结清单缺少标记类型 {want}"),
            ));
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
    let Ok(text) = fs::read_to_string(path) else {
        // 读不到即未覆盖，不是通过。这是全仓唯一一处曾静默返回「零违反」的规则路径。
        return vec![Violation {
            rule: "foundation-marker-shape",
            package: "id/marker.rs".to_string(),
            detail: "标记类型模块读不到，形态无法判定".to_string(),
        }];
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && !l.is_empty())
        .filter(|l| !is_unit_struct(l))
        .map(|l| Violation {
            rule: "foundation-marker-shape",
            package: "id/marker.rs".to_string(),
            detail: format!("标记类型模块只允许单元结构体，出现了：{l}"),
        })
        .collect()
}

fn is_unit_struct(line: &str) -> bool {
    line.starts_with("pub struct ") && line.ends_with(';') && !line.contains('(')
}

/// 判定文本内是否有一条以 `name` 命名的 pub 声明（struct/enum/trait/type）。
/// 名字后必须不是标识符续字符，否则 `Blind` 会误命中 `BlindIndex`。
fn declares(text: &str, name: &str) -> bool {
    text.lines().map(str::trim).any(|l| {
        ["pub struct ", "pub enum ", "pub trait ", "pub type "]
            .iter()
            .any(|h| {
                l.strip_prefix(&format!("{h}{name}")).is_some_and(|rest| {
                    rest.chars()
                        .next()
                        .is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_')
                })
            })
    })
}

fn count_unit_structs(path: &Path) -> Option<usize> {
    let text = fs::read_to_string(path).ok()?;
    Some(
        text.lines()
            .map(str::trim)
            .filter(|l| is_unit_struct(l))
            .count(),
    )
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

#[cfg(test)]
mod rule_negative_samples {
    use std::path::PathBuf;

    use super::*;

    fn marker_fixture(tag: &str, body: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ep-frozen-{tag}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("建夹具目录");
        let p = root.join("marker.rs");
        fs::write(&p, body).expect("写夹具文件");
        p
    }

    fn all_22() -> String {
        MARKER_NAMES
            .iter()
            .map(|n| format!("pub struct {n};\n"))
            .collect()
    }

    /// 负样例：改名。数量仍是 22，只数数量的实现会静默通过。
    #[test]
    fn negative_marker_rename_is_caught() {
        let body = all_22().replace("pub struct SalesOrder;", "pub struct Foo;");
        let v = check_marker_names(&marker_fixture("rename", &body));
        assert_eq!(v.len(), 2, "缺一个与多一个各报一条");
        assert!(v
            .iter()
            .any(|x| x.detail.contains("缺少标记类型 SalesOrder")));
        assert!(v.iter().any(|x| x.detail.contains("清单外的标记类型 Foo")));
        assert!(check_marker_names(&marker_fixture("ok", &all_22())).is_empty());
    }

    /// 负样例：形态违反——带字段的元组结构体与带 derive 的都不是标记类型。
    #[test]
    fn negative_marker_shape_is_caught() {
        let with_field = all_22() + "pub struct Extra(pub u32);\n";
        let v = check_marker_shape(&marker_fixture("shape", &with_field));
        assert!(v.iter().any(|x| x.rule == "foundation-marker-shape"));

        let with_derive = format!("#[derive(Clone)]\n{}", all_22());
        assert!(!check_marker_shape(&marker_fixture("derive", &with_derive)).is_empty());
    }

    /// 读不到文件必须报，不得判通过。
    #[test]
    fn negative_marker_unreadable_is_not_a_pass() {
        let missing = std::env::temp_dir().join("ep-frozen-nope/marker.rs");
        let v = check_marker_shape(&missing);
        assert_eq!(v.len(), 1);
        assert!(v[0].detail.contains("读不到"));
    }

    #[test]
    fn negative_declares() {
        let src = "pub struct BlindIndex([u8; 16]);\npub trait KmsBackend: Send {\n";
        assert!(declares(src, "BlindIndex"));
        assert!(declares(src, "KmsBackend"));
        // 同名前缀不得误判，改名即缺项。
        assert!(!declares(src, "Blind"));
        assert!(!declares(src, "CipherText"));
    }
}
