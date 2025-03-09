use indexmap::IndexMap;
use super::action_config::ActionConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionsMetadata {
    pub actions: IndexMap<String, ActionConfig>,
}

impl ActionsMetadata {
    pub fn new() -> Self {
        ActionsMetadata {
            actions: IndexMap::new(),
        }
    }
}