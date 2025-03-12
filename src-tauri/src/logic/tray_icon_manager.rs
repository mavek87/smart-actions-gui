use std::string::ToString;
use tauri::image::Image;
use tauri::tray::TrayIcon;

#[derive(Clone)]
pub struct TrayIconManager {
    tray_icon: TrayIcon,
    default_app_icon_path: String,
    recording_app_icon_path: String,
    waiting_app_icon_path: String,
}

// https://icon-icons.com/icon/chat-dialogue-bubbles-bubble-talk-conversation-green/65972
// https://icon-icons.com/icon/chat-dialogue-bubbles-bubble-talk-orange/65944
// https://icon-icons.com/icon/chat-dialogue-bubbles-bubble-talk-blue/65943
// https://icon-icons.com/icon/chat-dialogue-bubbles-bubble-talk-round/65949
impl TrayIconManager {
    pub fn new(tray_icon: TrayIcon) -> Self {
        Self {
            tray_icon,
            default_app_icon_path: "icons/normal.ico".to_string(),
            recording_app_icon_path: "icons/recording.ico".to_string(),
            waiting_app_icon_path: "icons/waiting.ico".to_string(),
        }
    }

    pub fn set_default_icon(&self) {
        self.set_icon_from_path(self.default_app_icon_path.clone());
    }

    pub fn set_recording_icon(&self) {
        self.set_icon_from_path(self.recording_app_icon_path.clone());
    }

    pub fn set_waiting_icon(&self) {
        self.set_icon_from_path(self.waiting_app_icon_path.clone());
    }

    fn set_icon_from_path(&self, icon_path: String) {
        let image = Image::from_path(icon_path).unwrap();
        self.tray_icon.set_icon(Some(image)).unwrap();
    }
}
