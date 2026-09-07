use std::process::{Command, Output};

use ep_platform_runtime::lifecycle::EXIT_CONFIG_OR_SELFCHECK;

const BIN: &str = env!("CARGO_BIN_EXE_job-worker");

fn run_default_check() -> Output {
    let isolated = std::env::temp_dir().join(format!(
        "ep-worker-default-kms-check-{}",
        std::process::id()
    ));
    let mut command = Command::new(BIN);
    command
        .args(["--check", "--config"])
        .arg(isolated.join("missing.toml"))
        .arg("--config-dir")
        .arg(isolated.join("missing.d"));
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("EP__") {
            command.env_remove(name);
        }
    }
    command.output().expect("能够启动 job-worker --check")
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn default_kms_provider_fails_closed_before_selfcheck_can_report_passed() {
    let output = run_default_check();
    let text = output_text(&output);
    assert_eq!(
        output.status.code(),
        Some(i32::from(EXIT_CONFIG_OR_SELFCHECK)),
        "{text}"
    );
    assert!(text.contains("NOT_IMPLEMENTED"), "{text}");
    assert!(text.contains("\"level\":\"ERROR\""), "{text}");
    assert!(!text.contains("建池失败"), "{text}");
    assert!(!text.contains("\"overall\": \"PASSED\""), "{text}");
}
