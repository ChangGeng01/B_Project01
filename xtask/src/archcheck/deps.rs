//! 依赖方向的七条禁止项，逐条断言。
//!
//! 判定只读 [`Workspace`]，不碰文件系统，因此每条规则的负样例可以直接构造
//! 合成图。禁止项编号与技术基线第 1.3 节「禁止的依赖方向」列表顺序一致。

use std::collections::BTreeMap;

use crate::graph::{Layer, Package, Workspace};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Violation {
    pub rule: &'static str,
    pub package: String,
    pub detail: String,
}

impl Violation {
    fn new(rule: &'static str, package: &str, detail: impl Into<String>) -> Violation {
        Violation {
            rule,
            package: package.to_string(),
            detail: detail.into(),
        }
    }
}

/// 第 3 条点名的第三方 crate。`tokio` 在 domain 与 contract 一律禁止：
/// 基线原文限的是「tokio 的 IO 模块」，但依赖图这一层分不出模块，
/// 取严不取宽，模块级细分由 `forbidden_std_io` 的源码扫描补。
const FORBIDDEN_THIRD_PARTY: [&str; 3] = ["sqlx", "reqwest", "tokio"];

/// 七条禁止项的规则名，供退出条件 3 逐条点名与负样例登记。
pub const FORBIDDEN_RULES: [&str; 7] = [
    "domain-no-cross-module",
    "app-no-peer-app",
    "domain-contract-no-io",
    "platform-no-domain-or-app",
    "adapter-no-peer-adapter",
    "foundation-no-business",
    "db-pg-one-schema-per-file",
];

pub fn check(ws: &Workspace) -> Vec<Violation> {
    let members: Vec<&str> = ws.member_names();
    let mut found = Vec::new();
    for pkg in &ws.packages {
        found.extend(rule_domain_no_cross_module(pkg, &members));
        found.extend(rule_app_no_peer_app(pkg));
        found.extend(rule_domain_contract_no_io(pkg));
        found.extend(rule_platform_no_domain_or_app(pkg));
        found.extend(rule_adapter_no_peer_adapter(pkg));
        found.extend(rule_platform_no_adapter(pkg));
        found.extend(rule_foundation_no_business(pkg, &members));
        found.extend(rule_postgres_driver_tooling_only(pkg));
    }
    found.extend(rule_platform_acyclic(ws));
    found
}

/// 禁止一：ep-domain-A 依赖 ep-domain-B、ep-app-B 或 ep-contract-B。
///
/// 跨模块只走 ep-app-A 依赖 ep-contract-B。
fn rule_domain_no_cross_module(pkg: &Package, members: &[&str]) -> Vec<Violation> {
    const RULE: &str = "domain-no-cross-module";
    let Layer::Domain(own) = &pkg.layer else {
        return Vec::new();
    };
    pkg.deps
        .iter()
        .filter(|d| members.contains(&d.as_str()))
        .filter_map(|dep| {
            let layer = Layer::of(dep, false);
            let other = layer.module()?;
            let kind = match layer {
                Layer::Domain(_) => "领域",
                Layer::Application(_) => "应用",
                Layer::Contract(_) => "契约",
                _ => return None,
            };
            (other != own).then(|| {
                Violation::new(
                    RULE,
                    &pkg.name,
                    format!("依赖了 {other} 模块的{kind} crate {dep}；跨模块只走 ep-app-{own} 依赖 ep-contract-{other}"),
                )
            })
        })
        .collect()
}

/// 禁止二：ep-app-A 依赖 ep-app-B。
///
/// 模块间同步调用只能通过 ep-contract-B 中的 trait，实现在 apps 装配时注入。
fn rule_app_no_peer_app(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "app-no-peer-app";
    let Layer::Application(own) = &pkg.layer else {
        return Vec::new();
    };
    pkg.deps
        .iter()
        .filter_map(|dep| match Layer::of(dep, false) {
            Layer::Application(other) if &other != own => Some(Violation::new(
                RULE,
                &pkg.name,
                format!("依赖了同层 {dep}；模块间同步调用只能经 ep-contract-{other} 的 trait"),
            )),
            _ => None,
        })
        .collect()
}

