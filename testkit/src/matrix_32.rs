//! 法人越权 32 组矩阵（阶段 4 任务 #23，04 计划 §8.3）。
//!
//! 八类（读取、写入、更新、删除、聚合、排序、报表投影、错误信息
//! 泄漏）× 2 法人 × 2 密级 = 32 组，每组既有"应可见"正例也有
//! "应不可见"反例。本文件不实现 C-05 冻结的十个断言函数中的任何
//! 一个（阶段 1 八个在 [`super::rls_matrix`]，阶段 2 两个同文件）：
//! 每组的判定映射到既有断言函数并继承其纪律——无活库时全部
//! `Skipped`，一律不判过。
//!
//! 判定标准（04 计划 §8.3，探针接通后按序生效）：
//! - 聚合泄漏：跨法人 count、sum、分面计数在越权上下文下返回 0
//!   或不返回该分面，不得返回真实值；
//! - 排序泄漏：按无权字段排序的请求返回 VALIDATION，不返回按该
//!   字段排好序的结果；
//! - 错误信息泄漏：对不可见记录的读写删三类请求，"记录不存在"与
//!   "记录存在但无权"两种情形的响应体与响应时间不可区分，时间差
//!   P95 不超过 5 毫秒。
//!
//! 入口借用五项中，内部对账上下文一项整条归阶段 9a（本阶段不跑
//! 不登记）；其余四项编入 [`ENTRY_BORROWS`] 并随矩阵与发布门禁项
//! `RG-RLS-MATRIX-GREEN` 判定。

use ep_foundation::security::level::SecurityLevel;

use crate::rls_matrix::{self, RlsAssertion};

/// 矩阵八类，与 C-05 冻结的八个断言函数一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpKind {
    Read,
    Write,
    Update,
    Delete,
    Aggregate,
    Sort,
    ReportProjection,
    ErrorLeak,
}

impl OpKind {
    pub const ALL: [OpKind; 8] = [
        OpKind::Read,
        OpKind::Write,
        OpKind::Update,
        OpKind::Delete,
        OpKind::Aggregate,
        OpKind::Sort,
        OpKind::ReportProjection,
        OpKind::ErrorLeak,
    ];

    pub fn label(self) -> &'static str {
        match self {
            OpKind::Read => "读取",
            OpKind::Write => "写入",
            OpKind::Update => "更新",
            OpKind::Delete => "删除",
            OpKind::Aggregate => "聚合",
            OpKind::Sort => "排序",
            OpKind::ReportProjection => "报表投影",
            OpKind::ErrorLeak => "错误信息泄漏",
        }
    }

    /// 本类判定映射到的冻结断言函数。八类与八函数逐字对应，
    /// 探针接通前继承其 `Skipped` 纪律。
    pub fn run(self) -> RlsAssertion {
        match self {
            OpKind::Read => rls_matrix::assert_read(),
            OpKind::Write => rls_matrix::assert_write(),
            OpKind::Update => rls_matrix::assert_update(),
            OpKind::Delete => rls_matrix::assert_delete(),
            OpKind::Aggregate => rls_matrix::assert_aggregate(),
            OpKind::Sort => rls_matrix::assert_sort(),
            OpKind::ReportProjection => rls_matrix::assert_report_projection(),
            OpKind::ErrorLeak => rls_matrix::assert_error_leak(),
        }
    }
}

/// 矩阵的 2 法人维度：序位 A/B，具体法人标识由 datagen 夹具在
/// 探针接通后绑定，此处只落序位。
pub const LEGAL_ENTITY_LABELS: [&str; 2] = ["法人 A", "法人 B"];

/// 矩阵的 2 密级维度：内部（默认值 20）与机密（40），交叉覆盖
/// 默认密级与高密级两条判定路径（取值见 data-dictionary §3）。
pub const MATRIX_LEVELS: [SecurityLevel; 2] = [SecurityLevel::Internal, SecurityLevel::Secret];

