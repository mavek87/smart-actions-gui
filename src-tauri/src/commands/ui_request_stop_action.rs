use crate::domain::app_state::AppState;
use tauri::{command, State};

#[command]
pub fn ui_request_stop_action(app_state: State<AppState>) {
    let smart_action_manager = &app_state.smart_action_manager;
    smart_action_manager.stop_current_smart_action();
}