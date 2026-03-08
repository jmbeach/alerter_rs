use std::fs;
use std::path::Path;

#[test]
fn test_showcase_file_exists() {
    let path = Path::new("examples/showcase.rs");
    assert!(path.exists(), "examples/showcase.rs should exist");
}

#[test]
fn test_showcase_imports_alerter() {
    let contents =
        fs::read_to_string("examples/showcase.rs").expect("failed to read examples/showcase.rs");

    assert!(
        contents.contains("alerter_rs::Alerter"),
        "showcase.rs should import alerter_rs::Alerter"
    );
}

#[test]
fn test_showcase_no_private_imports() {
    let contents =
        fs::read_to_string("examples/showcase.rs").expect("failed to read examples/showcase.rs");

    let private_modules = [
        "alerter_rs::binary",
        "alerter_rs::error",
        "alerter_rs::command",
    ];
    for module in &private_modules {
        assert!(
            !contents.contains(module),
            "showcase.rs should not import private module '{}'",
            module
        );
    }
}
