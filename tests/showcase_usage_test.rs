#![cfg(feature = "integration-tests")]

use std::process::Command;

#[test]
fn test_nonexistent_arg_exits_nonzero() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase", "--", "nonexistent"])
        .output()
        .expect("failed to run showcase");

    assert!(
        !output.status.success(),
        "showcase should exit with non-zero for unknown argument"
    );
}

#[test]
fn test_nonexistent_arg_prints_usage() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase", "--", "nonexistent"])
        .output()
        .expect("failed to run showcase");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("basic") && combined.contains("sound") && combined.contains("actions"),
        "usage output should list available notification types, got:\n{}",
        combined
    );
}

#[test]
fn test_no_args_prints_usage() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase"])
        .output()
        .expect("failed to run showcase");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        combined.contains("basic") && combined.contains("sound") && combined.contains("actions"),
        "usage output should list available notification types, got:\n{}",
        combined
    );
}
