use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let alerter_dir = PathBuf::from(&manifest_dir).join("alerter");
    let swift_build_dir = PathBuf::from(&out_dir).join("swift-build");

    println!("cargo:rerun-if-changed=alerter/Sources/");
    println!("cargo:rerun-if-changed=alerter/Package.swift");

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--scratch-path",
            swift_build_dir.to_str().unwrap(),
        ])
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

    let binary_path = swift_build_dir.join("release/alerter");
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
