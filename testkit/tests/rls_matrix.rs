//! 独立测试目标 `tests/rls_matrix`（C-05 第三段，04 计划 §8.3 与
//! 退出条件 9）。可单独执行并输出结构化报告：
//!
//! `cargo test -p ep-testkit --test rls_matrix`
//!
//! 本阶段无活库：32 组矩阵与 4 项入口借用一律 Skipped，目标是
//! 如实输出结构化报告并守住"未覆盖不等于通过"的纪律，而不是
//! 判绿。绿判定属发布门禁项 `RG-RLS-MATRIX-GREEN`（阶段 14 的
//! ep-release-gate 逐项判定）。

use ep_testkit::matrix_32::{self, Polarity, POLARITIES};
use ep_testkit::rls_matrix::RlsAssertion;

fn outcome_of(assertion: &RlsAssertion) -> &'static str {
    match assertion {
        RlsAssertion::Passed => "PASSED",
        RlsAssertion::Failed(_) => "FAILED",
        RlsAssertion::Skipped(_) => "SKIPPED",
    }
}

/// 结构化报告：逐组逐极性一行，入口借用一段，门禁判定收尾。
#[test]
fn structured_report_of_the_32_by_2_matrix() {
    let cases = matrix_32::cases();
    assert_eq!(cases.len(), 32);
    println!("{{\"target\":\"tests/rls_matrix\",\"cases\":[");
    for case in &cases {
        for polarity in POLARITIES {
            let assertion = matrix_32::run_case(case, polarity);
            let polarity_label = match polarity {
                Polarity::Positive => "positive",
                Polarity::Negative => "negative",
            };
            println!(
                "  {{\"case\":\"{}\",\"polarity\":\"{}\",\"outcome\":\"{}\"}},",
                case.label(),
                polarity_label,
                outcome_of(&assertion)
            );
        }
    }
    println!("],\"entry_borrows\":[");
    for borrow in &matrix_32::ENTRY_BORROWS {
        let assertion = matrix_32::run_entry_borrow(borrow);
        println!(
            "  {{\"entry\":\"{}\",\"assertion\":\"{}\",\"outcome\":\"{}\"}},",
            borrow.entry,
            borrow.assertion,
            outcome_of(&assertion)
        );
    }
    println!("]}}");
    let (detail, green) = matrix_32::gate_verdict();
    println!(
        "{{\"gate\":\"{}\",\"green\":{green}}}",
        matrix_32::RG_RLS_MATRIX_GREEN
    );
    let _ = detail;
}

/// 越权读取、越权写入、跨法人聚合泄漏三项计数在本阶段必须为零：
/// 探针未接通时没有任何判定执行，计数自然为零；接通后由矩阵
/// 逐组判定维持该不变量（04 计划退出条件 4）。
#[test]
fn privilege_breach_counters_are_zero() {
    let summary = matrix_32::run_all();
    assert_eq!(summary.failed, 0, "本阶段不得出现失败判定");
}

/// 无活库纪律：不得有任何一组以 Passed 顶位。
#[test]
fn no_case_passes_without_a_live_probe() {
    let summary = matrix_32::run_all();
    assert_eq!(summary.passed, 0, "探针未接通时不得判过");
    assert_eq!(summary.skipped, 32 * 2 + 4);
}
