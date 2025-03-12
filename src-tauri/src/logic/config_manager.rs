use crate::domain::app_config::AppConfig;
use std::fs;
use std::io::{Error, ErrorKind, Read};

pub struct ConfigManager {}

impl ConfigManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_config(&self, config_file_path: &str) -> Result<AppConfig, Error> {
        let file_content = fs::read_to_string(config_file_path)?;
        serde_json::from_str(&file_content).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}