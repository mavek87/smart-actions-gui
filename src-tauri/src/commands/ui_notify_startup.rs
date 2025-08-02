use crate::domain::app_state::AppState;
use crate::domain::startup_ui_metadata::StartupUIMetadata;
use crate::logic::action_config_parser::ActionConfigParser;
use std::process::Command;
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

    let script_path = format!(
        "{}/{}",
        config.smart_actions_folder, config.smart_actions_executable
    );
    let actions_json = run_command(&script_path, &["get_actions_json"]).unwrap_or_default();

    let parsed: serde_json::Value = serde_json::from_str(&actions_json).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON from get_actions_json: {}", e);
        serde_json::Value::Array(vec![])
    });

    if let Some(action_array) = parsed.as_array() {
        for action in action_array {
            if let Some(action_name) = action.get("action_name").and_then(|v| v.as_str()) {
                let action_output =
                    run_command(&script_path, &[action_name, "--print-config"]).unwrap_or_default();

                let action_config = ActionConfigParser::parse_from_string(&action_output);

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

fn run_command(interpreter: &str, args: &[&str]) -> Option<String> {
    let mut cmd = Command::new(interpreter);
    for arg in args {
        cmd.arg(arg);
    }
    match cmd.output() {
        Ok(output) => Some(String::from_utf8_lossy(&output.stdout).to_string()),
        Err(e) => {
            eprintln!("Failed to run command: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_bash_command_echo() {
        // Usa bash -c per eseguire direttamente il comando echo
        let output = run_command("/bin/bash", &["-c", "echo ciao mondo"]);
        assert_eq!(output.unwrap().trim(), "ciao mondo");
    }

    #[test]
    fn test_run_sh_command_echo() {
        // Usa bash -c per eseguire direttamente il comando echo
        let output = run_command("/bin/sh", &["-c", "echo ciao mondo"]);
        assert_eq!(output.unwrap().trim(), "ciao mondo");
    }
}
