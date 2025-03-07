use super::action_config::ActionConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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