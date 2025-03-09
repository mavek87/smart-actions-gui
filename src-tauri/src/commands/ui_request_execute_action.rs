use crate::domain::app_state::AppState;
use crate::domain::smart_action::SmartAction;
use tauri::{command, State};

#[command]
pub fn ui_request_execute_action(
    json_smart_action: String,
    app_state: State<AppState>,
    // app: tauri::AppHandle,
) {
    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON");

    // println!(
    //     "ui_request_execute_action :- SmartAction ?:{:?}",
    //     smart_action
    // );

    let tray_icon_manager = app_state.tray_icon_manager.lock().unwrap();
    tray_icon_manager.set_recording_icon();

    let smart_action_manager = &app_state.smart_action_manager;
    smart_action_manager.change_current_smart_action(smart_action);
    smart_action_manager.start_current_smart_action();
}
