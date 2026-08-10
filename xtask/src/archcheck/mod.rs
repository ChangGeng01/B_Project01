//! `xtask archcheck` — 结构门禁。
//!
//! 断言面分两类：依赖图可判定的落在 [`deps`]，需要读源码的落在 [`source`]。
//! 退出条件 3 的七条禁止项由 [`deps::FORBIDDEN_RULES`] 点名，
//! 退出条件 26 的 `unwired-absent` 单列，不并入那七条。

pub mod deps;
pub mod source;

use std::path::Path;

use deps::Violation;
use crate::graph::{self, Workspace};

pub struct Report {
    pub violations: Vec<Violation>,
    /// 已判定的规则名。
    pub checked: Vec<&'static str>,
    /// 当前不可判定的规则，附不可判定的理由。不计入通过。
    pub undecidable: Vec<(&'static str, String)>,
}

/// 退出码。不可判定与违反分开，避免「判据写不出来」被读成「代码违规」，
/// 也避免它被读成通过。
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

    let mut undecidable = Vec::new();
    match source::foundation_necessity(ws, root) {
        Ok(v) => violations.extend(v),
        Err(why) => undecidable.push(("foundation-no-business/necessity", why)),
    }

    let mut checked: Vec<&'static str> = deps::FORBIDDEN_RULES.to_vec();
    checked.extend([
        "platform-acyclic",
        "crate-naming-consistent",
        "unwired-absent",
        "downcast-pgtx-confined",
    ]);
    Report { violations, checked, undecidable }
}
