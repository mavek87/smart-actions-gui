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
