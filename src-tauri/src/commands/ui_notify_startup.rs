use crate::domain::actions_metadata::ActionsMetadata;
use crate::logic::action_config_parser::ActionConfigParser;
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::command;

#[command]
pub fn ui_notify_startup() -> String {
    let action_names: [&str; 3] = ["dictate_text", "ai_reply_text", "audio_to_text"];

    let mut actions_metadata = ActionsMetadata::new();

    for action_name in &action_names {
        // TODO 1: find a way to use config
        let action_output = Command::new("bash")
            .arg("/opt/FasterWhisper/smart-actions.sh") // Nessun bisogno di `format!()`
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

        // println!("{:#?}", action_config);

        actions_metadata
            .actions
            .insert(action_name.to_string(), action_config);
    }

    let json_actions_metadata =
        serde_json::to_string(&actions_metadata).expect("Failed to parse JSON");

    println!("JSON delle azioni: {}", json_actions_metadata);

    json_actions_metadata
}
