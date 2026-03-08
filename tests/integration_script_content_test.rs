use std::fs;

const SCRIPT_PATH: &str = "integration-tests.sh";

fn read_script() -> String {
    fs::read_to_string(SCRIPT_PATH).expect("integration-tests.sh should exist and be readable")
}

#[test]
fn test_script_contains_cargo_build_example() {
    let content = read_script();
    assert!(
        content.contains("cargo build --example showcase"),
        "script should contain 'cargo build --example showcase'"
    );
}

#[test]
fn test_script_uses_background_execution() {
    let content = read_script();
    // The script should launch processes in the background with &
    let has_background = content.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.starts_with('#') && trimmed.ends_with('&')
    });
    assert!(
        has_background,
        "script should use '&' for background/concurrent execution"
    );
}

#[test]
fn test_script_has_pass_fail_summary() {
    let content = read_script();
    let lower = content.to_lowercase();
    assert!(
        lower.contains("pass") && lower.contains("fail"),
        "script should contain pass/fail summary reporting"
    );
}

#[test]
fn test_script_exits_with_appropriate_codes() {
    let content = read_script();
    assert!(
        content.contains("exit 0") && content.contains("exit 1"),
        "script should exit with code 0 for success and code 1 for failure"
    );
}
