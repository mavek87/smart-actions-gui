use crate::logic::menu_manager::MenuManager;
use std::sync::Mutex;
use crate::logic::smart_action_state_manager::SmartActionStateManager;

pub struct AppState {
    pub smart_action_state_manager: SmartActionStateManager,
    pub menu_manager: Mutex<MenuManager>,
}