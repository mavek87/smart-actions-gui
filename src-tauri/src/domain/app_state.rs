use std::sync::Mutex;
// use tauri::menu::Menu;
// use tauri::Wry;
use crate::logic::menu_action_state_manager::MenuManager;

pub struct AppState {
    // pub menu_handle: Mutex<Menu<Wry>>,
    pub current_action_value: Mutex<String>,
    pub menu_manager: Mutex<MenuManager>
}