use tauri::{
    menu::{AboutMetadataBuilder, MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, State, Wry,
};

use std::{
    fs::File,
    io::Read,
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use serde::Deserialize;

struct AppState {
    current_action_name: Mutex<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    faster_whisper_folder: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn notify_change_action(name: &str, state: State<AppState>, app: AppHandle) -> String {
    let mut current_action_name = state.current_action_name.lock().unwrap();

    *current_action_name = name.to_string();

    // let icon = app.tray_by_id("action_state_item").unwrap();
    // icon.set_menu();
    //
    // // Ottenere la finestra principale
    // if let Some(main_window) = app.get_window("main") {
    //     let menu_handle = main_window.menu_handle();
    //
    //     // Cambiare il testo del MenuItem con ID "action_state_item"
    //     if let Ok(menu_item) = menu_handle.get_item("action_state_item") {
    //         let _ = menu_item.set_title(name.to_string());
    //     }
    // }

    // let Some(root) = app.menu() else {
    //     return Ok(());
    // };
    //
    // // This removes a menu item from the macOS "app menu" (the leftmost one)
    // if let Some(app_menu) = find(&root, "MyAppName") {
    //     if let Some(submenu) = app_menu.get("the_submenu_id") {
    //         app_menu.remove(&submenu)?;
    //     }
    // }

    // let menu = app.menu().unwrap();
    // let kind = menu.get("action_state_item").unwrap();
    // let x = kind.as_menuitem().unwrap();
    // x.set_text(current_action_name.to_string());

    println!("current action name: {}", current_action_name);

    current_action_name.to_string()
}

// fn change_tray_text(app: tauri::AppHandle, new_text: String) {
//     let tray_handle = app.tray_handle();
//     let item_handle = tray_handle.get_item("toggle_status");
//
//     let _ = item_handle.set_title(new_text);
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
    let config: Config = serde_json::from_str(&content).expect("Failed to parse JSON");

    println!("config: {:?}", config);

    tauri::Builder::default()
        .setup(|app| {
            let process_start = Arc::new(Mutex::new(None::<Child>));
            let process_stop = Arc::new(Mutex::new(None::<Child>));

            let action_state_item = MenuItemBuilder::new("AAA")
                .id("action_state_item")
                .enabled(false)
                .build(app)?;

            let start_record_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "start_record",
                "Start record",
                true,
                Some("Ctrl+N"),
            )?));

            let stop_record_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "stop_record",
                "Stop record",
                false,
                Some("Ctrl+E"),
            )?));

            let quit_item = MenuItemBuilder::new("Quit")
                .id("quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)?;

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

            let menu = MenuBuilder::new(app)
                .item(&action_state_item)
                .separator()
                .items(&[
                    &*start_record_menu_item.lock().unwrap(),
                    &*stop_record_menu_item.lock().unwrap(),
                ])
                .separator()
                .item(&help_submenu)
                .separator()
                .item(&quit_item)
                .build()?;

            let start_record_menu_item_clone = Arc::clone(&start_record_menu_item);
            let stop_record_menu_item_clone = Arc::clone(&stop_record_menu_item);
            let process_start_clone = Arc::clone(&process_start);
            let process_stop_clone = Arc::clone(&process_stop);

            // let tray = TrayIconBuilder::new()
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "start_record" => {
                        println!("start record was clicked");

                        switch_menu_items_states(
                            &start_record_menu_item_clone,
                            &stop_record_menu_item_clone,
                            true,
                        );

                        let mut process_start = process_start.lock().unwrap();

                        let app_state: State<AppState> = app.state();
                        let current_action_name = app_state.current_action_name.lock().unwrap();

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
                    "stop_record" => {
                        println!("stop record was clicked");

                        switch_menu_items_states(
                            &start_record_menu_item_clone.clone(),
                            &stop_record_menu_item_clone.clone(),
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
        .manage(AppState {
            current_action_name: Mutex::new("dictate_text".to_string()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![notify_change_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn switch_menu_items_states(
    start_record_menu_item: &Arc<Mutex<MenuItem<Wry>>>,
    stop_record_menu_item: &Arc<Mutex<MenuItem<Wry>>>,
    is_start_recording: bool,
) {
    start_record_menu_item
        .lock()
        .unwrap()
        .set_enabled(!is_start_recording)
        .unwrap(); // Disabilita Stop
    stop_record_menu_item
        .lock()
        .unwrap()
        .set_enabled(is_start_recording)
        .unwrap(); // Abilita Start
}
