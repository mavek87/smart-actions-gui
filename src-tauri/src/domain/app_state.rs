use crate::logic::smart_action_manager::SmartActionManager;
use crate::logic::tray_icon_manager::TrayIconManager;
use std::sync::Mutex;

pub struct AppState {
    pub smart_action_manager: SmartActionManager,
    pub tray_icon_manager: Mutex<TrayIconManager>,
}