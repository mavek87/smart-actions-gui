use crate::domain::app_state::AppState;
use crate::domain::startup_ui_metadata::StartupUIMetadata;
use crate::logic::action_config_parser::ActionConfigParser;
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::{command, State};

#[command]
pub fn ui_notify_startup(state: State<AppState>) -> String {
    let smart_action_manager = &state.smart_action_manager;
    let is_audio_enabled = smart_action_manager.is_audio_enabled().unwrap_or(false);
    let current_language = state.language_manager.get_current_language();

    let mut startup_ui_metadata =
        StartupUIMetadata::new(is_audio_enabled, current_language.code().to_string());

    let config_manager = state.config_manager.lock().unwrap();
    let config = config_manager.read_config().unwrap();

    let actions_json_output = Command::new("bash")
        .arg(format!(
            "{}/{}",
            config.smart_actions_folder, config.smart_actions_executable
        ))
        .arg("get_actions_json")
        .output()
        .expect("Failed to list smart actions");

    let json_str = String::from_utf8_lossy(&actions_json_output.stdout);

    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON from get_actions_json: {}", e);
        serde_json::Value::Array(vec![])
    });

    if let Some(action_array) = parsed.as_array() {
        for action in action_array {
            if let Some(action_name) = action.get("action_name").and_then(|v| v.as_str()) {
                let action_output = Command::new("bash")
                    .arg(format!(
                        "{}/{}",
                        config.smart_actions_folder, config.smart_actions_executable
                    ))
                    .arg(action_name)
                    .arg("--print-config")
                    .stdout(Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        let mut stdout = String::new();
                        if let Some(ref mut out) = child.stdout {
                            out.read_to_string(&mut stdout).ok();
                        }
                        Ok(stdout)
                    });

                let action_config_raw_output = action_output.unwrap_or_else(|e| {
                    eprintln!(
                        "Errore durante esecuzione comando per {}: {}",
                        action_name, e
                    );
                    "".to_string()
                });

                let action_config =
                    ActionConfigParser::parse_from_string(&action_config_raw_output);

                startup_ui_metadata
                    .actions
                    .insert(action_name.to_string(), action_config);
            }
        }
    } else {
        eprintln!("JSON ricevuto da get_actions_json non è un array.");
    }

    serde_json::to_string(&startup_ui_metadata).expect("Failed to serialize JSON")
}
