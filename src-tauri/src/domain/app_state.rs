use std::sync::Mutex;
use crate::logic::menu_manager::MenuManager;
use crate::logic::smart_action_manager::SmartActionManager;

pub struct AppState {
    pub smart_action_manager: SmartActionManager,
    pub menu_manager: Mutex<MenuManager>,
}