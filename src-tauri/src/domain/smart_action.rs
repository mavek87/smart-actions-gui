use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartAction {
    pub name: String,
    pub value: String,
    pub description: String,
    pub args: Vec<HashMap<String, String>>,
}