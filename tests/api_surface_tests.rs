use alerter_rs::{Alerter, AlerterError, AlerterResponse};

#[test]
fn alerter_is_importable() {
    let _alerter = Alerter::new("test");
}

#[test]
fn alerter_response_is_importable() {
    // AlerterResponse should be a struct we can reference as a type
    fn _takes_response(_r: AlerterResponse) {}
}

#[test]
fn alerter_error_is_importable() {
    // AlerterError should be a type we can reference
    fn _takes_error(_e: AlerterError) {}
}

#[test]
fn alerter_send_returns_result() {
    // Verify the send method returns Result<AlerterResponse, AlerterError>
    fn _assert_return_type(a: Alerter) {
        let _result: Result<AlerterResponse, AlerterError> = a.send();
    }
}

#[test]
fn alerter_error_implements_std_error() {
    fn _assert_std_error<E: std::error::Error>() {}
    _assert_std_error::<AlerterError>();
}

#[test]
fn alerter_error_implements_display() {
    fn _assert_display<E: std::fmt::Display>() {}
    _assert_display::<AlerterError>();
}

#[test]
fn alerter_error_implements_debug() {
    fn _assert_debug<E: std::fmt::Debug>() {}
    _assert_debug::<AlerterError>();
}

#[test]
fn alerter_response_implements_debug() {
    fn _assert_debug<T: std::fmt::Debug>() {}
    _assert_debug::<AlerterResponse>();
}
