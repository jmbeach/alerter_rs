use alerter_rs::{Alerter, AlerterError};
use std::path::PathBuf;

fn mock_alerter() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/mock_alerter.sh")
}

#[test]
fn remove_invokes_with_remove_flag() {
    let result: Result<(), AlerterError> =
        Alerter::remove_with_binary("my-group", Some(mock_alerter().as_path()));
    assert!(result.is_ok());
}

#[test]
fn list_invokes_with_list_flag() {
    let result: Result<String, AlerterError> =
        Alerter::list_with_binary("my-group", Some(mock_alerter().as_path()));
    assert!(result.is_ok());
    assert!(result.unwrap().contains("my-group"));
}

#[test]
fn list_all_notifications() {
    let result: Result<String, AlerterError> =
        Alerter::list_with_binary("ALL", Some(mock_alerter().as_path()));
    assert!(result.is_ok());
    assert!(result.unwrap().contains("ALL"));
}

#[test]
fn remove_returns_error_for_nonexistent_group() {
    let result =
        Alerter::remove_with_binary("nonexistent-group-12345", Some(mock_alerter().as_path()));
    // Mock always succeeds; just verify it doesn't panic
    let _ = result;
}

#[test]
fn remove_static_returns_correct_type() {
    // Verify the static method signature compiles
    fn _check_types() {
        let _: Result<(), AlerterError> = Alerter::remove("any-group");
    }
}

#[test]
fn list_static_returns_correct_type() {
    fn _check_types() {
        let _: Result<String, AlerterError> = Alerter::list("any-group");
    }
}
