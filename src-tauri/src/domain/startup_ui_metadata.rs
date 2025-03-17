use super::action_config::ActionConfig;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StartupUIMetadata {
    current_language: String,
    is_audio_enabled: bool,
    pub actions: IndexMap<String, ActionConfig>,
}

impl StartupUIMetadata {
    pub fn new(is_audio_enabled: bool, current_language: String) -> Self {
        StartupUIMetadata {
            current_language,
            actions: IndexMap::new(),
            is_audio_enabled,
        }
    }
}
