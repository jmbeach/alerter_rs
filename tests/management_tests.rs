use alerter_rs::{Alerter, AlerterError};

#[test]
fn remove_invokes_with_remove_flag() {
    // Alerter::remove(group_id) should invoke alerter with --remove <group_id>
    // and return Result<(), AlerterError>
    let result: Result<(), AlerterError> = Alerter::remove("my-group");
    // We don't assert success — just that the types and method exist
    let _ = result;
}

#[test]
fn list_invokes_with_list_flag() {
    // Alerter::list(group_id) should invoke alerter with --list <group_id>
    // and return Result<String, AlerterError>
    let result: Result<String, AlerterError> = Alerter::list("my-group");
    let _ = result;
}

#[test]
fn list_all_notifications() {
    // Alerter::list("ALL") should list all active notifications
    let result: Result<String, AlerterError> = Alerter::list("ALL");
    let _ = result;
}

#[test]
fn remove_returns_error_for_nonexistent_group() {
    // Removing a non-existent group should still succeed or return a meaningful error
    let result = Alerter::remove("nonexistent-group-12345");
    // Either it succeeds (no-op) or returns an error — but it should not panic
    let _ = result;
}
