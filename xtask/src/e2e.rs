//! `xtask e2e` — 端到端目标。本阶段只交付 `--profile=t0` 一个 profile。
//!
//! 出处：阶段 1 计划第 2 节 D-09 行与第 10 节退出条件第 25 条，判据是
//! 「`--profile=t0` 作为独立目标可执行，本阶段用例集为空并返回 0」。
//!
//! 用例集为空而返回 0 是计划明文允许的，但空集必须被印出来。理由是：
//! 一个静默返回 0 的目标，和一个登记了用例却因接线漏掉而一条没跑的目标，
//! 在 CI 上看起来完全一样。本模块因此在 stdout 上明写「本阶段用例集为空」
//! 并同时印出登记条数，让「加了用例却没被跑到」这件事在日志里一眼可见。
//!
//! 退出码三态与 `archcheck` 同源，不另立一套：全绿 0、有失败 1、有未覆盖 3。
//! 「未覆盖」单列一个退出码，是因为端到端用例的被测对象是一整套跑起来的进程，
//! 起不来时若判为通过，这个目标就成了摆设。

use std::process::ExitCode;

/// 本阶段唯一交付的 profile。
pub const PROFILE_T0: &str = "t0";

/// 已交付的 profile 名单。新增 profile 必须同批加登记与用例集。
pub const PROFILES: [&str; 1] = [PROFILE_T0];

/// 一条端到端用例的结论。没有第四种形态，也就没有「跑了但不知道结果」。
///
/// `Failed` 与 `Skipped` 在非测试构建下暂无构造点，因为本阶段用例集为空。
/// 这里用 `expect` 而不是 `allow`：第一条真用例落地并构造出这两个变体时，
/// 期望即落空并因 `-D warnings` 报错，逼这两行属性同批删掉，
/// 不会像 `allow` 那样永久留在文件里。测试构建下两个变体由夹具用例构造，
/// 故属性只挂在 `not(test)` 上。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Passed,
    /// 判据不成立，携带失败原因。
    #[cfg_attr(not(test), expect(dead_code, reason = "本阶段用例集为空，暂无构造点"))]
    Failed(String),
    /// 被测对象不可达（如整栈没起来）。不等于通过。
    #[cfg_attr(not(test), expect(dead_code, reason = "本阶段用例集为空，暂无构造点"))]
    Skipped(&'static str),
}

/// 一条端到端用例。`id` 取阶段 1 计划第 9.3 节的编号，如 `E2E-01`。
pub struct Case {
    pub id: &'static str,
    pub title: &'static str,
    pub run: fn() -> Outcome,
}

/// `--profile=t0` 的用例集。
///
/// 本阶段为空：T0 贯通线的十二条 E2E 用例的被测对象是 `deploy/` 下的单机编排与
/// 八个进程的运行时，二者本阶段均未接线，此处不放任何恒真占位用例。
/// 计划第 10 节退出条件第 25 条明文允许本阶段空集返回 0。
pub const T0_CASES: &[Case] = &[];

/// 一次运行的结论汇总。
pub struct Report {
    pub profile: String,
    pub results: Vec<(&'static str, &'static str, Outcome)>,
}

impl Report {
    pub fn total(&self) -> usize {
        self.results.len()
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    pub fn failed(&self) -> Vec<&(&'static str, &'static str, Outcome)> {
        self.results.iter().filter(|(_, _, o)| matches!(o, Outcome::Failed(_))).collect()
    }

    pub fn skipped(&self) -> Vec<&(&'static str, &'static str, Outcome)> {
        self.results.iter().filter(|(_, _, o)| matches!(o, Outcome::Skipped(_))).collect()
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|(_, _, o)| *o == Outcome::Passed).count()
    }
}

/// 逐条跑用例。不短路：一条失败不影响后续用例，报告要一次给全。
pub fn run_cases(profile: &str, cases: &[Case]) -> Report {
    let results = cases.iter().map(|c| (c.id, c.title, (c.run)())).collect();
    Report { profile: profile.to_string(), results }
}

/// 报告到退出码的映射。与 `archcheck` 的三态一致：0 通过、1 有失败、3 有未覆盖。
///
/// 失败优先于未覆盖：两者同时存在时报 1，因为「有判据不成立」比「有判据没跑」更硬。
pub fn exit_code(report: &Report) -> u8 {
    if !report.failed().is_empty() {
        1
    } else if !report.skipped().is_empty() {
        3
    } else {
        0
    }
}

/// 参数错误的退出码。与 `main.rs` 的约定一致：2 表调用方参数错误。
const EXIT_USAGE: u8 = 2;

const USAGE: &str = "用法: cargo xtask e2e --profile=t0";

/// 子命令入口。`args` 是已剥去 `e2e` 本身的剩余参数。
pub fn run(args: &[String]) -> ExitCode {
    let profile = match parse_profile(args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("{USAGE}");
            return ExitCode::from(EXIT_USAGE);
        }
    };

    let cases = match profile {
        PROFILE_T0 => T0_CASES,
        // `parse_profile` 已把未登记的 profile 挡在外面，走到这里说明登记表与
        // 用例集选取两处不同步，必须报错而不是当成空集放行。
        other => {
            eprintln!("profile {other} 已登记但没有对应的用例集，登记表与选取逻辑不同步。");
            return ExitCode::from(EXIT_USAGE);
        }
    };

    let report = run_cases(profile, cases);
    print_report(&report);
    ExitCode::from(exit_code(&report))
}

