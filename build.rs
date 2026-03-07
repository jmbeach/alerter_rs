use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let alerter_dir = PathBuf::from(&manifest_dir).join("alerter");

    println!("cargo:rerun-if-changed=alerter/Sources/");
    println!("cargo:rerun-if-changed=alerter/Package.swift");

    let output = Command::new("swift")
        .args(["build", "-c", "release"])
        .current_dir(&alerter_dir)
        .output()
        .expect("Failed to run `swift build`. Is the Swift toolchain installed?");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "swift build failed with status {}:\n{}",
            output.status, stderr
        );
    }

    let binary_path = alerter_dir.join(".build/release/alerter");
    if !binary_path.exists() {
        panic!(
            "Expected alerter binary not found at: {}",
            binary_path.display()
        );
    }

    println!(
        "cargo:rustc-env=ALERTER_BINARY_PATH={}",
        binary_path.display()
    );
}
