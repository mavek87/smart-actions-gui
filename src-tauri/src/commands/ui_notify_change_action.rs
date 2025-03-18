use crate::domain::app_state::AppState;
use crate::domain::smart_action::SmartAction;
use tauri::{command, AppHandle, State};

#[command]
pub fn ui_notify_change_action(
    json_smart_action: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    let smart_action: SmartAction =
        serde_json::from_str(&json_smart_action).expect("Failed to parse JSON Smart Action");

    let smart_action_clone = smart_action.clone();
    let smart_action_value_ref = &smart_action.value;

    state
        .smart_action_manager
        .change_current_smart_action(smart_action_clone);

    state
        .audio_player_manager
        .lock()
        .unwrap()
        .play_sound_for_smart_action(smart_action_value_ref)
        .inspect_err(|e| eprintln!("{}", e))
        .ok();

    "OK".to_string()
}
