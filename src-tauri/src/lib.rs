mod commands;
mod domain;
mod menu_action_state_manager;

use tauri::{
    menu::{
        AboutMetadataBuilder, CheckMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder,
        SubmenuBuilder,
    },
    tray::TrayIconBuilder,
    Manager, State,
};

use std::{
    fs::File,
    io::Read,
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use commands::{ui_notify_change_action, ui_notify_startup, ui_request_execute_action};
use domain::{AppConfig, AppState};

use menu_action_state_manager::MenuActionStateManager;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // TODO: refactor extract method

    // Percorso del file JSON
    let file_config_path = "assets/config.json";

    // Apri il file
    let mut file_config = File::open(file_config_path).expect("Failed to open file");

    // Leggi il contenuto del file in una stringa
    let mut content = String::new();
    file_config
        .read_to_string(&mut content)
        .expect("Failed to read file");

    // Parse del JSON nella struct `Config`
    let config: AppConfig = serde_json::from_str(&content).expect("Failed to parse JSON");

    // println!("config: {:?}", config);
    /////////////////////////

    tauri::Builder::default()
        .setup(|app| {
            let process_start = Arc::new(Mutex::new(None::<Child>));
            let process_stop = Arc::new(Mutex::new(None::<Child>));

            let action_state_item = MenuItemBuilder::new("Dictate Text")
                .id("action_state_item")
                .enabled(false)
                .build(app)?;

            let start_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "start",
                "Start",
                true,
                Some("Ctrl+N"),
            )?));

            let stop_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "stop",
                "Stop",
                false,
                Some("Ctrl+E"),
            )?));

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
                .item(&action_state_item)
                .separator()
                .items(&[
                    &*start_menu_item.lock().unwrap(),
                    &*stop_menu_item.lock().unwrap(),
                ])
                .separator()
                .item(&language_submenu)
                .separator()
                .item(&help_submenu)
                .separator()
                .item(&quit_item)
                .build()?;

            let menu_action_state_manager = Arc::new(Mutex::new(MenuActionStateManager::new(
                Arc::clone(&start_menu_item),
                Arc::clone(&stop_menu_item),
            )));

            // TODO: add the menu state manager in the app_state
            let app_state = AppState {
                current_action_value: Mutex::new("dictate_text".to_string()),
                menu_handle: Mutex::new(menu.clone()),
            };

            app.manage(app_state);
            app.on_menu_event(move |_app, event| match event.id().0.as_str() {
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
                    println!("unexpected menu event");
                }
            });

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                //TODO: unify the two on_menu_event
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "start" => {
                        println!("start record was clicked");

                        menu_action_state_manager
                            .lock()
                            .unwrap()
                            .set_action_started();

                        let mut process_start = process_start.lock().unwrap();

                        let app_state: State<AppState> = app.state();
                        let current_action_name = app_state.current_action_value.lock().unwrap();

                        if process_start.is_none() {
                            let child = Command::new("bash")
                                .arg(format!("{}/smart-actions.sh", config.faster_whisper_folder))
                                .arg(format!("{}", current_action_name))
                                .spawn()
                                .expect(
                                    "Failed to start 'dictate_text' action from smart-actions.sh",
                                );
                            *process_start = Some(child);
                            println!("Recording started!");
                        } else {
                            println!("Recording is already running.");
                        }
                    }
                    "stop" => {
                        println!("stop record was clicked");

                        menu_action_state_manager
                            .lock()
                            .unwrap()
                            .set_action_stopped();

                        // Gestione del processo di registrazione
                        let mut process_stop = process_stop.lock().unwrap();
                        if process_stop.is_none() {
                            let child = Command::new("bash")
                                .arg(format!("{}/smart-actions.sh", config.faster_whisper_folder))
                                .arg("end")
                                .spawn()
                                .expect("Failed to start 'end' action from smart-actions.sh");
                            *process_stop = Some(child);

                            // Aspettiamo che il processo STOP termini
                            if let Some(mut child) = process_stop.take() {
                                if let Err(err) = child.wait() {
                                    eprintln!(
                                        "Error while waiting for process termination: {}",
                                        err
                                    );
                                }
                            }

                            let mut process_start = process_start.lock().unwrap();
                            *process_start = None;
                            *process_stop = None;

                            println!("Recording stop!");
                        } else {
                            println!("Recording already stopping.");
                        }
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
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
            ui_request_execute_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
