use crate::domain::{ActionConfig, ActionsMetadata, AppState, Action};
use std::io::Read;
use std::process::{Command, Stdio};
use tauri::{command, AppHandle, State};

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

        let action_config = ActionConfig::parse_from_string(&action_config_raw_output);

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

#[command]
pub fn ui_notify_change_action(
    value: &str,
    name: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    println!("value: {}", value);
    println!("name: {}", name);

    let menu_handle = state.menu_handle.lock().unwrap();
    menu_handle
        .get("action_state_item")
        .unwrap()
        .as_menuitem()
        .unwrap()
        .set_text(format!("{}", name))
        .unwrap();

    let mut current_action_value = state.current_action_value.lock().unwrap();

    *current_action_value = value.to_string();
    println!("current_action_value: {}", value);

    // let menu = app.menu().unwrap();
    // let kind = menu.get("action_state_item").unwrap();
    // kind.as_menuitem().unwrap().set_text(format!("{}", name)).unwrap();

    current_action_value.to_string()
}

#[command]
pub fn ui_request_execute_action(json_action: String) {

    // println!("AAAAAAAAAAAAAAAAAAAAAAAAAAA\n\n\n");

    // println!("{}", &json_action);

    let action: Action = serde_json::from_str(&json_action).expect("Failed to parse JSON");
    println!("?:{:?}", action);
    // println!("?:{}", action);
}
