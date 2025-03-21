use crate::domain::app_config::AppConfig;
use std::env;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

pub struct ConfigManager {}

impl ConfigManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_config(&self) -> Result<AppConfig, Error> {
        let system_path = env::var("PATH").unwrap_or_else(|_| "".to_string());

        let output = Command::new("bash")
            .arg("-c")
            .arg("which smart-actions.sh")
            .env("PATH", &system_path)
            .output()?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "smart-actions.sh not found. Add it to the PATH.",
            ));
        }

        let smart_actions_path = String::from_utf8(output.stdout)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
            .trim()
            .to_string(); // Rimuove newline e spazi extra

        if smart_actions_path.is_empty() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "smart-actions.sh not found. Add it to the PATH.",
            ));
        }

        let path = Path::new(&smart_actions_path);
        let file_name = path.file_name().and_then(|f| f.to_str()).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Error extracting file name from {}", &smart_actions_path),
            )
        })?;

        let folder = path.parent().and_then(|p| p.to_str()).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Error extracting folder from {}", &smart_actions_path),
            )
        })?;

        Ok(AppConfig {
            smart_actions_folder: folder.to_string(),
            smart_actions_executable: file_name.to_string(),
        })
    }
}
