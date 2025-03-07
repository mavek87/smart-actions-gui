use crate::domain::app_state::AppState;
use tauri::{command, AppHandle, State};
use crate::domain::smart_action::SmartAction;

#[command]
pub fn ui_notify_change_action(
    json_smart_action: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    //println!("json_smart_action: {}", json_smart_action);

    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON Smart Action");

    state
        .menu_manager
        .lock()
        .unwrap()
        .set_action_name_text(format!("{}", smart_action.name));

    let mut current_action_value = state.current_action_value.lock().unwrap();

    *current_action_value = smart_action.value.to_string();
    println!("current_action_value: {}", smart_action.value);

    // TODO: what is the purpose of returning??? probably useless
    current_action_value.to_string()
}
