use crate::logic::audio_player_manager::AudioPlayerManager;
use crate::logic::config_manager::ConfigManager;
use crate::logic::smart_action_manager::SmartActionManager;
use crate::logic::tray_icon_manager::TrayIconManager;
use std::sync::Mutex;
use crate::logic::language_manager::LanguageManager;

pub struct AppState {
    pub smart_action_manager: SmartActionManager,
    pub tray_icon_manager: Mutex<TrayIconManager>,
    pub config_manager: Mutex<ConfigManager>,
    pub audio_player_manager: Mutex<AudioPlayerManager>,
    pub language_manager: LanguageManager
}
