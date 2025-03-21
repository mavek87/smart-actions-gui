use crate::domain::app_state::AppState;
use crate::domain::startup_ui_metadata::StartupUIMetadata;
use crate::logic::action_config_parser::ActionConfigParser;
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::{command, State};

#[command]
pub fn ui_notify_startup(state: State<AppState>) -> String {
    // TODO: find a way to automatically read all the actions without hardcoding an array
    let action_names: [&str; 4] = [
        "dictate_text",
        "ai_reply_text",
        "audio_to_text",
        "text_to_speech",
    ];

    let smart_action_manager = &state.smart_action_manager;
    let is_audio_enabled = smart_action_manager.is_audio_enabled().unwrap_or(false);
    let current_language = state.language_manager.get_current_language();

    let mut startup_ui_metadata =
        StartupUIMetadata::new(is_audio_enabled, current_language.code().to_string());

    let config_manager = state.config_manager.lock().unwrap();
    let config = config_manager
        .read_config()
        .unwrap();

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

        startup_ui_metadata
            .actions
            .insert(action_name.to_string(), action_config);
    }

    let json_actions_metadata =
        serde_json::to_string(&startup_ui_metadata).expect("Failed to parse JSON");

    json_actions_metadata
}
