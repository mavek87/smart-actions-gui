use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartAction {
    name: String,
    value: String,
    description: String,
    args: Vec<HashMap<String, String>>,
}