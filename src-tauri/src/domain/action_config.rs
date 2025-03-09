use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionConfig {
    pub name: String,
    pub description: String,
    pub defaults: IndexMap<String, String>,
    pub options: IndexMap<String, String>,
    pub mandatory_options: Vec<String>,
    pub examples: IndexMap<String, String>,
}
