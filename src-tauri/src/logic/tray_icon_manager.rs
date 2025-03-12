use tauri::image::Image;
use tauri::tray::TrayIcon;

#[derive(Clone)]
pub struct TrayIconManager {
    tray_icon: TrayIcon,
}

const DEFAULT_APP_ICON_PATH: &str = "icons/normal.ico";
const RECORDING_APP_ICON_PATH: &str = "icons/recording.ico";
const WAITING_APP_ICON_PATH: &str = "icons/waiting.ico";

impl TrayIconManager {
    pub fn new(tray_icon: TrayIcon) -> Self {
        Self { tray_icon }
    }

    pub fn show_default_icon(&self) {
        self.set_icon_from_path(DEFAULT_APP_ICON_PATH);
    }

    pub fn show_recording_icon(&self) {
        self.set_icon_from_path(RECORDING_APP_ICON_PATH);
    }

    pub fn show_waiting_icon(&self) {
        self.set_icon_from_path(WAITING_APP_ICON_PATH);
    }

    fn set_icon_from_path(&self, icon_path: &str) {
        let image = Image::from_path(icon_path)
            .expect(format!("Failed to load image from path {}", icon_path).as_str());

        self.tray_icon
            .set_icon(Some(image))
            .expect(format!("Failed to set icon from path {}", icon_path).as_str());
    }
}
