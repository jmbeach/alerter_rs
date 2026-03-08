use std::process::Command;

#[test]
fn test_showcase_example_compiles() {
    let output = Command::new("cargo")
        .args(["build", "--example", "showcase"])
        .output()
        .expect("failed to run cargo build");

    assert!(
        output.status.success(),
        "cargo build --example showcase failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_basic_exits_successfully() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase", "--", "basic"])
        .output()
        .expect("failed to run showcase");

    assert!(
        output.status.success(),
        "showcase basic failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_sound_exits_successfully() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase", "--", "sound"])
        .output()
        .expect("failed to run showcase");

    assert!(
        output.status.success(),
        "showcase sound failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_showcase_subtitle_exits_successfully() {
    let output = Command::new("cargo")
        .args(["run", "--example", "showcase", "--", "subtitle"])
        .output()
        .expect("failed to run showcase");

    assert!(
        output.status.success(),
        "showcase subtitle failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
