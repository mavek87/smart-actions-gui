use crate::domain::app_state::AppState;
use crate::domain::smart_action::SmartAction;
use tauri::{command, AppHandle, State};

#[command]
pub fn ui_notify_change_element_in_action(
    json_smart_action: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON Smart Action");

    state
        .smart_action_manager
        .change_current_smart_action(smart_action);

    "OK".to_string()
}
