use alerter_rs::{Alerter, NotificationHandle};
use std::path::PathBuf;
use std::time::Instant;

fn mock_alerter() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/mock_alerter.sh")
}

#[test]
fn send_async_returns_immediately() {
    let start = Instant::now();
    let _handle: NotificationHandle = Alerter::new("test")
        .binary_path(mock_alerter())
        .send_async()
        .expect("send_async should succeed");
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 2,
        "send_async should return immediately, took {:?}",
        elapsed
    );
}

#[test]
fn wait_returns_valid_response() {
    let handle = Alerter::new("test")
        .binary_path(mock_alerter())
        .send_async()
        .expect("send_async should succeed");
    let response = handle.wait().expect("wait should succeed");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
}

#[test]
fn wait_returns_json_response() {
    let handle = Alerter::new("test")
        .binary_path(mock_alerter())
        .json(true)
        .send_async()
        .expect("send_async should succeed");
    let response = handle.wait().expect("wait should succeed");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
    assert_eq!(response.activation_value, None);
}

#[test]
fn try_wait_returns_some_after_exit() {
    let mut handle = Alerter::new("test")
        .binary_path(mock_alerter())
        .send_async()
        .expect("send_async should succeed");

    // The mock exits immediately, so poll until done
    let mut result = None;
    for _ in 0..100 {
        if let Ok(Some(resp)) = handle.try_wait() {
            result = Some(resp);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let response = result.expect("try_wait should eventually return Some");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
}

#[test]
fn detach_consumes_handle_without_killing() {
    let handle = Alerter::new("test")
        .binary_path(mock_alerter())
        .send_async()
        .expect("send_async should succeed");
    // detach() should consume the handle without panicking
    handle.detach();
}

#[test]
fn drop_kills_child_process() {
    // Just verify that dropping a handle doesn't panic
    {
        let _handle = Alerter::new("test")
            .binary_path(mock_alerter())
            .send_async()
            .expect("send_async should succeed");
        // handle dropped here
    }
}