/// 禁止三：ep-domain-* 与 ep-contract-* 依赖 adapter、sqlx、reqwest、tokio 的 IO 模块、
/// std 的文件与网络 API。本函数覆盖依赖图可见的部分，std 部分见 `source::forbidden_std_io`。
fn rule_domain_contract_no_io(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "domain-contract-no-io";
    if !matches!(pkg.layer, Layer::Domain(_) | Layer::Contract(_)) {
        return Vec::new();
    }
    pkg.deps
        .iter()
        .filter_map(|dep| {
            if matches!(Layer::of(dep, false), Layer::Adapter(_)) {
                return Some(Violation::new(
                    RULE,
                    &pkg.name,
                    format!("依赖了适配层 {dep}"),
                ));
            }
            FORBIDDEN_THIRD_PARTY
                .contains(&dep.as_str())
                .then(|| Violation::new(RULE, &pkg.name, format!("依赖了 IO 第三方 crate {dep}")))
        })
        .collect()
}

/// 禁止四：ep-platform-* 依赖任何 domain 或 application。
fn rule_platform_no_domain_or_app(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "platform-no-domain-or-app";
    if !matches!(pkg.layer, Layer::Platform(_)) {
        return Vec::new();
    }
    pkg.deps
        .iter()
        .filter_map(|dep| match Layer::of(dep, false) {
            Layer::Domain(_) => Some(("领域", dep)),
            Layer::Application(_) => Some(("应用", dep)),
            _ => None,
        })
        .map(|(kind, dep)| Violation::new(RULE, &pkg.name, format!("依赖了{kind}层 {dep}")))
        .collect()
}

/// 禁止五：adapter 之间互相依赖，共用逻辑下沉到 ep-foundation。
fn rule_adapter_no_peer_adapter(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "adapter-no-peer-adapter";
    let Layer::Adapter(own) = &pkg.layer else {
        return Vec::new();
    };
    pkg.deps
        .iter()
        .filter_map(|dep| match Layer::of(dep, false) {
            Layer::Adapter(other) if &other != own => Some(Violation::new(
                RULE,
                &pkg.name,
                format!("依赖了同层 {dep}；共用逻辑应下沉到 ep-foundation"),
            )),
            _ => None,
        })
        .collect()
}

/// 禁止六（依赖图可判定的一半）：ep-foundation 不依赖工作区内任何 crate。
///
/// 另一半「必要性：被两个及以上 ep-contract-* 引用，或被 ep-platform-* 引用」
/// 按裁定 F-03 降为评审判据，不由本工具判定，登记见 [`super::DELEGATED`]
/// 与技术基线第 12.1 节 delegated 段。
fn rule_foundation_no_business(pkg: &Package, members: &[&str]) -> Vec<Violation> {
    const RULE: &str = "foundation-no-business";
    if pkg.layer != Layer::Foundation {
        return Vec::new();
    }
    pkg.deps
        .iter()
        .filter(|d| members.contains(&d.as_str()))
        .map(|dep| {
            Violation::new(
                RULE,
                &pkg.name,
                format!("依赖了工作区内 crate {dep}；foundation 必须无内部依赖"),
            )
        })
        .collect()
}

/// 允许项：ep-platform-* 只可依赖 ep-foundation 与其他 ep-platform-*。
///
/// 本规则不进 [`FORBIDDEN_RULES`]，禁止项仍是七条。它补的是允许项在 adapter
/// 一侧从来没有承接方这个缺口：[`rule_platform_no_domain_or_app`] 只把 Domain 与
/// Application 记为违规，Adapter 落进通配分支无检——实测给
/// `crates/platform/release/Cargo.toml` 加一行 `ep-adapter-kms` 后十六条规则全绿。
fn rule_platform_no_adapter(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "platform-no-adapter";
    if !matches!(pkg.layer, Layer::Platform(_)) {
        return Vec::new();
    }
    pkg.deps
        .iter()
        .filter(|dep| matches!(Layer::of(dep, false), Layer::Adapter(_)))
        .map(|dep| {
            Violation::new(
                RULE,
                &pkg.name,
                format!("依赖了适配层 {dep}；platform 只可依赖 ep-foundation 与其他 ep-platform-*，端口应下沉 ep_foundation::port::*"),
            )
        })
        .collect()
}

