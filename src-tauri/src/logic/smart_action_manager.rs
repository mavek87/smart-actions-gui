use crate::domain::app_config::AppConfig;
use crate::domain::smart_action::{SmartAction, SmartActionState};
use crate::logic::menu_manager::MenuManager;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct SmartActionManager {
    app_handle: AppHandle,
    app_config: AppConfig,
    menu_manager: Mutex<MenuManager>,
    smart_action_state: Mutex<SmartActionState>,
    // process_start: Mutex<Option<Child>>,
    process_stop: Mutex<Option<Child>>,
}

impl SmartActionManager {
    pub fn new(
        app_handle: AppHandle,
        app_config: AppConfig,
        menu_manager: MenuManager,
        smart_action: SmartAction,
    ) -> Self {
        SmartActionManager {
            app_handle,
            app_config,
            menu_manager: Mutex::new(menu_manager),
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

        let smart_action_state = self.smart_action_state.lock().unwrap();

        let current_smart_action_value = smart_action_state.value.lock().unwrap();
        let current_smart_action_args = smart_action_state.args.lock().unwrap();

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

        let child_arc = Arc::new(Mutex::new(process_command_smart_action));
        let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));

        // Monitoriamo l'uscita del processo in un thread separato
        thread::spawn(move || {
            let app_handle = app_handle.lock().unwrap();

            // app_handle.emit("process-success", {}).unwrap();

            let status = child_arc
                .lock()
                .unwrap()
                .wait()
                .expect("Errore nel wait del processo");

            if status.success() {
                println!("Il processo è terminato con successo!");
                app_handle.emit("smart_action_waiting_stop", "Stop waiting...").unwrap();
            } else if let Some(code) = status.code() {
                println!("Il processo è terminato con codice di errore: {}", code);
                // app_handle.emit("process-failed", code).unwrap();
                app_handle.emit("smart_action_waiting_error", "Error during waiting...").unwrap();
            } else {
                println!("Il processo è terminato in modo anomalo.");
                // app_handle.emit("process-failed", "unknown").unwrap();
                app_handle.emit("smart_action_waiting_error", "Error during waiting...").unwrap();
            }
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

    // #[tauri::command]
    // fn run_child_process(app_handle: tauri::AppHandle) {
    //     let (tx, rx) = mpsc::channel();
    //
    //     thread::spawn(move || {
    //         let status: ExitStatus = Command::new("ls") // Cambia con il tuo comando
    //             .arg("-l")
    //             .status()
    //             .expect("Errore nell'avvio del processo");
    //
    //         // Invia il risultato tramite il canale
    //         let _ = tx.send(status);
    //     });
    //
    //     // Ascolta il risultato nel thread principale senza bloccare
    //     thread::spawn(move || {
    //         if let Ok(status) = rx.recv() {
    //             if status.success() {
    //                 println!("Il processo è terminato con successo!");
    //                 app_handle.emit("process-success", {}).unwrap();
    //             } else if let Some(code) = status.code() {
    //                 println!("Il processo è terminato con codice di errore: {}", code);
    //                 app_handle.emit("process-failed", code).unwrap();
    //             } else {
    //                 println!("Il processo è terminato in modo anomalo.");
    //                 app_handle.emit("process-failed", "unknown").unwrap();
    //             }
    //         }
    //     });
    // }

    // pub fn stop_current_smart_action(&self) {
    //     self.menu_manager.lock().unwrap().set_action_stopped();
    //
    //     self.app_handle
    //         .emit("smart_action_waiting_start", "Start waiting...")
    //         .unwrap();
    //
    //     let mut a = self.process_start.lock().unwrap();
    //     // Usa `take` per ottenere il valore senza clonarlo
    //     let process_start = Arc::new(Mutex::new(a.take()));
    //     let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
    //
    //     println!("1");
    //
    //     thread::spawn(move || {
    //         println!("2");
    //
    //         let mut process_start = process_start.lock().unwrap();
    //         let app_handle = app_handle.lock().unwrap();
    //
    //         println!("3");
    //
    //         if let Some(mut child) = process_start.take() {
    //             let id = child.id();
    //             println!("child id: {}",id);
    //
    //             let _ = kill(Pid::from_raw(id as i32), Signal::SIGINT);
    //
    //             match child.wait() {
    //                 Ok(status) => {
    //                     println!("Process exited with: {:?}", status);
    //                     if status.success() {
    //                         println!("Process exited with successful exit code");
    //                         app_handle
    //                             .emit("smart_action_waiting_stop", "Stop waiting...")
    //                             .unwrap();
    //                     } else {
    //                         eprintln!("Process exited with failing exit code");
    //                         app_handle
    //                             .emit("smart_action_waiting_error", "Error during stop waiting...")
    //                             .unwrap();
    //                     }
    //                 }
    //                 Err(e) => {
    //                     eprintln!("Failed to wait for process: {}", e);
    //                     app_handle
    //                         .emit("smart_action_waiting_error", "Error during stop waiting...")
    //                         .unwrap();
    //                 }
    //             }
    //
    //             // `process_start` ora contiene `None`, quindi è sicuro riempirlo di nuovo con `None`
    //             *process_start = None;
    //         }
    //     });
    // }
}
