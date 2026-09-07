use std::process::Command;

fn run_configdoc(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ep-xtask"))
        .arg("configdoc")
        .args(args)
        .output()
        .expect("ep-xtask binary should start")
}

#[test]
fn configdoc_rejects_unknown_argument_instead_of_running_default_gate() {
    let output = run_configdoc(&["--not-a-configdoc-option"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("未知参数 --not-a-configdoc-option"),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn configdoc_rejects_duplicate_type_code_flag() {
    let output = run_configdoc(&["--check-doc-type-codes", "--check-doc-type-codes"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("只能出现一次"),
        "stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
