use std::io::Read;
use std::process::Child;

use crate::error::AlerterError;
use crate::response::AlerterResponse;

pub struct NotificationHandle {
    child: Option<Child>,
    json: bool,
}

impl NotificationHandle {
    pub(crate) fn new(child: Child, json: bool) -> Self {
        Self {
            child: Some(child),
            json,
        }
    }

    pub fn wait(mut self) -> Result<AlerterResponse, AlerterError> {
        let mut child = self
            .child
            .take()
            .ok_or_else(|| AlerterError::Runtime("handle already consumed".to_string()))?;

        let status = child
            .wait()
            .map_err(|e| AlerterError::Runtime(format!("failed to wait on child: {e}")))?;

        if !status.success() {
            let mut stderr_str = String::new();
            if let Some(mut stderr) = child.stderr.take() {
                let _ = stderr.read_to_string(&mut stderr_str);
            }
            return Err(AlerterError::Runtime(stderr_str.trim().to_string()));
        }

        let mut stdout_str = String::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout
                .read_to_string(&mut stdout_str)
                .map_err(|e| AlerterError::Runtime(format!("failed to read stdout: {e}")))?;
        }

        let stdout_str = stdout_str.trim().to_string();

        if self.json {
            AlerterResponse::from_json(&stdout_str)
        } else {
            Ok(AlerterResponse::from_plain_text(&stdout_str))
        }
    }

    pub fn try_wait(&mut self) -> Result<Option<AlerterResponse>, AlerterError> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| AlerterError::Runtime("handle already consumed".to_string()))?;

        match child
            .try_wait()
            .map_err(|e| AlerterError::Runtime(format!("failed to check child status: {e}")))?
        {
            None => Ok(None),
            Some(status) => {
                if !status.success() {
                    let mut stderr_str = String::new();
                    if let Some(mut stderr) = child.stderr.take() {
                        let _ = stderr.read_to_string(&mut stderr_str);
                    }
                    return Err(AlerterError::Runtime(stderr_str.trim().to_string()));
                }

                let mut stdout_str = String::new();
                if let Some(mut stdout) = child.stdout.take() {
                    stdout.read_to_string(&mut stdout_str).map_err(|e| {
                        AlerterError::Runtime(format!("failed to read stdout: {e}"))
                    })?;
                }

                let stdout_str = stdout_str.trim().to_string();

                if self.json {
                    AlerterResponse::from_json(&stdout_str).map(Some)
                } else {
                    Ok(Some(AlerterResponse::from_plain_text(&stdout_str)))
                }
            }
        }
    }

    pub fn detach(mut self) {
        self.child.take();
    }
}

impl Drop for NotificationHandle {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