/// 极性：每组既有"应可见"正例也有"应不可见"反例。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Polarity {
    Positive,
    Negative,
}

pub const POLARITIES: [Polarity; 2] = [Polarity::Positive, Polarity::Negative];

/// 一组矩阵用例：八类 × 法人序位 × 密级，共 32 组。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MatrixCase {
    pub op: OpKind,
    pub entity_index: usize,
    pub level: SecurityLevel,
}

impl MatrixCase {
    /// 组的稳定标识：`类-法人序位-密级码`，供报告与门禁引用。
    pub fn label(&self) -> String {
        format!(
            "{}-{}-{}",
            self.op.label(),
            LEGAL_ENTITY_LABELS[self.entity_index],
            self.level.code()
        )
    }
}

/// 32 组矩阵：八类 × 2 法人 × 2 密级，序位按类、法人、密级展开。
pub fn cases() -> Vec<MatrixCase> {
    let mut out = Vec::with_capacity(32);
    for op in OpKind::ALL {
        for entity_index in 0..LEGAL_ENTITY_LABELS.len() {
            for level in MATRIX_LEVELS {
                out.push(MatrixCase {
                    op,
                    entity_index,
                    level,
                });
            }
        }
    }
    out
}

/// 一组内两极性的判定：探针接通前映射到该类的冻结断言函数，
/// 正例反例共用同一纪律（无活库即 Skipped，不判过）。
pub fn run_case(case: &MatrixCase, polarity: Polarity) -> RlsAssertion {
    let _ = polarity;
    case.op.run()
}

/// 入口借用项（04 计划 §8.3 五项中的前四项）。内部对账上下文
/// 一项整条归阶段 9a，本阶段不编入不登记。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EntryBorrow {
    /// 被测入口角色名。
    pub entry: &'static str,
    /// 判定映射到的冻结断言函数名。
    pub assertion: &'static str,
}

pub const ENTRY_BORROWS: [EntryBorrow; 4] = [
    EntryBorrow {
        entry: "ep_archiver",
        assertion: "assert_replication_role_containment",
    },
    EntryBorrow {
        entry: "ep_backuper",
        assertion: "assert_replication_role_containment",
    },
    EntryBorrow {
        entry: "ep_analyst_ro",
        assertion: "assert_read",
    },
    EntryBorrow {
        entry: "ep_ops_ro",
        assertion: "assert_read",
    },
];

/// 入口借用的判定：按断言函数名映射到阶段 1/2 已冻结的函数。
pub fn run_entry_borrow(borrow: &EntryBorrow) -> RlsAssertion {
    match borrow.assertion {
        "assert_replication_role_containment" => rls_matrix::assert_replication_role_containment(),
        "assert_read" => rls_matrix::assert_read(),
        // 登记面只允许上列两个函数名；其他取值属登记错误。
        _ => RlsAssertion::Failed(format!(
            "入口借用登记了未冻结的断言函数：{}",
            borrow.assertion
        )),
    }
}

/// 矩阵汇总：32 组 × 2 极性 + 4 项入口借用。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MatrixSummary {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl MatrixSummary {
    fn fold(&mut self, assertion: &RlsAssertion) {
        match assertion {
            RlsAssertion::Passed => self.passed += 1,
            RlsAssertion::Failed(_) => self.failed += 1,
            RlsAssertion::Skipped(_) => self.skipped += 1,
        }
    }

    /// 全绿判据：零失败且零跳过。`RG-RLS-MATRIX-GREEN` 据此判定；
    /// 无活库时 skipped 非零，门禁如实不绿，不以 Skipped 顶过。
    pub fn is_green(self) -> bool {
        self.failed == 0 && self.skipped == 0
    }
}

