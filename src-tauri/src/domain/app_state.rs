use std::sync::Mutex;
use tauri::menu::Menu;
use tauri::Wry;

pub struct AppState {
    pub menu_handle: Mutex<Menu<Wry>>,
    pub current_action_value: Mutex<String>,
}