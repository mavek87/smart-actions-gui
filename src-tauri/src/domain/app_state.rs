use crate::logic::menu_manager::MenuManager;
use std::sync::Mutex;
use crate::logic::smart_action_manager::SmartActionManager;

pub struct AppState {
    pub smart_action_manager: SmartActionManager,
    // TODO: remove menu_manager from here if not used
    pub menu_manager: Mutex<MenuManager>,
}