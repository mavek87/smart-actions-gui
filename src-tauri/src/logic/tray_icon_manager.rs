use std::collections::HashMap;
use std::path::PathBuf;
use tauri::image::Image;
use tauri::path::BaseDirectory;
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct TrayIconManager {
    tray_icon: TrayIcon,
    icons_map: HashMap<AppIcon, PathBuf>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum AppIcon {
    DEFAULT,
    RECORDING,
    WAITING,
}

impl AppIcon {
    pub fn relative_icon_path(&self) -> &'static str {
        match self {
            AppIcon::DEFAULT => "icons/normal.ico",
            AppIcon::RECORDING => "icons/recording.ico",
            AppIcon::WAITING => "icons/waiting.ico",
        }
    }

    pub fn all_variants() -> &'static [AppIcon] {
        &[AppIcon::DEFAULT, AppIcon::RECORDING, AppIcon::WAITING]
    }
}

impl TrayIconManager {
    pub fn new(tray_icon: TrayIcon, app_handle: &AppHandle) -> Self {
        let mut icons_map = HashMap::new();

        for app_icon in AppIcon::all_variants() {
            let path = app_handle
                .path()
                .resolve(app_icon.relative_icon_path(), BaseDirectory::Resource)
                .expect(&format!("Unable to load {}", app_icon.relative_icon_path()));

            icons_map.insert(app_icon.clone(), path);
        }

        Self { tray_icon, icons_map }
    }

    pub fn show_default_icon(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_icon_from_path(&AppIcon::DEFAULT)
    }

    pub fn show_recording_icon(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_icon_from_path(&AppIcon::RECORDING)
    }

    pub fn show_waiting_icon(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_icon_from_path(&AppIcon::WAITING)
    }

    fn set_icon_from_path(&self, app_icon: &AppIcon) -> Result<(), Box<dyn std::error::Error>> {
        let icon_path = self.icons_map.get(app_icon).ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Icon path not found",
            ))
        })?;

        let image = Image::from_path(icon_path)?;

        self.tray_icon.set_icon(Some(image))?;

        Ok(())
    }
}
