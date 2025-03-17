mod commands;
mod domain;
mod logic;

use std::collections::HashMap;
use std::string::ToString;
use std::sync::Mutex;
use sys_locale::get_locale;
use tauri::{
    menu::{
        AboutMetadataBuilder, CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder,
    },
    tray::TrayIconBuilder,
    Emitter, Manager, State,
};

use commands::{
    ui_notify_change_action::ui_notify_change_action, ui_notify_startup::ui_notify_startup,
    ui_request_execute_action::ui_request_execute_action,
    ui_request_stop_action::ui_request_stop_action,
};

use domain::{app_state::AppState, smart_action::SmartAction};

use crate::commands::ui_notify_change_element_in_action::ui_notify_change_element_in_action;
use crate::domain::constants::{
    APP_NAME, APP_VERSION, AUTHORS, DEFAULT_CONFIG_FILE,
    EVENT_TO_UI_CHANGE_CURRENT_LANGUAGE_ACTION, WEBSITE, WEBSITE_LABEL,
};
use crate::domain::language::Language;
use logic::{
    audio_player_manager::AudioPlayerManager, config_manager::ConfigManager,
    language_manager::LanguageManager, menu_manager::MenuManager,
    smart_action_manager::SmartActionManager, tray_icon_manager::TrayIconManager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_manager: ConfigManager = ConfigManager::new();

    let app_config = config_manager
        .read_config(DEFAULT_CONFIG_FILE)
        .expect(&format!(
            "Error reading config file {}",
            DEFAULT_CONFIG_FILE
        ));

    println!("app_config: {:?}", app_config);

    tauri::Builder::default()
        .setup(|app| {
            let lang_code = get_locale()
                .map(|s| s.split('-').next().unwrap_or("unset").to_string())
                .unwrap_or_else(|| "unset".to_string());

            println!("System language detected: {}", &lang_code);

            let mut language_manager =
                LanguageManager::new(Language::from_str(&lang_code).unwrap_or(Language::UNSET));

            let action_name_menu_item = MenuItemBuilder::new("Dictate Text")
                .id("action_name_menu_item")
                .enabled(false)
                .build(app)?;

            let start_action_menu_item = MenuItemBuilder::new("Start")
                .id("start")
                .enabled(true)
                .build(app)?;

            let stop_action_menu_item = MenuItemBuilder::new("Stop")
                .id("stop")
                .enabled(false)
                .build(app)?;

            let stop_vocal_audio_item = MenuItemBuilder::new("Stop vocal audio")
                .id("stop_vocal_audio")
                .enabled(false)
                .build(app)?;

            let lang_str_code = language_manager.get_current_language().code();

            // https://v2.tauri.app/learn/window-menu/
            let lang_sub_menu_item_unset = CheckMenuItemBuilder::with_id("unset", "Unset")
                .checked(lang_str_code == "")
                .build(app)?;

            let lang_sub_menu_item_en = CheckMenuItemBuilder::with_id("en", "English")
                .checked(lang_str_code == "en")
                .build(app)?;

            let lang_sub_menu_item_it = CheckMenuItemBuilder::with_id("it", "Italian")
                .checked(lang_str_code == "it")
                .build(app)?;

            let language_submenu = SubmenuBuilder::new(app, "Language")
                .item(&lang_sub_menu_item_unset)
                .item(&lang_sub_menu_item_en)
                .item(&lang_sub_menu_item_it)
                .build()?;

            let audio_sub_menu_item_enabled =
                CheckMenuItemBuilder::with_id("audio_enabled", "Enabled")
                    .checked(true)
                    .build(app)?;

            let audio_submenu = SubmenuBuilder::new(app, "Audio")
                .item(&audio_sub_menu_item_enabled)
                .build()?;

            let help_sub_menu_item_about_metadata = AboutMetadataBuilder::new()
                .name(Some(APP_NAME.to_string()))
                .version(Some(APP_VERSION.to_string()))
                .website_label(Some(WEBSITE_LABEL.to_string()))
                .website(Some(WEBSITE.to_string()))
                .authors(Some(AUTHORS.iter().map(|&s| s.to_string()).collect()))
                .build();

            let help_sub_menu_item_try_to_fix_issues = MenuItemBuilder::new("Try to fix issues")
                .id("try_to_fix_issues")
                .enabled(true)
                .build(app)?;

            let help_submenu = SubmenuBuilder::new(app, "Help")
                .item(&help_sub_menu_item_try_to_fix_issues)
                .about(Some(help_sub_menu_item_about_metadata))
                .build()?;

            let quit_item = MenuItemBuilder::new("Quit")
                .id("quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&action_name_menu_item)
                .separator()
                .items(&[
                    &start_action_menu_item,
                    &stop_action_menu_item,
                    &stop_vocal_audio_item,
                ])
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
                stop_vocal_audio_item,
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
                    "stop_vocal_audio" => {
                        println!("stop vocal audio was clicked");
                        let app_state: State<AppState> = app.state();
                        app_state.smart_action_manager.stop_vocal_audio();
                    }
                    "try_to_fix_issues" => {
                        println!("try to fix issues menu item was clicked");
                        let app_state: State<AppState> = app.state();
                        app_state.smart_action_manager.try_to_fix_issues();
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "unset" | "en" | "it" => {
                        let selected_language = event.id().0.as_str();

                        lang_sub_menu_item_unset
                            .set_checked(selected_language == "unset")
                            .expect("Change check error");
                        lang_sub_menu_item_en
                            .set_checked(selected_language == "en")
                            .expect("Change check error");
                        lang_sub_menu_item_it
                            .set_checked(selected_language == "it")
                            .expect("Change check error");

                        let app_state: State<AppState> = app.state();

                        app_state
                            .language_manager
                            .set_current_language_as_str(&selected_language);

                        if let Err(e) = app.emit(
                            &EVENT_TO_UI_CHANGE_CURRENT_LANGUAGE_ACTION,
                            app_state.language_manager.get_current_language().code(),
                        ) {
                            eprintln!("Error during emission: {}", e);
                        }
                    }
                    "audio_enabled" => match audio_sub_menu_item_enabled.is_checked() {
                        Ok(is_checked) => {
                            audio_sub_menu_item_enabled
                                .set_checked(is_checked)
                                .expect("Change check error");
                            let app_state: State<AppState> = app.state();
                            app_state.smart_action_manager.set_audio_enable(is_checked);
                        }
                        Err(e) => {
                            eprintln!("Error updating audio_enabled: {}", e);
                        }
                    },
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(true)
                .build(app)?;

            let tray_icon_manager = TrayIconManager::new(tray_icon);
            tray_icon_manager.show_default_icon();

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["alt+s", "alt+a", "alt+n", "alt+b", "alt+h"])?
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
                                    app_state
                                        .smart_action_manager
                                        .change_with_next_smart_action();
                                } else if shortcut.matches(Modifiers::ALT, Code::KeyB) {
                                    println!("ALT+B Pressed! - change with previous smart action");
                                    let app_state: State<AppState> = app.state();
                                    app_state
                                        .smart_action_manager
                                        .change_with_previous_smart_action();
                                } else if shortcut.matches(Modifiers::ALT, Code::KeyH) {
                                    println!("ALT+H Pressed! - try to fix issues");
                                    let app_state: State<AppState> = app.state();
                                    app_state.smart_action_manager.try_to_fix_issues();
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

            let default_smart_action = SmartAction {
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
            };

            let audio_player_manager = AudioPlayerManager::new(app_config.clone(), true);

            let app_state = AppState {
                smart_action_manager: SmartActionManager::new(
                    app.handle().clone(),
                    app_config,
                    menu_manager,
                    tray_icon_manager.clone(),
                    audio_player_manager.clone(),
                    default_smart_action,
                ),
                tray_icon_manager: Mutex::new(tray_icon_manager),
                config_manager: Mutex::new(config_manager),
                audio_player_manager: Mutex::new(audio_player_manager),
                language_manager,
            };

            app.manage(app_state);

            Ok(())
        })
        // .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            ui_notify_change_action,
            ui_notify_change_element_in_action,
            ui_notify_startup,
            ui_request_execute_action,
            ui_request_stop_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
