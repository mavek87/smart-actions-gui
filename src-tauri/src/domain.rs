use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::menu::Menu;
use tauri::Wry;
use indexmap::IndexMap;

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
pub struct Action {
    name: String,
    value: String,
    description: String,
    args: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionConfig {
    name: String,
    description: String,
    defaults: IndexMap<String, String>,
    options: IndexMap<String, String>,
    mandatory_options: Vec<String>,
    examples: IndexMap<String, String>,
}

impl ActionConfig {
    pub fn parse_from_string(contents: &str) -> Self {
        let mut name = String::new();
        let mut description = String::new();
        let mut defaults = IndexMap::new();
        let mut options = IndexMap::new();
        let mut mandatory_options = Vec::new();
        let mut examples = IndexMap::new();

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue; // Skip empty rows and comments
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
}
