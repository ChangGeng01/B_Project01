//! `xtask archcheck` — 结构门禁。
//!
//! 断言面分三类：依赖图可判定的落在 [`deps`]，需要读源码的落在 [`source`]，
//! ep-foundation 的冻结项与准入替身落在 [`frozen`] 与 [`foundation`]。
//!
//! 退出条件 3 的七条禁止项由 [`deps::FORBIDDEN_RULES`] 点名，
//! 退出条件 26 的 `unwired-absent` 单列，不并入那七条。
//!
//! 三态输出（裁定 F-03 第五段）：机检面全绿为 0，登记表比对不符或有违反为 1，
//! 仍存在真正不可判定项为 3。三条路径退出码互不相同，任何一条不能被读成另一条。

pub mod deps;
pub mod foundation;
pub mod frozen;
pub mod registry;
pub mod source;

use std::path::Path;

use crate::graph::{self, Workspace};
use deps::Violation;

/// 已裁定不由本工具执行的判据。永久登记，必须点名承接方。
///
/// 出处：裁定 F-03 第二段与技术基线第 12.1 节 delegated 段。
/// 本表与基线 12.1 节 delegated 段必须逐行相等。
pub const DELEGATED: [(&str, &str); 2] = [
    (
    "foundation-no-business/necessity",
    "不由本工具判定。承接方：评审举证，加 foundation-no-business/no-internal-dep、\
     foundation-frozen-items、foundation-marker-shape、foundation-module-registry、\
     foundation-no-single-owner 五条替身",
    ),
    (
        // 裁定 F-05：本条只在源码字面量里取 <schema>.<object>，读不出运行期连接取的角色。
        "db-pg-one-schema-per-file/analyst-ro-connection",
        "不由本工具判定。承接方：阶段 11 的 reporting-dataset-signature-matched 启动自检，加评审举证",
    ),
];

pub struct Report {
    pub violations: Vec<Violation>,
    /// 已判定的规则名。
    pub checked: Vec<&'static str>,
    /// 当前无法执行的判据，附不可判定的理由。临时登记，条数只减不增。
    pub undecidable: Vec<(&'static str, String)>,
}

/// 退出码。三者互不相同：判据写不出来、代码违规、判据被降为评审，
/// 是三件不同的事，不得互相冒充。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Clean,
    Undecidable,
    Violated,
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        if !self.violations.is_empty() {
            Outcome::Violated
        } else if !self.undecidable.is_empty() {
            Outcome::Undecidable
        } else {
            Outcome::Clean
        }
    }
}

pub fn run(root: &Path) -> Result<Report, String> {
    let ws = graph::load(root)?;
    Ok(evaluate(&ws))
}

pub fn evaluate(ws: &Workspace) -> Report {
    let root = ws.root.as_path();
    let mut violations = deps::check(ws);
    violations.extend(source::naming(ws));
    violations.extend(source::unwired_absent(root));
    violations.extend(source::downcast_confined(root));
    violations.extend(source::forbidden_std_io(ws, root));
    violations.extend(source::one_schema_per_file(root));
    violations.extend(frozen::check(root));
    violations.extend(foundation::module_registry(root));
    violations.extend(foundation::no_single_owner(root));

    let mut checked: Vec<&'static str> = deps::FORBIDDEN_RULES.to_vec();
    checked.extend([
        "platform-acyclic",
        "platform-no-adapter",
        "postgres-driver-tooling-only",
        "crate-naming-consistent",
        "unwired-absent",
        "downcast-pgtx-confined",
        "foundation-frozen-items",
        "foundation-module-registry",
        "foundation-no-single-owner",
        "foundation-marker-shape",
    ]);

    checked.push("undecidable-registry-matched");
    checked.push("rule-roster-matched");

    // 裁定 F-03 后本工具不再声称判定必要性，因此没有任何不可判定项。
    // 该表保留是为了让「未来新增的不可判定判据」有唯一且诚实的落点。
    let undecidable: Vec<(&'static str, String)> = Vec::new();
    violations.extend(registry::check(root, &DELEGATED, &undecidable));
    violations.extend(registry::rule_roster(root, &checked));

    Report {
        violations,
        checked,
        undecidable,
    }
}
