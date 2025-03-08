use crate::domain::smart_action::SmartActionState;
use crate::logic::menu_manager::MenuManager;
use std::sync::Mutex;

pub struct AppState {
    pub current_smart_action: Mutex<SmartActionState>,
    pub menu_manager: Mutex<MenuManager>,
}
