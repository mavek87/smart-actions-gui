use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartAction {
    pub name: String,
    pub value: String,
    pub description: String,
    pub args: Vec<HashMap<String, String>>,
}

pub struct SmartActionState {
    pub name: Arc<Mutex<String>>,
    pub value: Arc<Mutex<String>>,
    pub description: Arc<Mutex<String>>,
    pub args: Arc<Mutex<Vec<HashMap<String, String>>>>,
}

impl SmartActionState {
    pub fn new(smart_action: SmartAction) -> Self {
        SmartActionState {
            name: Arc::new(Mutex::new(smart_action.name)),
            value: Arc::new(Mutex::new(smart_action.value)),
            description: Arc::new(Mutex::new(smart_action.description)),
            args: Arc::new(Mutex::new(smart_action.args)),
        }
    }
}