/// 跑完整矩阵：32 组逐组跑两个极性，另跑 4 项入口借用。
pub fn run_all() -> MatrixSummary {
    let mut summary = MatrixSummary::default();
    for case in cases() {
        for polarity in POLARITIES {
            summary.fold(&run_case(&case, polarity));
        }
    }
    for borrow in &ENTRY_BORROWS {
        summary.fold(&run_entry_borrow(borrow));
    }
    summary
}

/// 发布门禁项 `RG-RLS-MATRIX-GREEN`（14 计划 §8.7 门禁表 L543，
/// 判据提供方阶段 4）。逐项判定属阶段 14 的 ep-release-gate，
/// 本阶段只落静态登记与判定输入；判据口径按 02 计划 L757 改判：
/// `platform_core.unpoliced_table_registry` 行数与 rls_matrix 中
/// 承接入口用例数相等且全绿，本文件交付的 32 组是其中一段。
pub const RG_RLS_MATRIX_GREEN: &str = "RG-RLS-MATRIX-GREEN";

/// 门禁判定：绿当且仅当矩阵汇总零失败零跳过。无活库时如实不绿。
pub fn gate_verdict() -> (String, bool) {
    let summary = run_all();
    let detail = format!(
        "{RG_RLS_MATRIX_GREEN}: passed={} failed={} skipped={} green={}",
        summary.passed,
        summary.failed,
        summary.skipped,
        summary.is_green()
    );
    (detail, summary.is_green())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_are_exactly_thirty_two() {
        let cases = cases();
        assert_eq!(cases.len(), 32, "八类 × 2 法人 × 2 密级 = 32 组");
        for case in &cases {
            assert!(case.entity_index < LEGAL_ENTITY_LABELS.len());
            assert!(MATRIX_LEVELS.contains(&case.level));
        }
    }

    #[test]
    fn every_combination_appears_exactly_once() {
        let mut seen = Vec::new();
        for case in cases() {
            let key = (case.op, case.entity_index, case.level);
            assert!(!seen.contains(&key), "组合重复：{}", case.label());
            seen.push(key);
        }
        assert_eq!(seen.len(), 32);
    }

    /// 探针未接通时 32 组一律 Skipped：未覆盖不等于通过。
    #[test]
    fn without_a_probe_no_case_passes() {
        for case in cases() {
            for polarity in POLARITIES {
                assert!(
                    matches!(run_case(&case, polarity), RlsAssertion::Skipped(_)),
                    "{} 在无活库时必须 Skipped",
                    case.label()
                );
            }
        }
    }

    /// 入口借用只登记四项：内部对账归阶段 9a，不在本阶段矩阵内。
    #[test]
    fn entry_borrows_cover_four_runnable_entries() {
        assert_eq!(ENTRY_BORROWS.len(), 4);
        let entries: Vec<&str> = ENTRY_BORROWS.iter().map(|b| b.entry).collect();
        assert_eq!(
            entries,
            ["ep_archiver", "ep_backuper", "ep_analyst_ro", "ep_ops_ro"]
        );
        for borrow in &ENTRY_BORROWS {
            assert!(matches!(run_entry_borrow(borrow), RlsAssertion::Skipped(_)));
        }
    }

    /// 汇总在无活库时不得为绿：skipped 非零即不绿。
    #[test]
    fn summary_is_not_green_without_a_probe() {
        let summary = run_all();
        assert_eq!(summary.passed, 0);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.skipped, 32 * 2 + 4);
        assert!(!summary.is_green());
    }

    /// 两个密级维度取自冻结枚举且跨默认与高密两级。
    #[test]
    fn matrix_levels_span_internal_and_secret() {
        assert_eq!(MATRIX_LEVELS[0].code(), 20);
        assert_eq!(MATRIX_LEVELS[1].code(), 40);
    }

    /// 门禁登记口径：无活库时如实不绿，不以 Skipped 顶过。
    #[test]
    fn gate_verdict_is_not_green_without_a_probe() {
        let (detail, green) = gate_verdict();
        assert!(detail.contains(RG_RLS_MATRIX_GREEN));
        assert!(!green);
    }
}
