use crate::domain::action_config::ActionConfig;
use indexmap::IndexMap;

pub struct ActionConfigParser;

impl ActionConfigParser {
    pub fn parse_from_string(contents: &str) -> ActionConfig {
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

        ActionConfig {
            name,
            description,
            defaults,
            options,
            mandatory_options,
            examples,
        }
    }
}
