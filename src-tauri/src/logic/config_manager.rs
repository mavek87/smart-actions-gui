use crate::domain::app_config::AppConfig;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub struct ConfigManager {}

impl ConfigManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_config(&self, config_file_path: String) -> Result<AppConfig, Error> {
        let mut file_config = File::open(config_file_path)?;

        let mut file_content = String::new();

        file_config.read_to_string(&mut file_content)?;

        serde_json::from_str(&file_content).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}