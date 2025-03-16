use crate::domain::actions_metadata::ActionsMetadata;
use crate::domain::app_state::AppState;
use crate::domain::constants::DEFAULT_CONFIG_FILE;
use crate::logic::action_config_parser::ActionConfigParser;
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::{command, State};

#[command]
pub fn ui_notify_startup(state: State<AppState>) -> String {
    // TODO: find a way to automatically read all the actions without hardcoding an array
    let action_names: [&str; 4] = ["dictate_text", "ai_reply_text", "audio_to_text", "text_to_speech"];

    let smart_action_manager = &state.smart_action_manager;
    let is_audio_enabled = smart_action_manager.is_audio_enabled().unwrap_or(false);
    let mut actions_metadata = ActionsMetadata::new(is_audio_enabled);

    let config_manager = state.config_manager.lock().unwrap();

    let config = config_manager
        .read_config(DEFAULT_CONFIG_FILE)
        .expect(&format!(
            "Error reading config file {}",
            DEFAULT_CONFIG_FILE
        ));

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
