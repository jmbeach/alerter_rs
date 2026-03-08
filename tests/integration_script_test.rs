use std::fs;
use std::os::unix::fs::PermissionsExt;

const SCRIPT_PATH: &str = "integration-tests.sh";

#[test]
fn test_integration_script_exists() {
    assert!(
        fs::metadata(SCRIPT_PATH).is_ok(),
        "integration-tests.sh should exist in the project root"
    );
}

#[test]
fn test_integration_script_has_bash_shebang() {
    let content =
        fs::read_to_string(SCRIPT_PATH).expect("integration-tests.sh should exist and be readable");
    assert!(
        content.starts_with("#!/usr/bin/env bash"),
        "integration-tests.sh should start with #!/usr/bin/env bash shebang"
    );
}

#[test]
fn test_integration_script_is_executable() {
    let metadata = fs::metadata(SCRIPT_PATH).expect("integration-tests.sh should exist");
    let mode = metadata.permissions().mode();
    assert!(
        mode & 0o111 != 0,
        "integration-tests.sh should have executable permissions set"
    );
}
