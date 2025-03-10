mod commands;
mod domain;
mod logic;

use std::collections::HashMap;
use std::string::ToString;
use std::sync::Mutex;
use tauri::{
    menu::{
        AboutMetadataBuilder, CheckMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder,
        SubmenuBuilder,
    },
    tray::TrayIconBuilder,
    Manager, State,
};

use commands::{
    ui_notify_change_action::ui_notify_change_action, ui_notify_startup::ui_notify_startup,
    ui_request_execute_action::ui_request_execute_action,
    ui_request_stop_action::ui_request_stop_action,
};

use domain::{app_state::AppState, smart_action::SmartAction};

use logic::{
    audio_player_manager::AudioPlayerManager, config_manager::ConfigManager,
    menu_manager::MenuManager, smart_action_manager::SmartActionManager,
    tray_icon_manager::TrayIconManager,
};

const CONFIG_FILE: &str = "assets/config.json";
const APP_NAME: &str = "smart-actions-gui";
const APP_VERSION: &str = "0.1.0";
const WEBSITE_LABEL: &str = "Github Repository";
const WEBSITE: &str = "https://github.com/mavek87/smart-actions-gui";
const AUTHORS: &[&str] = &["Matteo Veroni"];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_manager: ConfigManager = ConfigManager::new();

    let app_config = config_manager.read_config(CONFIG_FILE.to_string()).unwrap();
    println!("app_config: {:?}", app_config);

    tauri::Builder::default()
        .setup(|app| {
            let action_name_menu_item = MenuItemBuilder::new("Dictate Text")
                .id("action_name_menu_item")
                .enabled(false)
                .build(app)?;

            let start_action_menu_item =
                MenuItem::with_id(app, "start", "Start", true, Some("Ctrl+N"))?;

            let stop_action_menu_item =
                MenuItem::with_id(app, "stop", "Stop", false, Some("Ctrl+E"))?;

            // TODO: use the current language if present (A language manager could be created too)
            //let lang_str = "unset";

            // https://v2.tauri.app/learn/window-menu/
            let lang_sub_item_unset = CheckMenuItemBuilder::with_id("unset", "Unset")
                .checked(true)
                .build(app)?;

            let lang_sub_item_en = CheckMenuItemBuilder::with_id("en", "English")
                .checked(false)
                .build(app)?;

            let lang_sub_item_it = CheckMenuItemBuilder::with_id("it", "Italian")
                .checked(false)
                .build(app)?;

            let language_submenu = SubmenuBuilder::new(app, "Language")
                .item(&lang_sub_item_unset)
                .item(&lang_sub_item_en)
                .item(&lang_sub_item_it)
                .build()?;

            let about_metadata = AboutMetadataBuilder::new()
                .name(Some(APP_NAME.to_string()))
                .version(Some(APP_VERSION.to_string()))
                .website_label(Some(WEBSITE_LABEL.to_string()))
                .website(Some(WEBSITE.to_string()))
                .authors(Some(AUTHORS.iter().map(|&s| s.to_string()).collect()))
                .build();

            let audio_sub_item_enabled = CheckMenuItemBuilder::with_id("audio_enabled", "Enabled")
                .checked(true)
                .build(app)?;

            let audio_submenu = SubmenuBuilder::new(app, "Audio")
                .item(&audio_sub_item_enabled)
                .build()?;

            let help_submenu = SubmenuBuilder::new(app, "Help")
                .about(Some(about_metadata))
                .build()?;

            let quit_item = MenuItemBuilder::new("Quit")
                .id("quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&action_name_menu_item)
                .separator()
                .items(&[&start_action_menu_item, &stop_action_menu_item])
                .separator()
                .item(&language_submenu)
                .item(&audio_submenu)
                .separator()
                .item(&help_submenu)
                .separator()
                .item(&quit_item)
                .build()?;

            let menu_manager = MenuManager::new(
                action_name_menu_item,
                start_action_menu_item,
                stop_action_menu_item,
            );

            let tray_icon = TrayIconBuilder::new()
                // .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "start" => {
                        println!("start record was clicked");
                        let app_state: State<AppState> = app.state();
                        app_state.smart_action_manager.start_current_smart_action();
                    }
                    "stop" => {
                        println!("stop record was clicked");
                        let app_state: State<AppState> = app.state();
                        app_state.smart_action_manager.stop_current_smart_action();
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "unset" | "en" | "it" => {
                        lang_sub_item_unset
                            .set_checked(event.id().0.as_str() == "unset")
                            .expect("Change check error");
                        lang_sub_item_en
                            .set_checked(event.id().0.as_str() == "en")
                            .expect("Change check error");
                        audio_sub_item_enabled
                            .set_checked(event.id().0.as_str() == "it")
                            .expect("Change check error");
                    }
                    "audio_enabled" => {
                        // TODO: fix bug
                        audio_sub_item_enabled
                            .set_checked(event.id().0.as_str() == "audio_enabled")
                            .expect("Change check error");
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(true)
                .build(app)?;

            let tray_icon_manager = TrayIconManager::new(tray_icon.clone());
            tray_icon_manager.set_default_icon();

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["alt+s", "alt+a", "alt+n", "alt+b"])?
                        .with_handler(|app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::ALT, Code::KeyS) {
                                    println!("ALT+S Pressed! - start smart action");
                                    let app_state: State<AppState> = app.state();
                                    app_state.smart_action_manager.start_current_smart_action();
                                } else if shortcut.matches(Modifiers::ALT, Code::KeyA) {
                                    println!("ALT+A Pressed! - stop smart action");
                                    let app_state: State<AppState> = app.state();
                                    app_state.smart_action_manager.stop_current_smart_action();
                                } else if shortcut.matches(Modifiers::ALT, Code::KeyN) {
                                    println!("ALT+N Pressed! - change with next smart action");
                                    let app_state: State<AppState> = app.state();
                                    app_state.smart_action_manager.change_with_next_smart_action();
                                } else if shortcut.matches(Modifiers::ALT, Code::KeyB) {
                                    println!("ALT+B Pressed! - change with previous smart action");
                                    let app_state: State<AppState> = app.state();
                                    app_state.smart_action_manager.change_with_previous_smart_action();
                                }
                            }
                        })
                        .build(),
                )?;
            }

            let arg_default_audio_device: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-a".to_string()),
                ("audio_device".to_string(), "hw:3,0".to_string()),
            ]);
            let arg_default_audio_sampling_rate: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-r".to_string()),
                ("audio_sampling_rate".to_string(), "44000".to_string()),
            ]);
            let arg_default_model: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-m".to_string()),
                ("model".to_string(), "medium".to_string()),
            ]);
            let arg_default_task: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-t".to_string()),
                ("task".to_string(), "transcribe".to_string()),
            ]);
            let arg_default_output_format: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-of".to_string()),
                ("output_format".to_string(), "string".to_string()),
            ]);
            let arg_default_output_terminator: HashMap<String, String> = HashMap::from([
                ("arg".to_string(), "-ot".to_string()),
                ("output_terminator".to_string(), "none".to_string()),
            ]);

            let audio_player_manager = AudioPlayerManager::new(true);

            let app_state = AppState {
                smart_action_manager: SmartActionManager::new(
                    app.handle().clone(),
                    app_config,
                    menu_manager.clone(),
                    tray_icon_manager.clone(),
                    audio_player_manager.clone(),
                    SmartAction {
                        value: String::from("dictate_text"),
                        name: String::from("Dictate Text"),
                        description: String::from("Record an audio and convert it to text."),
                        args: vec![
                            arg_default_audio_device,
                            arg_default_audio_sampling_rate,
                            arg_default_model,
                            arg_default_task,
                            arg_default_output_format,
                            arg_default_output_terminator,
                        ],
                    },
                ),
                tray_icon_manager: Mutex::new(tray_icon_manager.clone()),
            };

            app.manage(app_state);

            Ok(())
        })
        // .manage(app_state)
        // .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ui_notify_change_action,
            ui_notify_startup,
            ui_request_execute_action,
            ui_request_stop_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
