use crate::domain::app_state::AppState;
use crate::domain::smart_action::SmartAction;
use tauri::{command, State};

#[command]
pub fn ui_request_execute_action(json_smart_action: String, app_state: State<AppState>) {
    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON");
    println!("ui_request_execute_action :- SmartAction ?:{:?}", smart_action);

    app_state
        .menu_manager
        .lock()
        .unwrap()
        .set_action_started()
}