use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, State, Wry,
};

use std::{
    process::{Child, Command},
    sync::{Arc, Mutex},
    fs::File,
    io::Read
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
fn notify_change_action(name: &str, state: State<AppState>) -> String {
    let mut current_action_name = state.current_action_name.lock().unwrap();

    *current_action_name = name.to_string();

    println!("current action name: {}", current_action_name);

    current_action_name.to_string()
}

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

            let action_state_item = MenuItem::with_id(app, "action_state_item", "AAA", true, None::<&str>)?;
            action_state_item.set_enabled(false);

            let start_record_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "start_record",
                "Start record",
                true,
                None::<&str>,
            )?));
            let stop_record_menu_item = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "stop_record",
                "Stop record",
                false,
                None::<&str>,
            )?));
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &action_state_item,
                    &*start_record_menu_item.lock().unwrap(),
                    &*stop_record_menu_item.lock().unwrap(),
                    &quit_item,
                ],
            )?;

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
            current_action_name: Mutex::new("dictate_text".to_string())
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
