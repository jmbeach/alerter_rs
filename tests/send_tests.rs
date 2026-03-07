use alerter_rs::{Alerter, AlerterError, AlerterResponse};
use std::path::PathBuf;

fn mock_alerter() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/mock_alerter.sh")
}

#[test]
fn send_returns_result() {
    let result: Result<AlerterResponse, AlerterError> = Alerter::new("test message")
        .binary_path(mock_alerter())
        .send();
    assert!(result.is_ok());
}

#[test]
fn send_with_mock_returns_plain_text_response() {
    let response = Alerter::new("test message")
        .binary_path(mock_alerter())
        .send()
        .expect("send should succeed with mock");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
    assert_eq!(response.activation_value, None);
}

#[test]
fn send_with_mock_json_returns_json_response() {
    let response = Alerter::new("test message")
        .binary_path(mock_alerter())
        .json(true)
        .send()
        .expect("send should succeed with mock in json mode");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
    assert_eq!(response.activation_value, None);
}

#[test]
fn send_parses_plain_text_activation_type() {
    let response = AlerterResponse::from_plain_text("@CONTENTCLICKED");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
    assert_eq!(response.activation_value, None);
}

#[test]
fn send_parses_plain_text_action_clicked() {
    let response = AlerterResponse::from_plain_text("OK");
    assert_eq!(response.activation_type, "OK");
}

#[test]
fn send_parses_plain_text_timeout() {
    let response = AlerterResponse::from_plain_text("@TIMEOUT");
    assert_eq!(response.activation_type, "@TIMEOUT");
}

#[test]
fn send_parses_plain_text_closed() {
    let response = AlerterResponse::from_plain_text("@CLOSED");
    assert_eq!(response.activation_type, "@CLOSED");
}

#[test]
fn send_parses_json_output() {
    let json_str = r#"{"activationType": "@CONTENTCLICKED", "activationValue": null}"#;
    let response = AlerterResponse::from_json(json_str).expect("should parse valid JSON");
    assert_eq!(response.activation_type, "@CONTENTCLICKED");
    assert_eq!(response.activation_value, None);
}

#[test]
fn send_parses_json_with_activation_value() {
    let json_str = r#"{"activationType": "@REPLIED", "activationValue": "hello back"}"#;
    let response = AlerterResponse::from_json(json_str).expect("should parse valid JSON");
    assert_eq!(response.activation_type, "@REPLIED");
    assert_eq!(response.activation_value, Some("hello back".to_string()));
}

#[test]
fn send_parses_json_with_action_value() {
    let json_str = r#"{"activationType": "Yes", "activationValue": null}"#;
    let response = AlerterResponse::from_json(json_str).expect("should parse valid JSON");
    assert_eq!(response.activation_type, "Yes");
    assert_eq!(response.activation_value, None);
}

#[test]
fn send_parses_json_returns_error_for_invalid_json() {
    let result = AlerterResponse::from_json("not valid json {{{");
    assert!(result.is_err());
}

#[test]
fn alerter_error_binary_extraction_variant() {
    let err = AlerterError::BinaryExtraction("failed to write binary".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("failed to write binary"));
}

#[test]
fn alerter_error_process_spawn_variant() {
    let err = AlerterError::ProcessSpawn("could not execute".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("could not execute"));
}

#[test]
fn alerter_error_runtime_variant() {
    let err = AlerterError::Runtime("alerter returned error".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("alerter returned error"));
}

#[test]
fn alerter_error_implements_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(AlerterError::Runtime("test".to_string()));
    let _ = err;
}
