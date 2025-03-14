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

    pub fn change_with_next_smart_action(&self) {
        self.app_handle
            .emit("request_to_ui_next_smart_action", "")
            .expect("Failed to emit request_to_ui_next_smart_action");
    }

    pub fn change_with_previous_smart_action(&self) {
        self.app_handle
            .emit("request_to_ui_previous_smart_action", "")
            .expect("Failed to emit request_to_ui_previous_smart_action");
    }

    pub fn change_current_smart_action(&self, new_smart_action: SmartAction) {
        let smart_action_name = &new_smart_action.name;

        self.menu_manager
            .lock()
            .unwrap()
            .set_action_name_text(&smart_action_name);

        let mut current_smart_action = self.smart_action_state.lock().unwrap();
        *current_smart_action = SmartActionState::new(new_smart_action);
    }

    // TODO: handle errors
    pub fn start_current_smart_action(&self) {
        // TODO: check if the smart action status consent to start
        self.tray_icon_manager.lock().unwrap().show_default_icon();
        self.menu_manager.lock().unwrap().set_action_started();

        let smart_action_state = self.smart_action_state.lock().unwrap();
        let current_smart_action_value = smart_action_state.value.lock().unwrap();
        let current_smart_action_args = smart_action_state.args.lock().unwrap();

        // TODO: change the internal state to recording????
        // self.smart_action_state.lock().unwrap().status =
        //     Arc::new(Mutex::new(SmartActionStatus::RECORDING));

        self.audio_player_manager
            .lock()
            .unwrap()
            .play_sound_for_smart_action(
                &current_smart_action_value,
                Some(SmartActionStatus::RECORDING),
            ); // TODO: it depends can be recording or not...

        // if self.process_start.lock().unwrap().is_none() {
        let mut command_smart_action = Command::new("bash");

        command_smart_action
            .arg(format!(
                "{}/smart-actions.sh",
                self.app_config.smart_actions_folder
            ))
            .arg(format!("{}", &current_smart_action_value));

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

            if !arg_value.is_empty() {
                command_smart_action.arg(arg_param);
                command_smart_action.arg(arg_value);
            }
        }

        let process_command_smart_action = command_smart_action.spawn().expect(&format!(
            "Failed to start '{} smart action",
            current_smart_action_value
        ));

        let arc_mutex_app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
        let arc_mutex_process_command_smart_action =
            Arc::new(Mutex::new(process_command_smart_action));
        let arc_mutex_tray_icon_manager =
            Arc::new(Mutex::new(self.tray_icon_manager.lock().unwrap().clone()));
        let arc_mutex_audio_player_manager = Arc::new(Mutex::new(
            self.audio_player_manager.lock().unwrap().clone(),
        ));
        let arc_mutex_current_smart_action_value =
            Arc::new(Mutex::new(current_smart_action_value.clone()));

        thread::spawn(move || {
            let exit_status = arc_mutex_process_command_smart_action
                .lock()
                .unwrap()
                .wait()
                .expect("Error during process wait");

            let app_handle_guard = arc_mutex_app_handle.lock().unwrap();
            let smart_action_status;
            if exit_status.success() {
                println!("The process is end with success!");

                smart_action_status = SmartActionStatus::COMPLETED;

                if let Err(e) =
                    app_handle_guard.emit("smart_action_waiting_stop", "Stop waiting...")
                {
                    eprintln!("Error during emission: {}", e);
                    drop(app_handle_guard);
                }
            } else if let Some(code) = exit_status.code() {
                eprintln!("The process is end with error status code: {}", code);

                smart_action_status = SmartActionStatus::FAILED;

                if let Err(e) =
                    app_handle_guard.emit("smart_action_waiting_error", "Error during waiting...")
                {
                    eprintln!("Error during emission: {}", e);
                    drop(app_handle_guard);
                }
            } else {
                eprintln!("The process is end anomaly");

                smart_action_status = SmartActionStatus::FAILED;

                if let Err(e) =
                    app_handle_guard.emit("smart_action_waiting_error", "Error during waiting...")
                {
                    eprintln!("Error during emission: {}", e);
                    drop(app_handle_guard);
                }
            }

            arc_mutex_tray_icon_manager
                .lock()
                .unwrap()
                .show_default_icon();

            let current_smart_action_value = arc_mutex_current_smart_action_value.lock().unwrap();

            arc_mutex_audio_player_manager
                .lock()
                .unwrap()
                .play_sound_for_smart_action(
                    &current_smart_action_value,
                    Some(smart_action_status),
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
        let current_smart_action_state = self.smart_action_state.lock().unwrap();
        let current_smart_action_value = current_smart_action_state.value.lock().unwrap();
        // let smart_action_status = current_smart_action_state.status.lock().unwrap();

        // TODO: this is very complicated to do (handle the state is very hard!)
        // if *smart_action_status != SmartActionStatus::RECORDING {
        //     println!(
        //         "Current smart action status is {} so it cannot be stopped",
        //         smart_action_status
        //     );
        //     return;
        // }

        self.menu_manager.lock().unwrap().set_action_stopped(); // TODO: unlock if error occurs (???)

        self.tray_icon_manager.lock().unwrap().show_waiting_icon();

        self.audio_player_manager
            .lock()
            .unwrap()
            .play_sound_for_smart_action(
                &current_smart_action_value,
                Some(SmartActionStatus::WAITING),
            );

        self.app_handle
            .emit("smart_action_waiting_start", "Waiting response...")
            .unwrap();

        // Gestione del processo di registrazione
        let mut process_stop = self.process_stop.lock().unwrap();
        if process_stop.is_none() {
            let child = Command::new("bash")
                .arg(format!(
                    "{}/smart-actions.sh",
                    self.app_config.smart_actions_folder
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
