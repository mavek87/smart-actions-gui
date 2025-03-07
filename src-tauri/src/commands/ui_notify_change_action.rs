use crate::domain::app_state::AppState;
use tauri::{command, AppHandle, State};

#[command]
pub fn ui_notify_change_action(
    value: &str,
    name: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    println!("value: {}", value);
    println!("name: {}", name);

    state
        .menu_manager
        .lock()
        .unwrap()
        .set_action_name_text(format!("{}", name));

    let mut current_action_value = state.current_action_value.lock().unwrap();

    *current_action_value = value.to_string();
    println!("current_action_value: {}", value);

    current_action_value.to_string()
}