fn parse_profile(args: &[String]) -> Result<&'static str, String> {
    let mut chosen: Option<&'static str> = None;
    for arg in args {
        let Some(value) = arg.strip_prefix("--profile=") else {
            return Err(format!("未知参数 {arg}"));
        };
        let Some(known) = PROFILES.iter().find(|p| **p == value) else {
            return Err(format!("未知 profile {value}；已交付：{}", PROFILES.join("、")));
        };
        chosen = Some(known);
    }
    chosen.ok_or_else(|| format!("缺必填选项 --profile；已交付：{}", PROFILES.join("、")))
}

fn print_report(report: &Report) {
    println!("e2e profile={} 登记用例 {} 条。", report.profile, report.total());

    if report.is_empty() {
        // 这三行是本模块存在的一半理由，见模块注释：空集必须看得见。
        println!("本阶段用例集为空：profile {} 下没有任何已登记用例，一条都没有执行。", report.profile);
        println!("这是阶段 1 计划第 10 节退出条件第 25 条明文允许的状态，不是执行失败。");
        println!("若你刚加了用例却仍看到这一行，说明它没有被登记进 T0_CASES。");
        return;
    }

    for (id, title, outcome) in &report.results {
        let status = match outcome {
            Outcome::Passed => "通过".to_string(),
            Outcome::Failed(why) => format!("失败 — {why}"),
            Outcome::Skipped(why) => format!("未覆盖 — {why}"),
        };
        println!("  {id:<8} {title:<24} {status}");
    }

    println!(
        "\n通过 {}，失败 {}，未覆盖 {}。",
        report.passed_count(),
        report.failed().len(),
        report.skipped().len()
    );
    if !report.skipped().is_empty() {
        eprintln!("存在未覆盖用例（退出码 3）。未覆盖不等于通过。");
    }
    if !report.failed().is_empty() {
        eprintln!("存在失败用例（退出码 1）。");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing() -> Outcome {
        Outcome::Passed
    }
    fn failing() -> Outcome {
        Outcome::Failed("判据不成立".to_string())
    }
    fn skipping() -> Outcome {
        Outcome::Skipped("整栈未起")
    }

    fn case(id: &'static str, run: fn() -> Outcome) -> Case {
        Case { id, title: "夹具用例", run }
    }

    /// 本阶段判据：空用例集返回 0。
    #[test]
    fn empty_case_set_exits_zero() {
        let report = run_cases(PROFILE_T0, T0_CASES);
        assert!(report.is_empty());
        assert_eq!(exit_code(&report), 0);
    }

    /// 负样例：有失败用例时必须返回 1，断言的是退出码映射规则本身。
    ///
    /// 这条拦的是「用例集从空变非空之后，目标仍然恒返回 0」这条退化路径。
    #[test]
    fn failing_case_exits_one() {
        let cases = [case("E2E-XX", failing)];
        let report = run_cases(PROFILE_T0, &cases);
        assert_eq!(exit_code(&report), 1);
        assert_eq!(report.failed().len(), 1);
        assert_eq!(report.passed_count(), 0);
    }

    /// 负样例：未覆盖用例必须返回 3，不得判通过。
    #[test]
    fn skipped_case_exits_three() {
        let cases = [case("E2E-XX", skipping)];
        let report = run_cases(PROFILE_T0, &cases);
        assert_eq!(exit_code(&report), 3);
        assert_ne!(exit_code(&report), 0, "未覆盖不等于通过");
    }

    /// 失败与未覆盖同时存在时报 1，不被 3 盖掉。
    #[test]
    fn failure_outranks_skip() {
        let cases = [case("E2E-01", skipping), case("E2E-02", failing)];
        assert_eq!(exit_code(&run_cases(PROFILE_T0, &cases)), 1);
    }

    /// 全绿返回 0，且不短路：三条都跑到。
    #[test]
    fn all_passing_exits_zero_and_runs_every_case() {
        let cases = [case("A", passing), case("B", passing), case("C", passing)];
        let report = run_cases(PROFILE_T0, &cases);
        assert_eq!(report.total(), 3);
        assert_eq!(report.passed_count(), 3);
        assert_eq!(exit_code(&report), 0);
    }

    #[test]
    fn profile_t0_parses() {
        assert_eq!(parse_profile(&["--profile=t0".to_string()]), Ok(PROFILE_T0));
    }

    /// 负样例：缺 --profile 必须报参数错误，不得默认跑 t0。
    #[test]
    fn missing_profile_is_rejected() {
        let e = parse_profile(&[]).unwrap_err();
        assert!(e.contains("--profile"), "{e}");
    }

    /// 负样例：未登记的 profile 必须报错，不得静默当成空集返回 0。
    #[test]
    fn unknown_profile_is_rejected() {
        for bad in ["t1", "T0", "full", ""] {
            let e = parse_profile(&[format!("--profile={bad}")]).unwrap_err();
            assert!(e.contains("未知 profile"), "{bad} 未被拦下：{e}");
        }
    }

    /// 负样例：多余参数不得被静默忽略。
    #[test]
    fn unknown_argument_is_rejected() {
        let e = parse_profile(&["--fast".to_string()]).unwrap_err();
        assert!(e.contains("未知参数"), "{e}");
    }

    /// 登记表里的每个 profile 都必须能选到用例集，否则 `run` 会走到不同步分支。
    #[test]
    fn every_registered_profile_has_a_case_set() {
        for p in PROFILES {
            assert!(matches!(p, PROFILE_T0), "profile {p} 没有对应的用例集");
        }
    }
}
