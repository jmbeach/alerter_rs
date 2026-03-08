#![cfg(feature = "integration-tests")]

use alerter_rs::Alerter;
use std::process::Command;

#[test]
fn test_interactive_alert_returns_clicked_action() {
    let response = Alerter::new("Click 'OK' to pass this test")
        .title("Interactive Test")
        .actions(vec!["OK", "Cancel"])
        .json(true)
        .send()
        .expect("failed to send notification");

    assert_eq!(
        response.activation_type, "actionClicked",
        "expected user to click a button, got: {}",
        response.activation_type
    );
    assert_eq!(
        response.activation_value.as_deref(),
        Some("OK"),
        "expected user to click 'OK', got: {:?}",
        response.activation_value
    );
}

#[test]
fn test_alert_click_triggers_system_dialog() {
    let response = Alerter::new("Click this notification!")
        .title("Click Me")
        .json(true)
        .send()
        .expect("failed to send notification");

    if response.activation_type == "contentsClicked" {
        let output = Command::new("osascript")
            .args([
                "-e",
                r#"display dialog "You clicked the notification!" with title "alerter_rs" buttons {"Nice!"} default button "Nice!""#,
            ])
            .output()
            .expect("failed to run osascript");

        assert!(
            output.status.success(),
            "system dialog failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        panic!(
            "expected contentsClicked, got: {} — click the notification body, not a button",
            response.activation_type
        );
    }
}
