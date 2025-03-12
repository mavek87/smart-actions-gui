use crate::domain::actions_metadata::ActionsMetadata;
use crate::domain::app_state::AppState;
use crate::logic::action_config_parser::ActionConfigParser;
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::{command, State};
use crate::domain::constants::DEFAULT_CONFIG_FILE;

#[command]
pub fn ui_notify_startup(state: State<AppState>) -> String {
    let action_names: [&str; 3] = ["dictate_text", "ai_reply_text", "audio_to_text"];

    let mut actions_metadata = ActionsMetadata::new();

    let config_manager = state.config_manager.lock().unwrap();

    let config = config_manager
        .read_config(DEFAULT_CONFIG_FILE)
        .expect(&format!("Error reading config file {}", DEFAULT_CONFIG_FILE));

    for action_name in &action_names {
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
            eprintln!("Errore durante l'esecuzione del comando: {}", e);
            "".to_string()
        });

        let action_config = ActionConfigParser::parse_from_string(&action_config_raw_output);

        actions_metadata
            .actions
            .insert(action_name.to_string(), action_config);
    }

    let json_actions_metadata =
        serde_json::to_string(&actions_metadata).expect("Failed to parse JSON");

    json_actions_metadata
}
