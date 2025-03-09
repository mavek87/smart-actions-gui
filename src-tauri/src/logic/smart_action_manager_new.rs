// use crate::domain::app_config::AppConfig;
// use crate::domain::smart_action::{SmartAction, SmartActionState};
// use crate::logic::menu_manager::MenuManager;
// use nix::sys::signal::{kill, Signal};
// use nix::unistd::Pid;
// use std::process::{Child, Command};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use tauri::{AppHandle, Emitter};
//
// pub struct SmartActionManager {
//     app_handle: AppHandle,
//     app_config: AppConfig,
//     menu_manager: Mutex<MenuManager>,
//     smart_action_state: Mutex<SmartActionState>,
//     process_start: Mutex<Option<Child>>,
//     process_stop: Mutex<Option<Child>>,
// }
//
// impl SmartActionManager {
//     pub fn new(
//         app_handle: AppHandle,
//         app_config: AppConfig,
//         menu_manager: MenuManager,
//         smart_action: SmartAction,
//     ) -> Self {
//         SmartActionManager {
//             app_handle,
//             app_config,
//             menu_manager: Mutex::new(menu_manager),
//             smart_action_state: Mutex::new(SmartActionState::new(smart_action)),
//             process_start: Mutex::new(None::<Child>),
//             process_stop: Mutex::new(None::<Child>),
//         }
//     }
//
//     pub fn change_current_smart_action(&self, new_smart_action: SmartAction) {
//         let mut current_smart_action = self.smart_action_state.lock().unwrap();
//         *current_smart_action = SmartActionState::new(new_smart_action.clone());
//
//         let action_name = format!("{}", new_smart_action.name);
//         self.menu_manager
//             .lock()
//             .unwrap()
//             .set_action_name_text(action_name);
//     }
//
//     // TODO: handle errors
//     pub fn start_current_smart_action(&self) {
//         // TODO: unlock if error occurs
//         self.menu_manager.lock().unwrap().set_action_started();
//
//         let smart_action_state = self.smart_action_state.lock().unwrap();
//
//         let current_smart_action_value = smart_action_state.value.lock().unwrap();
//         let current_smart_action_args = smart_action_state.args.lock().unwrap();
//
//         if self.process_start.lock().unwrap().is_none() {
//             let mut command_smart_action = Command::new("bash");
//
//             command_smart_action
//                 .arg(format!(
//                     "{}/smart-actions.sh",
//                     self.app_config.faster_whisper_folder
//                 ))
//                 .arg(format!("{}", current_smart_action_value));
//
//             // TODO: a refactoring is necessary
//             for arg in current_smart_action_args.iter() {
//                 let mut arg_param: String = String::new();
//                 let mut arg_value: String = String::new();
//
//                 for arg_key in arg.keys() {
//                     if let Some(value) = arg.get(arg_key) {
//                         if arg_key == "arg" {
//                             arg_param = value.to_string(); // -l
//                         } else {
//                             arg_value = value.to_string(); // it
//                         }
//                     }
//                 }
//
//                 // let command_arg = format!("{} {}", arg_param, arg_value);
//                 // println!("Argomento: {}", command_arg);
//
//                 // TODO: what to do if value is empty?
//                 if !arg_value.is_empty() {
//                     command_smart_action.arg(arg_param);
//                     command_smart_action.arg(arg_value);
//                 }
//             }
//
//             let process_command_smart_action = command_smart_action
//                 .spawn()
//                 .expect(format!("Failed to start {} action from smart-actions.sh", current_smart_action_value).as_str());
//
//             *self.process_start.lock().unwrap() = Some(process_command_smart_action);
//
//             let mut i = self.process_start.lock().unwrap();
//             let x = i.as_ref().unwrap().id();
//             println!("Process ID: {}", x);
//
//             self.app_handle
//                 .emit("smart_action_recording_start", "Start recording...")
//                 .unwrap();
//
//             println!("Recording started!");
//         } else {
//             println!("Recording is already running.");
//         }
//     }
//
//     // pub fn stop_current_smart_action(&self) {
//     //     // TODO: unlock if error occurs (???)
//     //     self.menu_manager.lock().unwrap().set_action_stopped();
//     //
//     //     self.app_handle
//     //         .emit("smart_action_waiting_start", "start waiting...")
//     //         .unwrap();
//     //
//     //     // Gestione del processo di registrazione
//     //     let mut process_stop = self.process_stop.lock().unwrap();
//     //     if process_stop.is_none() {
//     //         let child = Command::new("bash")
//     //             .arg(format!(
//     //                 "{}/smart-actions.sh",
//     //                 self.app_config.faster_whisper_folder
//     //             ))
//     //             .arg("end")
//     //             .spawn()
//     //             .expect("Failed to start 'end' action from smart-actions.sh");
//     //         *process_stop = Some(child);
//     //
//     //         // Aspettiamo che il processo STOP termini
//     //         if let Some(mut child) = process_stop.take() {
//     //             if let Err(err) = child.wait() {
//     //                 eprintln!("Error while waiting for process termination: {}", err);
//     //                 self.app_handle
//     //                     .emit("smart_action_waiting_error", "error during waiting...")
//     //                     .unwrap();
//     //             } else {
//     //                 self.app_handle
//     //                     .emit("smart_action_waiting_stop", "stop waiting...")
//     //                     .unwrap();
//     //             }
//     //         }
//     //
//     //         let mut process_start = self.process_start.lock().unwrap();
//     //         *process_start = None;
//     //         *process_stop = None;
//     //
//     //         println!("Recording stop!");
//     //     } else {
//     //         println!("Recording already stopping.");
//     //     }
//     // }
//
//     // pub fn stop_current_smart_action(&self) {
//     //     // TODO: unlock if error occurs (???)
//     //     self.menu_manager.lock().unwrap().set_action_stopped();
//     //
//     //     self.app_handle
//     //         .emit("smart_action_waiting_start", "Start waiting...")
//     //         .unwrap();
//     //
//     //     let mut process_start = self.process_start.lock().unwrap();
//     //     let mut child = process_start.take().unwrap();
//     //
//     //     thread::spawn(move || {
//     //         // Invia un SIGINT (Ctrl+C)
//     //         let _ = kill(Pid::from_raw(child.id() as i32), Signal::SIGINT);
//     //
//     //         // Attendi che il processo termini
//     //         match child.wait() {
//     //             Ok(status) => {
//     //                 println!("Process exited with: {:?}", status);
//     //                 if status.success() {
//     //                     println!("Process exited with successful exit code");
//     //                     self.app_handle
//     //                         .emit("smart_action_waiting_stop", "Stop waiting...")
//     //                         .unwrap();
//     //                     *process_start = None;
//     //                 } else {
//     //                     eprintln!("Process exited with failing exit code");
//     //                     self.app_handle
//     //                         .emit("smart_action_waiting_error", "Error during stop waiting...")
//     //                         .unwrap();
//     //                     *process_start = None;
//     //                 }
//     //             }
//     //             Err(e) => {
//     //                 eprintln!("Failed to wait for process: {}", e);
//     //                 self.app_handle
//     //                     .emit("smart_action_waiting_error", "Error during stop waiting...")
//     //                     .unwrap();
//     //                 *process_start = None;
//     //             }
//     //         }
//     //     });
//     // }
//
//     // pub fn stop_current_smart_action(&self) {
//     //     self.menu_manager.lock().unwrap().set_action_stopped();
//     //
//     //     self.app_handle
//     //         .emit("smart_action_waiting_start", "Start waiting...")
//     //         .unwrap();
//     //
//     //     let a = self.process_start.lock().unwrap();
//     //     let process_start = Arc::new(Mutex::new(a.clone()));
//     //     let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
//     //
//     //     thread::spawn(move || {
//     //         let mut process_start = process_start.lock().unwrap();
//     //         let app_handle = app_handle.lock().unwrap();
//     //
//     //         if let Some(mut child) = process_start.take() {
//     //             let _ = kill(Pid::from_raw(child.id() as i32), Signal::SIGINT);
//     //
//     //             match child.wait() {
//     //                 Ok(status) => {
//     //                     println!("Process exited with: {:?}", status);
//     //                     if status.success() {
//     //                         println!("Process exited with successful exit code");
//     //                         app_handle
//     //                             .emit("smart_action_waiting_stop", "Stop waiting...")
//     //                             .unwrap();
//     //                     } else {
//     //                         eprintln!("Process exited with failing exit code");
//     //                         app_handle
//     //                             .emit("smart_action_waiting_error", "Error during stop waiting...")
//     //                             .unwrap();
//     //                     }
//     //                 }
//     //                 Err(e) => {
//     //                     eprintln!("Failed to wait for process: {}", e);
//     //                     app_handle
//     //                         .emit("smart_action_waiting_error", "Error during stop waiting...")
//     //                         .unwrap();
//     //                 }
//     //             }
//     //
//     //             *process_start = None;
//     //         }
//     //     });
//     // }
//
//     pub fn stop_current_smart_action(&self) {
//         self.menu_manager.lock().unwrap().set_action_stopped();
//
//         self.app_handle
//             .emit("smart_action_waiting_start", "Start waiting...")
//             .unwrap();
//
//         let mut a = self.process_start.lock().unwrap();
//         // Usa `take` per ottenere il valore senza clonarlo
//         let process_start = Arc::new(Mutex::new(a.take()));
//         let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
//
//         println!("1");
//
//         thread::spawn(move || {
//             println!("2");
//
//             let mut process_start = process_start.lock().unwrap();
//             let app_handle = app_handle.lock().unwrap();
//
//             println!("3");
//
//             if let Some(mut child) = process_start.take() {
//                 let id = child.id();
//                 println!("child id: {}",id);
//
//                 let _ = kill(Pid::from_raw(id as i32), Signal::SIGINT);
//
//                 match child.wait() {
//                     Ok(status) => {
//                         println!("Process exited with: {:?}", status);
//                         if status.success() {
//                             println!("Process exited with successful exit code");
//                             app_handle
//                                 .emit("smart_action_waiting_stop", "Stop waiting...")
//                                 .unwrap();
//                         } else {
//                             eprintln!("Process exited with failing exit code");
//                             app_handle
//                                 .emit("smart_action_waiting_error", "Error during stop waiting...")
//                                 .unwrap();
//                         }
//                     }
//                     Err(e) => {
//                         eprintln!("Failed to wait for process: {}", e);
//                         app_handle
//                             .emit("smart_action_waiting_error", "Error during stop waiting...")
//                             .unwrap();
//                     }
//                 }
//
//                 // `process_start` ora contiene `None`, quindi è sicuro riempirlo di nuovo con `None`
//                 *process_start = None;
//             }
//         });
//     }
// }
