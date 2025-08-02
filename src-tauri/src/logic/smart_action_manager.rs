use crate::domain::app_config::AppConfig;
use crate::domain::audio::Audio;
use crate::domain::constants::{
    EVENT_TO_UI_ENABLE_AUDIO_CHANGED, EVENT_TO_UI_NEXT_SMART_ACTION,
    EVENT_TO_UI_PREVIOUS_SMART_ACTION, EVENT_TO_UI_RECORDING_START, EVENT_TO_UI_WAITING_ERROR,
    EVENT_TO_UI_WAITING_START, EVENT_TO_UI_WAITING_STOP,
};
use crate::domain::mouse_cursor_icon::MouseCursorIcon;
use crate::domain::smart_action::{SmartAction, SmartActionState, SmartActionStatus};
use crate::logic::audio_player_manager::AudioPlayerManager;
use crate::logic::menu_manager::MenuManager;
use crate::logic::tray_icon_manager::TrayIconManager;
use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct SmartActionManager {
    app_handle: AppHandle,
    app_config: AppConfig,
    menu_manager: MenuManager,
    tray_icon_manager: TrayIconManager,
    audio_player_manager: AudioPlayerManager,
    smart_action_state: Mutex<SmartActionState>, // TODO: can be directly an arc mutex right??
    is_running: Arc<Mutex<bool>>,
    is_waiting: Arc<Mutex<bool>>,
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
            menu_manager,
            tray_icon_manager,
            audio_player_manager,
            smart_action_state: Mutex::new(SmartActionState::new(smart_action)), // TODO: can be directly an arc mutex right??
            is_running: Arc::new(Mutex::new(false)),
            is_waiting: Arc::new(Mutex::new(false)),
        }
    }

    pub fn change_with_next_smart_action(&self) {
        if let Err(e) = self.app_handle.emit(&EVENT_TO_UI_NEXT_SMART_ACTION, "") {
            eprintln!("Error during emission: {}", e);
        }
    }

    pub fn change_with_previous_smart_action(&self) {
        if let Err(e) = self.app_handle.emit(&EVENT_TO_UI_PREVIOUS_SMART_ACTION, "") {
            eprintln!("Error during emission: {}", e);
        }
    }

    pub fn change_current_smart_action(&self, new_smart_action: SmartAction) {
        let smart_action_name = &new_smart_action.name;

        self.menu_manager.set_action_name_text(&smart_action_name);

        let mut current_smart_action = self.smart_action_state.lock().unwrap();
        *current_smart_action = SmartActionState::new(new_smart_action);
    }

    pub fn start_current_smart_action(&self) {
        if !*self.is_running.lock().unwrap() && !*self.is_waiting.lock().unwrap() {
            *self.is_running.lock().unwrap() = true;
        } else {
            println!("Smart action cannot be started because it's already started...");
            return;
        }

        self.tray_icon_manager
            .show_default_icon()
            .unwrap_or_else(|err| eprintln!("Error showing default icon: {}", err));

        self.menu_manager.set_action_started();

        let smart_action_state = self.smart_action_state.lock().unwrap();
        let current_smart_action_value = smart_action_state.value.lock().unwrap();
        let current_smart_action_args = smart_action_state.args.lock().unwrap();

        self.audio_player_manager.play_audio_file(Audio::START);

        let mut command_smart_action =
            self.build_cmd_smart_action(&current_smart_action_value, current_smart_action_args);

        let process_command_smart_action = command_smart_action.spawn().expect(&format!(
            "Failed to start '{} smart action",
            current_smart_action_value
        ));

        let thread_check_status_closure = {
            let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
            let command_smart_action = Arc::new(Mutex::new(process_command_smart_action));
            let tray_icon_manager = Arc::new(Mutex::new(self.tray_icon_manager.clone()));
            let audio_player_manager = Arc::new(Mutex::new(self.audio_player_manager.clone()));

            let is_running = self.is_running.clone();
            let is_waiting = self.is_waiting.clone();

            move || {
                let result_exit_status = command_smart_action.lock().unwrap().wait();

                let set_not_running_nor_waiting_states = || {
                    *is_running.lock().unwrap() = false;
                    *is_waiting.lock().unwrap() = false;
                };

                let exit_status = match result_exit_status {
                    Ok(exit_status) => {
                        set_not_running_nor_waiting_states();
                        exit_status
                    }
                    Err(e) => {
                        eprintln!("Error {e}");
                        set_not_running_nor_waiting_states();
                        return;
                    }
                };

                let _ = Self::emit_terminal_event(app_handle, &exit_status);

                {
                    tray_icon_manager
                        .lock()
                        .unwrap()
                        .show_default_icon()
                        .unwrap_or_else(|err| eprintln!("Error showing default icon: {}", err));
                }

                Self::set_mouse_cursor(&MouseCursorIcon::DEFAULT);

                {
                    audio_player_manager
                        .lock()
                        .unwrap()
                        .play_audio_file(Audio::STOP);
                }
            }
        };

        thread::spawn(thread_check_status_closure);

        if let Err(e) = self
            .app_handle
            .emit(&EVENT_TO_UI_RECORDING_START, "Start recording...")
        {
            eprintln!("Error during emission: {}", e);
        }

        println!("Recording started!");
    }

    pub fn stop_current_smart_action(&self) {
        if *self.is_running.lock().unwrap() && !*self.is_waiting.lock().unwrap() {
            *self.is_waiting.lock().unwrap() = true;
        } else {
            println!("Smart action can't be stopped because it's not running or it's still waiting for a response");
            return;
        }

        // TODO: to be more precise this should be set to true only if a vocal audio is running for the current smart action...
        self.menu_manager.set_vocal_audio_menu_item_enabled(true);

        let mut child = Command::new("bash")
            .arg(format!(
                "{}/smart-actions.sh",
                self.app_config.smart_actions_folder
            ))
            .arg("end")
            .spawn()
            .expect("Failed to start 'end' action from smart-actions.sh");

        // TODO: this blocks the tauri main thread... it's not ok
        if let Err(err) = child.wait() {
            eprintln!("Error while waiting for process termination: {}", err);
        } else {
            self.menu_manager.set_action_stopped();

            self.tray_icon_manager
                .show_waiting_icon()
                .unwrap_or_else(|err| eprintln!("Error showing wait icon: {}", err));

            if let Err(e) = self
                .app_handle
                .emit(&EVENT_TO_UI_WAITING_START, "Waiting response...")
            {
                eprintln!("Error during emission: {}", e);
            }

            Self::set_mouse_cursor(&MouseCursorIcon::WAITING);

            println!("Smart action stopped!");
        }
    }

    pub fn stop_vocal_audio(&self) {
        if let Err(e) = Command::new("bash")
            .arg(format!(
                "{}/smart-actions.sh",
                self.app_config.smart_actions_folder
            ))
            .arg("end_output_audio_vocal")
            .spawn()
        {
            eprintln!("Error stopping vocal audio: {}", e);
        } else {
            println!("Vocal audio stopped!");
        }

        self.menu_manager.set_vocal_audio_menu_item_enabled(false)
    }

    pub fn is_audio_enabled(&self) -> Result<bool, Error> {
        self.audio_player_manager.is_audio_enabled()
    }

    pub fn set_audio_enable(&self, is_audio_enabled: bool) {
        if let Err(e) = self
            .audio_player_manager
            .set_audio_enabled(is_audio_enabled)
        {
            eprintln!(
                "Error during setting audio enabled ({}) {}",
                is_audio_enabled, e
            );
        } else {
            if let Err(e) = self
                .app_handle
                .emit(&EVENT_TO_UI_ENABLE_AUDIO_CHANGED, is_audio_enabled)
            {
                eprintln!("Error during emission: {}", e);
            }
        }
    }

    pub fn try_to_fix_issues(&self) {
        let processes_to_kill: Vec<&str> = vec![
            "piper",
            "nerd-dictation",
            "whisper",
            "ffmpeg",
            "tgpt",
            "dotool",
            "smart-actions.sh",
        ];

        for process in processes_to_kill {
            self.try_to_kill_process_or_print_error_if_it_fail(process);
        }

        Self::set_mouse_cursor(&MouseCursorIcon::DEFAULT);
    }

    fn try_to_kill_process_or_print_error_if_it_fail(&self, system_process_name: &str) {
        Command::new("pkill")
            .arg("-9")
            .arg("-f")
            .arg(system_process_name)
            .spawn()
            .map_err(|e| eprintln!("Error trying to kill process {e}"))
            .ok();
    }

    fn build_cmd_smart_action(
        &self,
        current_smart_action_value: &MutexGuard<String>,
        current_smart_action_args: MutexGuard<Vec<HashMap<String, String>>>,
    ) -> Command {
        let mut command_smart_action = Command::new("bash");

        let script_path = format!("{}/smart-actions.sh", self.app_config.smart_actions_folder);

        command_smart_action
            .arg(&script_path)
            .arg(format!("{}", &current_smart_action_value));

        let smart_action_value = current_smart_action_value.to_string();

        let mut debug_cmd = format!("bash {} {}", script_path, smart_action_value);

        for arg in current_smart_action_args.iter() {
            let mut arg_param: String = String::new();
            let mut arg_value: String = String::new();

            for arg_key in arg.keys() {
                if let Some(value) = arg.get(arg_key) {
                    if arg_key == "arg" {
                        arg_param = value.to_string(); // -l
                    } else {
                        // TODO: piper doesn't wait for the pauses... I don't know why
                        if smart_action_value == "text_to_speech" && arg_key == "text" {
                            arg_value = value
                                .to_string()
                                .replace("$", "")
                                .replace("\n", ". ")
                                .replace("\"", "")
                                .replace("'", "");
                        } else {
                            arg_value = value.to_string(); // it
                        }
                    }
                }
            }

            if !arg_value.is_empty() {
                command_smart_action.arg(&arg_param);
                command_smart_action.arg(&arg_value);

                debug_cmd.push(' ');
                debug_cmd.push_str(&arg_param);
                debug_cmd.push(' ');
                debug_cmd.push_str(&arg_value);
            }
        }

        println!("Command: {}", debug_cmd);

        command_smart_action
    }

    fn emit_terminal_event(
        arc_mutex_app_handle: Arc<Mutex<AppHandle>>,
        exit_status: &ExitStatus,
    ) -> SmartActionStatus {
        let app_handle_guard = arc_mutex_app_handle.lock().unwrap();
        let smart_action_status;
        if exit_status.success() {
            println!("The process is end with success!");

            smart_action_status = SmartActionStatus::COMPLETED;

            if let Err(e) = app_handle_guard.emit(&EVENT_TO_UI_WAITING_STOP, "Stop waiting...") {
                eprintln!("Error during emission: {}", e);
                drop(app_handle_guard);
            }
        } else if let Some(code) = exit_status.code() {
            eprintln!("The process is end with error status code: {}", code);

            smart_action_status = SmartActionStatus::FAILED;

            if let Err(e) =
                app_handle_guard.emit(&EVENT_TO_UI_WAITING_ERROR, "Error during waiting...")
            {
                eprintln!("Error during emission: {}", e);
                drop(app_handle_guard);
            }
        } else {
            eprintln!("The process is end anomaly");

            smart_action_status = SmartActionStatus::FAILED;

            if let Err(e) =
                app_handle_guard.emit(&EVENT_TO_UI_WAITING_ERROR, "Error during waiting...")
            {
                eprintln!("Error during emission: {}", e);
                drop(app_handle_guard);
            }
        }

        smart_action_status
    }

    fn set_mouse_cursor(mouse_cursor_type: &MouseCursorIcon) {
        Command::new("xsetroot")
            .arg("-cursor_name")
            .arg(mouse_cursor_type.value())
            .spawn()
            .map_err(|e| eprintln!("Error trying to change the mouse cursor {e}"))
            .ok();
    }
}
