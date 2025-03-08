use crate::domain::app_config::AppConfig;
use crate::domain::smart_action::{SmartAction, SmartActionState};
use crate::logic::menu_manager::MenuManager;
use std::process::{Child, Command};
use std::sync::Mutex;

pub struct SmartActionManager {
    smart_action_state: Mutex<SmartActionState>,
    app_config: AppConfig,
    menu_manager: Mutex<MenuManager>,
    process_start: Mutex<Option<Child>>,
    process_stop: Mutex<Option<Child>>,
}

impl SmartActionManager {
    pub fn new(
        app_config: AppConfig,
        menu_manager: MenuManager,
        smart_action: SmartAction,
    ) -> Self {
        SmartActionManager {
            app_config,
            smart_action_state: Mutex::new(SmartActionState::new(smart_action)),
            menu_manager: Mutex::new(menu_manager),
            process_start: Mutex::new(None::<Child>),
            process_stop: Mutex::new(None::<Child>),
        }
    }

    pub fn change_current_smart_action(&self, new_smart_action: SmartAction) {
        let mut current_smart_action = self.smart_action_state.lock().unwrap();
        *current_smart_action = SmartActionState::new(new_smart_action);
    }

    pub fn start_current_smart_action(&self) {
        // TODO: unlock if error occurs
        self.menu_manager.lock().unwrap().set_action_started();

        let smart_action_state = self.smart_action_state.lock().unwrap();

        let current_smart_action_value = smart_action_state.value.lock().unwrap();
        let current_smart_action_args = smart_action_state.args.lock().unwrap();

        if self.process_start.lock().unwrap().is_none() {
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

                let command_arg = format!("{} {}", arg_param, arg_value);
                println!("Argomento: {}", command_arg);

                // TODO: what to do if value is empty?
                if !arg_value.is_empty() {
                    command_smart_action.arg(arg_param);
                    command_smart_action.arg(arg_value);
                }
            }

            let process_command_smart_action = command_smart_action
                .spawn()
                .expect("Failed to start 'dictate_text' action from smart-actions.sh");

            *self.process_start.lock().unwrap() = Some(process_command_smart_action);
            println!("Recording started!");
        } else {
            println!("Recording is already running.");
        }
    }

    pub fn stop_current_smart_action(&self) {
        // TODO: unlock if error occurs (???)
        self.menu_manager.lock().unwrap().set_action_stopped();

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
                }
            }

            let mut process_start = self.process_start.lock().unwrap();
            *process_start = None;
            *process_stop = None;

            println!("Recording stop!");
        } else {
            println!("Recording already stopping.");
        }
    }
}
