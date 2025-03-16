use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub smart_actions_folder: String,
    pub smart_actions_executable: String,
}
