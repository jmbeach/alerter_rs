use std::fmt;

#[derive(Debug)]
pub enum AlerterError {
    BinaryExtraction(String),
    ProcessSpawn(String),
    Runtime(String),
}

impl fmt::Display for AlerterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlerterError::BinaryExtraction(msg) => {
                write!(f, "failed to extract alerter binary: {msg}")
            }
            AlerterError::ProcessSpawn(msg) => {
                write!(f, "failed to spawn alerter process: {msg}")
            }
            AlerterError::Runtime(msg) => write!(f, "alerter runtime error: {msg}"),
        }
    }
}

impl std::error::Error for AlerterError {}
