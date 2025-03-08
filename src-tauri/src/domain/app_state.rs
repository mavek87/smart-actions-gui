use std::sync::Mutex;
use crate::domain::smart_action::SmartAction;
use crate::logic::menu_manager::MenuManager;

pub struct AppState {
    pub current_smart_action: Mutex<SmartAction>,
    pub menu_manager: Mutex<MenuManager>
}