/// 只许工具层引用的驱动 crate。`tokio-postgres` 是阶段 2 因 refinery 结构冲突
/// （见 tools/migrate/src/history.rs 模块头逐字记录）经 leader 批准仅给
/// tools/migrate 自建兼容 Runner 用的裸驱动；库访问的正道是 ep-adapter-db-pg，
/// crates/ 下任何包再引它就是把工具层的临时通道变成业务层的常驻依赖。
const TOOL_ONLY_DEPS: [&str; 1] = ["tokio-postgres"];

/// 允许项：裸 Postgres 驱动只许出现在工具层（tools/）。
///
/// 与 `platform-no-adapter` 同款，不进 [`FORBIDDEN_RULES`]，禁止项仍是七条。
fn rule_postgres_driver_tooling_only(pkg: &Package) -> Vec<Violation> {
    const RULE: &str = "postgres-driver-tooling-only";
    if matches!(pkg.layer, Layer::Tooling(_)) {
        return Vec::new();
    }
    pkg.deps
        .iter()
        .filter(|dep| TOOL_ONLY_DEPS.contains(&dep.as_str()))
        .map(|dep| {
            Violation::new(
                RULE,
                &pkg.name,
                format!(
                    "依赖了裸驱动 {dep}；该驱动仅许 tools/migrate 使用，库访问一律走 ep-adapter-db-pg"
                ),
            )
        })
        .collect()
}

/// 允许项：platform 内部不得成环。
///
/// 注意判定次序：包级环由 Cargo 先行拒绝（`cyclic package dependency`），
/// `cargo metadata` 随之失败，本规则实际到不了。它守的是 Cargo 放行而层位不允许
/// 的情形，并在 archcheck 的输出里为该允许项留一个具名落点。
fn rule_platform_acyclic(ws: &Workspace) -> Vec<Violation> {
    const RULE: &str = "platform-acyclic";
    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Open,
        Done,
    }

    fn visit(
        ws: &Workspace,
        name: &str,
        state: &mut BTreeMap<String, Mark>,
        found: &mut Vec<Violation>,
    ) {
        state.insert(name.to_string(), Mark::Open);
        if let Some(pkg) = ws.get(name) {
            for dep in &pkg.deps {
                if !matches!(Layer::of(dep, false), Layer::Platform(_)) {
                    continue;
                }
                match state.get(dep.as_str()) {
                    Some(Mark::Open) => found.push(Violation::new(
                        RULE,
                        name,
                        format!("platform 内部成环：{name} -> {dep}"),
                    )),
                    Some(Mark::Done) => {}
                    None => visit(ws, dep, state, found),
                }
            }
        }
        state.insert(name.to_string(), Mark::Done);
    }

    let mut state: BTreeMap<String, Mark> = BTreeMap::new();
    let mut found = Vec::new();
    for pkg in ws
        .packages
        .iter()
        .filter(|p| matches!(p.layer, Layer::Platform(_)))
    {
        if !state.contains_key(&pkg.name) {
            visit(ws, &pkg.name, &mut state, &mut found);
        }
    }
    found
}

#[cfg(test)]
mod negative_samples {
    use super::*;

