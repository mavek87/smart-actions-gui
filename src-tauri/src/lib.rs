mod commands;
mod domain;
mod logic;

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
    ui_request_execute_action::ui_request_execute_action, ui_request_stop_action::ui_request_stop_action,
};

use domain::app_state::AppState;

use crate::domain::smart_action::SmartAction;
use crate::logic::smart_action_manager::SmartActionManager;
use logic::config_manager::ConfigManager;
use logic::menu_manager::MenuManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_manager: ConfigManager = ConfigManager::new();

    let app_config = config_manager
        .read_config("assets/config.json".to_string())
        .unwrap();
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

            // TODO: use the current language if present
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
                .name(Some("smart-actions-gui"))
                .version(Some("0.1.0"))
                .website_label(Some("Github Repository"))
                .website(Some("https://github.com/mavek87/smart-actions-gui"))
                .authors(Some(vec![String::from("Matteo Veroni")]))
                .build();

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

            let app_state = AppState {
                smart_action_manager: SmartActionManager::new(
                    app.handle().clone(),
                    app_config,
                    menu_manager.clone(),
                    // TODO: bug. if ui doesnt call change select probably, it doesnt work (uses the empty smart action)
                    // this is not easy to fix because at the moment in the view the first smart action set is random... check why
                    SmartAction {
                        value: String::from("empty_smart_action_value"),
                        name: String::from("empty_smart_action_name"),
                        description: String::from("empty_smart_action_description"),
                        args: vec![],
                    },
                ),
                menu_manager: Mutex::new(menu_manager.clone()),
            };

            app.manage(app_state);

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
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
                        lang_sub_item_it
                            .set_checked(event.id().0.as_str() == "it")
                            .expect("Change check error");
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(true)
                .build(app)?;
            Ok(())
        })
        // .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ui_notify_change_action,
            ui_notify_startup,
            ui_request_execute_action,
            ui_request_stop_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use tauri::GlobalShortcutManager;
//
// // Registriamo la scorciatoia CTRL + U
// app_handle.global_shortcut_manager().register("CTRL + U", move || {
// // Apre una finestra di dialogo con un messaggio
// tauri::api::dialog::message(
// Some(&app_handle),
// "Scorciatoia premuta",
// "Hai premuto CTRL + U!",
// );
// });
