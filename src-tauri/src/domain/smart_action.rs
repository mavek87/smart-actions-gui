use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmartAction {
    pub name: String,
    pub value: String,
    pub description: String,
    pub args: Vec<HashMap<String, String>>,
}

#[allow(dead_code)]
pub struct SmartActionState {
    pub name: Arc<Mutex<String>>, // TODO: remove this field and allow(dead_code) if not used
    pub value: Arc<Mutex<String>>,
    pub description: Arc<Mutex<String>>, // TODO: remove this field and allow(dead_code) if not used
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

// TODO: improve the possible statuses (off [completed|failed|not_executed?], running, recording, waiting, etc...)
#[derive(Debug, Serialize, Deserialize)]
pub enum SmartActionStatus {
    SELECTED,
    ON,
    OFF,
}

impl SmartActionStatus {
    pub fn value(&self) -> &'static str {
        match self {
            SmartActionStatus::SELECTED => "selected",
            SmartActionStatus::ON => "on",
            SmartActionStatus::OFF => "off",
        }
    }
}

impl Display for SmartActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let valore = match self {
            SmartActionStatus::SELECTED => "selected",
            SmartActionStatus::ON => "on",
            SmartActionStatus::OFF => "off",
        };
        write!(f, "{}", valore)
    }
}