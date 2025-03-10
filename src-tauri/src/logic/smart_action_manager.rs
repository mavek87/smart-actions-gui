use crate::domain::app_config::AppConfig;
use crate::domain::smart_action::{SmartAction, SmartActionState, SmartActionStatus};
use crate::logic::audio_player_manager::AudioPlayerManager;
use crate::logic::menu_manager::MenuManager;
use crate::logic::tray_icon_manager::TrayIconManager;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct SmartActionManager {
    app_handle: AppHandle,
    app_config: AppConfig,
    menu_manager: Mutex<MenuManager>,
    tray_icon_manager: Mutex<TrayIconManager>,
    audio_player_manager: Mutex<AudioPlayerManager>,
    smart_action_state: Mutex<SmartActionState>,
    // process_start: Mutex<Option<Child>>,
    process_stop: Mutex<Option<Child>>,
}

impl SmartActionManager {
    pub fn new(
        app_handle: AppHandle,
        app_config: AppConfig,
        menu_manager: MenuManager,
        tray_icon_manager: TrayIconManager,
        audio_player_manager: AudioPlayerManager,
        smart_action: SmartAction,
    ) -> Self {
        SmartActionManager {
            app_handle,
            app_config,
            menu_manager: Mutex::new(menu_manager),
            tray_icon_manager: Mutex::new(tray_icon_manager),
            audio_player_manager: Mutex::new(audio_player_manager),
            smart_action_state: Mutex::new(SmartActionState::new(smart_action)),
            // process_start: Mutex::new(None::<Child>),
            process_stop: Mutex::new(None::<Child>),
        }
    }

    pub fn change_current_smart_action(&self, new_smart_action: SmartAction) {
        let mut current_smart_action = self.smart_action_state.lock().unwrap();
        *current_smart_action = SmartActionState::new(new_smart_action.clone());

        let action_name = format!("{}", new_smart_action.name);
        self.menu_manager
            .lock()
            .unwrap()
            .set_action_name_text(action_name);
    }

    // TODO: handle errors
    pub fn start_current_smart_action(&self) {
        // let (tx, rx) = mpsc::channel();

        // TODO: unlock if error occurs
        self.menu_manager.lock().unwrap().set_action_started();
        self.tray_icon_manager.lock().unwrap().set_recording_icon();

        let smart_action_state = self.smart_action_state.lock().unwrap();

        let current_smart_action_value = smart_action_state.value.lock().unwrap();
        let current_smart_action_args = smart_action_state.args.lock().unwrap();

        let mut audio_player_manager = self.audio_player_manager.lock().unwrap();
        audio_player_manager
            .play_sound_for_smart_action(current_smart_action_value.clone(), SmartActionStatus::ON);

        // if self.process_start.lock().unwrap().is_none() {
        let mut command_smart_action = Command::new("bash");

        command_smart_action
            .arg(format!(
                "{}/smart-actions.sh",
                self.app_config.faster_whisper_folder
            ))
            .arg(format!("{}", current_smart_action_value));

        // TODO: a refactoring is necessary
        for arg in current_smart_action_args.iter() {
            let mut arg_param: String = String::new();
            let mut arg_value: String = String::new();

            for arg_key in arg.keys() {
                if let Some(value) = arg.get(arg_key) {
                    if arg_key == "arg" {
                        arg_param = value.to_string(); // -l
                    } else {
                        arg_value = value.to_string(); // it
                    }
                }
            }

            // TODO: what to do if value is empty?
            if !arg_value.is_empty() {
                command_smart_action.arg(arg_param);
                command_smart_action.arg(arg_value);
            }
        }

        let process_command_smart_action = command_smart_action
            .spawn()
            .expect("Failed to start 'dictate_text' action from smart-actions.sh"); // TODO: remove hardcoded value

        let arc_mutex_app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
        let arc_mutex_process_command_smart_action =
            Arc::new(Mutex::new(process_command_smart_action));
        let arc_mutex_tray_icon_manager =
            Arc::new(Mutex::new(self.tray_icon_manager.lock().unwrap().clone()));
        let arc_mutex_audio_player_manager = Arc::new(Mutex::new(audio_player_manager.clone()));
        let arc_mutex_current_smart_action_value =
            Arc::new(Mutex::new(current_smart_action_value.clone()));

        thread::spawn(move || {
            let app_handle = arc_mutex_app_handle.lock().unwrap();

            let status = arc_mutex_process_command_smart_action
                .lock()
                .unwrap()
                .wait()
                .expect("Errore nel wait del processo");

            if status.success() {
                println!("Il processo è terminato con successo!");
                app_handle
                    .emit("smart_action_waiting_stop", "Stop waiting...")
                    .unwrap();
            } else if let Some(code) = status.code() {
                println!("Il processo è terminato con codice di errore: {}", code);
                app_handle
                    .emit("smart_action_waiting_error", "Error during waiting...")
                    .unwrap();
            } else {
                println!("Il processo è terminato in modo anomalo.");
                app_handle
                    .emit("smart_action_waiting_error", "Error during waiting...")
                    .unwrap();
            }

            arc_mutex_tray_icon_manager
                .lock()
                .unwrap()
                .set_default_icon();

            let current_smart_action_value = arc_mutex_current_smart_action_value.lock().unwrap();
            arc_mutex_audio_player_manager
                .lock()
                .unwrap()
                .play_sound_for_smart_action(
                    current_smart_action_value.clone(),
                    SmartActionStatus::OFF,
                );
        });

        // let id = process_command_smart_action.id();
        // println!("Process ID: {}", id);

        // *self.process_start.lock().unwrap() = Some(child_arc.lock().unwrap());

        self.app_handle
            .emit("smart_action_recording_start", "Start recording...")
            .unwrap();

        println!("Recording started!");
        // } else {
        //     println!("Recording is already running.");
        // }
    }

    pub fn stop_current_smart_action(&self) {
        // TODO: unlock if error occurs (???)
        self.menu_manager.lock().unwrap().set_action_stopped();
        self.tray_icon_manager.lock().unwrap().set_waiting_icon();

        self.app_handle
            .emit("smart_action_waiting_start", "Waiting response...")
            .unwrap();

        // Gestione del processo di registrazione
        let mut process_stop = self.process_stop.lock().unwrap();
        if process_stop.is_none() {
            let child = Command::new("bash")
                .arg(format!(
                    "{}/smart-actions.sh",
                    self.app_config.faster_whisper_folder
                ))
                .arg("end")
                .spawn()
                .expect("Failed to start 'end' action from smart-actions.sh");
            *process_stop = Some(child);

            // Aspettiamo che il processo STOP termini
            if let Some(mut child) = process_stop.take() {
                if let Err(err) = child.wait() {
                    eprintln!("Error while waiting for process termination: {}", err);
                    // self.app_handle
                    //     .emit("smart_action_waiting_error", "Error during waiting...")
                    //     .unwrap();
                } else {
                    // self.app_handle
                    //     .emit("smart_action_waiting_stop", "Stop waiting...")
                    //     .unwrap();
                }
            }

            // let mut process_start = self.process_start.lock().unwrap();
            // *process_start = None;
            *process_stop = None;

            println!("Recording stop!");
        } else {
            println!("Recording already stopping.");
        }
    }
}