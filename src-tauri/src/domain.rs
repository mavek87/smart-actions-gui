use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::menu::Menu;
use tauri::Wry;

pub struct AppState {
    pub menu_handle: Mutex<Menu<Wry>>,
    pub current_action_value: Mutex<String>,
}

// TODO: add the app config into the app state as a mutex

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub faster_whisper_folder: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionsMetadata {
    pub actions: HashMap<String, ActionConfig>,
}

impl ActionsMetadata {
    pub fn new() -> Self {
        ActionsMetadata {
            actions: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionConfig {
    name: String,
    description: String,
    defaults: HashMap<String, String>,
    options: HashMap<String, String>,
    mandatory_options: Vec<String>,
    examples: HashMap<String, String>,
}

impl ActionConfig {
    pub fn parse_from_string(contents: &str) -> Self {
        let mut name = String::new();
        let mut description = String::new();
        let mut defaults = HashMap::new();
        let mut options = HashMap::new();
        let mut mandatory_options = Vec::new();
        let mut examples = HashMap::new();

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue; // Salta le righe vuote e i commenti
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"'); // Rimuove eventuali virgolette

                if key == "NAME" {
                    name = value.to_string();
                } else if key == "DESCRIPTION" {
                    description = value.to_string();
                } else if key.starts_with("DEFAULTS_") {
                    defaults.insert(key["DEFAULTS_".len()..].to_string(), value.to_string());
                } else if key.starts_with("OPTIONS_") {
                    options.insert(key["OPTIONS_".len()..].to_string(), value.to_string());
                } else if key == "MANDATORY_OPTIONS" {
                    mandatory_options = value.split_whitespace().map(String::from).collect();
                } else if key.starts_with("EXAMPLES_") {
                    examples.insert(key["EXAMPLES_".len()..].to_string(), value.to_string());
                }
            }
        }

        Self {
            name,
            description,
            defaults,
            options,
            mandatory_options,
            examples,
        }
    }

    // fn from_file(path: &Path) -> std::io::Result<Self> {
    //     let contents = fs::read_to_string(path)?;
    //     Ok(Self::parse_from_string(&contents))
    // }
}
