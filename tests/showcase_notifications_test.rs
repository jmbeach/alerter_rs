#![cfg(feature = "integration-tests")]

use std::process::Command;

fn run_showcase(arg: &str) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "--example", "showcase", "--", arg])
        .output()
        .unwrap_or_else(|e| panic!("failed to run showcase with arg '{}': {}", arg, e))
}

#[test]
fn test_showcase_actions() {
    let output = run_showcase("actions");
    assert!(
        output.status.success(),
        "showcase actions failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_dropdown() {
    let output = run_showcase("dropdown");
    assert!(
        output.status.success(),
        "showcase dropdown failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_reply() {
    let output = run_showcase("reply");
    assert!(
        output.status.success(),
        "showcase reply failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_icon() {
    let output = run_showcase("icon");
    assert!(
        output.status.success(),
        "showcase icon failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_content_image() {
    let output = run_showcase("content-image");
    assert!(
        output.status.success(),
        "showcase content-image failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_group() {
    let output = run_showcase("group");
    assert!(
        output.status.success(),
        "showcase group failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_sender() {
    let output = run_showcase("sender");
    assert!(
        output.status.success(),
        "showcase sender failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_json() {
    let output = run_showcase("json");
    assert!(
        output.status.success(),
        "showcase json failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_close_label() {
    let output = run_showcase("close-label");
    assert!(
        output.status.success(),
        "showcase close-label failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_ignore_dnd() {
    let output = run_showcase("ignore-dnd");
    assert!(
        output.status.success(),
        "showcase ignore-dnd failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_remove() {
    let output = run_showcase("remove");
    assert!(
        output.status.success(),
        "showcase remove failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
