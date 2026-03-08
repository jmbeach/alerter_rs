use crate::binary::extract_binary;
use crate::error::AlerterError;
use crate::handle::NotificationHandle;
use crate::response::AlerterResponse;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Alerter {
    message: String,
    title: Option<String>,
    subtitle: Option<String>,
    sound: Option<String>,
    actions: Option<Vec<String>>,
    dropdown_label: Option<String>,
    reply: Option<String>,
    close_label: Option<String>,
    group: Option<String>,
    sender: Option<String>,
    app_icon: Option<String>,
    content_image: Option<String>,
    timeout: Option<u32>,
    json: bool,
    delay: Option<u32>,
    at: Option<String>,
    ignore_dnd: bool,
    binary_path: Option<PathBuf>,
}

fn resolve_binary(override_path: Option<&Path>) -> Result<PathBuf, AlerterError> {
    match override_path {
        Some(p) => Ok(p.to_path_buf()),
        None => extract_binary().map_err(|e| AlerterError::BinaryExtraction(e.to_string())),
    }
}

fn run_alerter(binary: &Path, args: &[&str]) -> Result<std::process::Output, AlerterError> {
    let output = Command::new(binary)
        .args(args)
        .output()
        .map_err(|e| AlerterError::ProcessSpawn(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(AlerterError::Runtime(stderr));
    }

    Ok(output)
}

impl Alerter {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            title: None,
            subtitle: None,
            sound: None,
            actions: None,
            dropdown_label: None,
            reply: None,
            close_label: None,
            group: None,
            sender: None,
            app_icon: None,
            content_image: None,
            timeout: None,
            json: false,
            delay: None,
            at: None,
            ignore_dnd: false,
            binary_path: None,
        }
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn binary_path(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        self.binary_path = Some(path.into());
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn subtitle(&mut self, subtitle: &str) -> &mut Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn sound(&mut self, sound: &str) -> &mut Self {
        self.sound = Some(sound.to_string());
        self
    }

    pub fn actions(&mut self, actions: Vec<&str>) -> &mut Self {
        self.actions = Some(actions.into_iter().map(String::from).collect());
        self
    }

    pub fn dropdown_label(&mut self, label: &str) -> &mut Self {
        self.dropdown_label = Some(label.to_string());
        self
    }

    pub fn reply(&mut self, placeholder: &str) -> &mut Self {
        self.reply = Some(placeholder.to_string());
        self
    }

    pub fn close_label(&mut self, label: &str) -> &mut Self {
        self.close_label = Some(label.to_string());
        self
    }

    pub fn group(&mut self, group: &str) -> &mut Self {
        self.group = Some(group.to_string());
        self
    }

    pub fn sender(&mut self, sender: &str) -> &mut Self {
        self.sender = Some(sender.to_string());
        self
    }

    pub fn app_icon(&mut self, path: &str) -> &mut Self {
        self.app_icon = Some(path.to_string());
        self
    }

    pub fn content_image(&mut self, path: &str) -> &mut Self {
        self.content_image = Some(path.to_string());
        self
    }

    pub fn timeout(&mut self, seconds: u32) -> &mut Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn json(&mut self, enabled: bool) -> &mut Self {
        self.json = enabled;
        self
    }

    pub fn delay(&mut self, seconds: u32) -> &mut Self {
        self.delay = Some(seconds);
        self
    }

    pub fn at(&mut self, time: &str) -> &mut Self {
        self.at = Some(time.to_string());
        self
    }

    pub fn ignore_dnd(&mut self, enabled: bool) -> &mut Self {
        self.ignore_dnd = enabled;
        self
    }

    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec!["--message".to_string(), self.message.clone()];

        if let Some(ref v) = self.title {
            args.push("--title".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.subtitle {
            args.push("--subtitle".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.sound {
            args.push("--sound".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.actions {
            args.push("--actions".to_string());
            args.push(v.join(","));
        }
        if let Some(ref v) = self.dropdown_label {
            args.push("--dropdown-label".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.reply {
            args.push("--reply".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.close_label {
            args.push("--close-label".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.group {
            args.push("--group".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.sender {
            args.push("--sender".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.app_icon {
            args.push("--app-icon".to_string());
            args.push(v.clone());
        }
        if let Some(ref v) = self.content_image {
            args.push("--content-image".to_string());
            args.push(v.clone());
        }
        if let Some(v) = self.timeout {
            args.push("--timeout".to_string());
            args.push(v.to_string());
        }
        if self.json {
            args.push("--json".to_string());
        }
        if let Some(v) = self.delay {
            args.push("--delay".to_string());
            args.push(v.to_string());
        }
        if let Some(ref v) = self.at {
            args.push("--at".to_string());
            args.push(v.clone());
        }
        if self.ignore_dnd {
            args.push("--ignore-dnd".to_string());
        }

        args
    }

    pub fn send_async(&self) -> Result<NotificationHandle, AlerterError> {
        let binary = resolve_binary(self.binary_path.as_deref())?;
        let args: Vec<String> = self.build_args();

        let child = Command::new(&binary)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| AlerterError::ProcessSpawn(e.to_string()))?;

        Ok(NotificationHandle::new(child, self.json))
    }

    pub fn send(&self) -> Result<AlerterResponse, AlerterError> {
        let binary = resolve_binary(self.binary_path.as_deref())?;
        let args: Vec<String> = self.build_args();
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        let output = run_alerter(&binary, &arg_refs)?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if self.json {
            AlerterResponse::from_json(&stdout)
        } else {
            Ok(AlerterResponse::from_plain_text(&stdout))
        }
    }

    pub fn remove(group_id: &str) -> Result<(), AlerterError> {
        Self::remove_with_binary(group_id, None)
    }

    pub fn remove_with_binary(group_id: &str, binary: Option<&Path>) -> Result<(), AlerterError> {
        let binary = resolve_binary(binary)?;
        run_alerter(&binary, &["--remove", group_id])?;
        Ok(())
    }

    pub fn list(group_id: &str) -> Result<String, AlerterError> {
        Self::list_with_binary(group_id, None)
    }

    pub fn list_with_binary(group_id: &str, binary: Option<&Path>) -> Result<String, AlerterError> {
        let binary = resolve_binary(binary)?;
        let output = run_alerter(&binary, &["--list", group_id])?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
