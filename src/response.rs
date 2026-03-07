use crate::error::AlerterError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlerterResponse {
    pub activation_type: String,
    pub activation_value: Option<String>,
}

impl AlerterResponse {
    pub fn from_plain_text(text: &str) -> Self {
        Self {
            activation_type: text.trim().to_string(),
            activation_value: None,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, AlerterError> {
        serde_json::from_str(json)
            .map_err(|e| AlerterError::Runtime(format!("failed to parse JSON: {e}")))
    }
}