    fn pkg(name: &str, deps: &[&str]) -> Package {
        Package {
            name: name.to_string(),
            dir: format!("crates/{name}"),
            layer: Layer::of(name, false),
            deps: deps.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn violated(rule: &str, packages: Vec<Package>) -> bool {
        let ws = Workspace::from_packages(packages);
        check(&ws).iter().any(|v| v.rule == rule)
    }

    /// 负样例一：领域层跨模块。
    #[test]
    fn negative_domain_no_cross_module() {
        assert!(violated(
            "domain-no-cross-module",
            vec![
                pkg("ep-domain-sales", &["ep-contract-mdm"]),
                pkg("ep-contract-mdm", &[])
            ],
        ));
        assert!(!violated(
            "domain-no-cross-module",
            vec![
                pkg("ep-domain-sales", &["ep-contract-sales"]),
                pkg("ep-contract-sales", &[])
            ],
        ));
    }

    /// 负样例二：应用层依赖同层。
    #[test]
    fn negative_app_no_peer_app() {
        assert!(violated(
            "app-no-peer-app",
            vec![
                pkg("ep-app-sales", &["ep-app-ledger"]),
                pkg("ep-app-ledger", &[])
            ],
        ));
    }

    /// 负样例三：契约层依赖 IO。
    #[test]
    fn negative_domain_contract_no_io() {
        assert!(violated(
            "domain-contract-no-io",
            vec![pkg("ep-contract-sales", &["sqlx"])]
        ));
        assert!(violated(
            "domain-contract-no-io",
            vec![pkg("ep-domain-sales", &["ep-adapter-db-pg"])],
        ));
    }

    /// 负样例四：平台层依赖领域。
    #[test]
    fn negative_platform_no_domain_or_app() {
        assert!(violated(
            "platform-no-domain-or-app",
            vec![pkg("ep-platform-authz", &["ep-domain-sales"])],
        ));
    }

    /// 负样例五：适配层互相依赖。
    #[test]
    fn negative_adapter_no_peer_adapter() {
        assert!(violated(
            "adapter-no-peer-adapter",
            vec![pkg("ep-adapter-search", &["ep-adapter-db-pg"])],
        ));
        assert!(!violated(
            "adapter-no-peer-adapter",
            vec![pkg("ep-adapter-db-pg", &["ep-foundation"])]
        ));
    }

    /// 负样例六：foundation 反向依赖工作区内 crate。
    #[test]
    fn negative_foundation_no_business() {
        assert!(violated(
            "foundation-no-business",
            vec![
                pkg("ep-foundation", &["ep-contract-sales"]),
                pkg("ep-contract-sales", &[])
            ],
        ));
        assert!(!violated(
            "foundation-no-business",
            vec![pkg("ep-foundation", &["uuid"])]
        ));
    }

    /// 附加负样例：platform 依赖 adapter。
    #[test]
    fn negative_platform_no_adapter() {
        assert!(violated(
            "platform-no-adapter",
            vec![pkg("ep-platform-release", &["ep-adapter-kms"])],
        ));
        assert!(!violated(
            "platform-no-adapter",
            vec![pkg(
                "ep-platform-release",
                &["ep-foundation", "ep-platform-audit"]
            )],
        ));
    }

    /// 附加负样例：platform 内部成环。
    #[test]
    fn negative_platform_acyclic() {
        assert!(violated(
            "platform-acyclic",
            vec![
                pkg("ep-platform-authz", &["ep-platform-tenancy"]),
                pkg("ep-platform-tenancy", &["ep-platform-authz"]),
            ],
        ));
        assert!(!violated(
            "platform-acyclic",
            vec![
                pkg("ep-platform-authz", &["ep-platform-tenancy"]),
                pkg("ep-platform-tenancy", &["ep-foundation"]),
            ],
        ));
    }

    /// 附加负样例：裸 Postgres 驱动渗入非工具层即违规；工具层自己用则放行。
    #[test]
    fn negative_postgres_driver_tooling_only() {
        assert!(violated(
            "postgres-driver-tooling-only",
            vec![pkg("ep-adapter-db-pg", &["tokio-postgres"])],
        ));
        assert!(violated(
            "postgres-driver-tooling-only",
            vec![pkg("ep-foundation", &["tokio-postgres"])],
        ));
        assert!(!violated(
            "postgres-driver-tooling-only",
            vec![pkg("ep-migrate", &["tokio-postgres"])],
        ));
    }
}
