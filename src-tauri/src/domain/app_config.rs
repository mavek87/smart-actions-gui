use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub faster_whisper_folder: String,
}