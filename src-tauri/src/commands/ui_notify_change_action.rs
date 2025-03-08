use crate::domain::app_state::AppState;
use crate::domain::smart_action::SmartAction;
use tauri::{command, AppHandle, State};

#[command]
pub fn ui_notify_change_action(
    json_smart_action: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    println!("ui_notify_change_action:- json_smart_action: {}", json_smart_action);

    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON Smart Action");

    state
        .menu_manager
        .lock()
        .unwrap()
        .set_action_name_text(format!("{}", smart_action.name));

    state
        .smart_action_manager
        .change_current_smart_action(smart_action);

    "OK".to_string()
}
