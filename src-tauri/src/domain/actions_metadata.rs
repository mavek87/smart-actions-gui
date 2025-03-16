use super::action_config::ActionConfig;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionsMetadata {
    pub is_audio_enabled: bool,
    pub actions: IndexMap<String, ActionConfig>,
}

impl ActionsMetadata {
    pub fn new(is_audio_enabled: bool) -> Self {
        ActionsMetadata {
            actions: IndexMap::new(),
            is_audio_enabled,
        }
    }
}
