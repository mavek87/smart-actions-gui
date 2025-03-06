use tauri::{
    menu::{
        AboutMetadataBuilder, CheckMenuItemBuilder, Menu, MenuBuilder, MenuItem, MenuItemBuilder,
        SubmenuBuilder,
    },
    tray::TrayIconBuilder,
    AppHandle, Manager, State, Wry,
};

use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::{
    fs::File,
    io::Read,
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use domain::{AppState, AppConfig, ActionConfig, ActionsMetadata};

pub mod domain;
pub mod commands;

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

#[tauri::command]
fn notify_ui_startup() -> String {
    let action_names: [&str; 3] = ["dictate_text", "ai_reply_text", "audio_to_text"];

    let mut actions_metadata = ActionsMetadata::new();

    for action_name in &action_names {
        // TODO 1: find a way to use config
        let action_output = Command::new("bash")
            .arg("/opt/FasterWhisper/smart-actions.sh") // Nessun bisogno di `format!()`
            .arg(action_name)
            .arg("--print-config")
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                let mut stdout = String::new();
                if let Some(ref mut out) = child.stdout {
                    out.read_to_string(&mut stdout).ok();
                }
                Ok(stdout)
            });

        let action_config_raw_output = action_output.unwrap_or_else(|e| {
            eprintln!("Errore durante l'esecuzione del comando: {}", e);
            "".to_string()
        });

        let action_config = ActionConfig::parse_from_string(&action_config_raw_output);

        println!("{:#?}", action_config);

        actions_metadata
            .actions
            .insert(action_name.to_string(), action_config);
    }

    let json_actions_metadata =
        serde_json::to_string(&actions_metadata).expect("Failed to parse JSON");

    println!("JSON delle azioni: {}", json_actions_metadata);

    json_actions_metadata
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn notify_change_action(
    value: &str,
    name: &str,
    state: State<AppState>,
    _app: AppHandle,
) -> String {
    println!("value: {}", value);
    println!("name: {}", name);

    let menu_handle = state.menu_handle.lock().unwrap();
    menu_handle
        .get("action_state_item")
        .unwrap()
        .as_menuitem()
        .unwrap()
        .set_text(format!("{}", name))
        .unwrap();

    let mut current_action_value = state.current_action_value.lock().unwrap();

    *current_action_value = value.to_string();
    println!("current_action_value: {}", value);

    // let menu = app.menu().unwrap();
    // let kind = menu.get("action_state_item").unwrap();
    // kind.as_menuitem().unwrap().set_text(format!("{}", name)).unwrap();

    current_action_value.to_string()
}

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

    /////////////////////////77

    println!("config: {:?}", config);

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

            let start_menu_item_clone = Arc::clone(&start_menu_item);
            let stop_menu_item_clone = Arc::clone(&stop_menu_item);

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                //TODO: unify the two on_menu_event
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "start" => {
                        println!("start record was clicked");

                        switch_menu_items_states(
                            &start_menu_item_clone,
                            &stop_menu_item_clone,
                            true,
                        );

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

                        switch_menu_items_states(
                            &start_menu_item_clone.clone(),
                            &stop_menu_item_clone.clone(),
                            false,
                        );

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
            notify_change_action,
            notify_ui_startup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn switch_menu_items_states(
    start_menu_item: &Arc<Mutex<MenuItem<Wry>>>,
    stop_menu_item: &Arc<Mutex<MenuItem<Wry>>>,
    is_start_recording: bool,
) {
    start_menu_item
        .lock()
        .unwrap()
        .set_enabled(!is_start_recording)
        .unwrap(); // Disabilita Stop
    stop_menu_item
        .lock()
        .unwrap()
        .set_enabled(is_start_recording)
        .unwrap(); // Abilita Start
}
