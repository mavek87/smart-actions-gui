use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub smart_actions_folder: String,
    pub smart_actions_executable: String,
}
