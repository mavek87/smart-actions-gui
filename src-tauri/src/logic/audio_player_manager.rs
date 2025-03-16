use crate::domain::app_config::AppConfig;
use crate::domain::constants::AUDIO_FOLDER;
use crate::domain::smart_action::SmartActionStatus;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::sync::{Arc, Mutex};
//
// Tool used to generate voices:
//
// https://ttsmp3.com/
//
// US English / Sally
//

#[derive(Debug, Clone)]
pub struct AudioPlayerManager {
    app_config: AppConfig,
    is_audio_enabled: Arc<Mutex<bool>>,
}

impl AudioPlayerManager {
    pub fn new(app_config: AppConfig, is_audio_enabled: bool) -> Self {
        Self {
            app_config,
            is_audio_enabled: Arc::new(Mutex::new(is_audio_enabled)),
        }
    }

    pub fn set_audio_enabled(&self, is_audio_enabled: bool) -> Result<(), Error> {
        match self.is_audio_enabled.lock() {
            Ok(mut guard_is_audio_enabled) => {
                *guard_is_audio_enabled = is_audio_enabled;
                Ok(())
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Error: {}", e))),
        }
    }

    pub fn is_audio_enabled(&self) -> Result<bool, Error> {
        match self.is_audio_enabled.lock() {
            Ok(guard_is_audio_enabled) => {
                let is_audio_enabled = *guard_is_audio_enabled;
                Ok(is_audio_enabled)
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Error: {}", e))),
        }
    }

    pub fn play_sound_from_text(&self, text: &str, language: &str) -> Result<(), Error> {
        match self.is_audio_enabled.lock() {
            Ok(is_audio_enabled) => {
                if *is_audio_enabled {
                    let mut command = Command::new("bash");  // Usa let mut per la variabile command
                    command
                        .arg(format!(
                            "{}/smart-actions.sh",
                            self.app_config.smart_actions_folder
                        ))
                        .arg("text_to_speech")
                        .arg("--text")
                        .arg(text);
                    // TODO: use option could be better
                    if !language.trim().is_empty() {
                        command.arg("--language").arg(language);
                    }
                    command
                        .spawn()
                        .expect("Failed to start 'end' action from smart-actions.sh");
                }
                Ok(())
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Error: {}", e))),
        }
    }

    pub fn play_sound_for_smart_action(
        &self,
        smart_action_value: &str,
        smart_action_status: Option<SmartActionStatus>,
    ) {
        if *self.is_audio_enabled.lock().unwrap() {
            // TODO: the phisical files are not used anymore
            // let audio_file = self.find_audio_file(smart_action_value, smart_action_status);
            // self.play_audio_file(&audio_file);

            // Now the text is readed by piper speech to text directly
            let _ = match smart_action_status {
                Some(smart_action_status) => {
                    self.play_sound_from_text(&format!("{}", smart_action_status), "en")
                }
                None => {
                    self.play_sound_from_text(&format!("{}", smart_action_value), "en")
                }
            };
        } else {
            let _ = match smart_action_status {
                Some(smart_action_status) => println!(
                    "Audio is disabled. Skipping audio status: {} for smart action: {}",
                    smart_action_status, smart_action_value,
                ),
                _ => println!(
                    "Audio is disabled. Skipping sound for smart action: {}",
                    smart_action_value,
                ),
            };
        }
    }

    pub fn find_audio_file(
        &self,
        smart_action_value: &str,
        smart_action_status: Option<SmartActionStatus>,
    ) -> String {
        let audio_file = match smart_action_status {
            Some(smart_action_status) => {
                format!("{}.mp3", smart_action_status)
            }
            None => format!("{}_selected.mp3", smart_action_value),
        };

        format!("{}/{}", AUDIO_FOLDER, audio_file)
    }

    fn play_audio_file(&self, audio_file_path: &str) {
        println!("Playing audio file: {}", audio_file_path);

        // TODO: redirect output to /dev/null
        // ffplay -v 0 -nodisp -autoexit dictate-text-on.mp3
        if let Err(e) = Command::new("ffplay")
            .arg("-v")
            .arg("0")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(audio_file_path)
            .spawn()
        {
            eprintln!("Failed to start '{}': {}", audio_file_path, e);
        }
    }
}
