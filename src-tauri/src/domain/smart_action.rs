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
#[derive(Debug, Clone)]
pub struct SmartActionState {
    pub name: Arc<Mutex<String>>, // TODO: remove this field and allow(dead_code) if not used
    pub value: Arc<Mutex<String>>,
    pub description: Arc<Mutex<String>>, // TODO: remove this field and allow(dead_code) if not used
    pub args: Arc<Mutex<Vec<HashMap<String, String>>>>,
    pub status: Arc<Mutex<SmartActionStatus>>,
}

impl SmartActionState {
    pub fn new(smart_action: SmartAction) -> Self {
        SmartActionState {
            name: Arc::new(Mutex::new(smart_action.name)),
            value: Arc::new(Mutex::new(smart_action.value)),
            description: Arc::new(Mutex::new(smart_action.description)),
            args: Arc::new(Mutex::new(smart_action.args)),
            status: Arc::new(Mutex::new(SmartActionStatus::SELECTED)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmartActionStatus {
    SELECTED,
    RECORDING,
    WAITING,
    COMPLETED,
    FAILED,
}

impl SmartActionStatus {
    pub fn value(&self) -> &'static str {
        match self {
            SmartActionStatus::SELECTED => "selected",
            SmartActionStatus::RECORDING => "recording",
            SmartActionStatus::WAITING => "waiting",
            SmartActionStatus::COMPLETED => "completed",
            SmartActionStatus::FAILED => "failed",
        }
    }
}

impl Display for SmartActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
