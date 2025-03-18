use crate::domain::app_config::AppConfig;
use crate::domain::audio::Audio;
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
                    let mut command = Command::new("bash"); // Usa let mut per la variabile command
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

    pub fn play_sound_for_smart_action(&self, smart_action_value: &str) -> Result<(), Error> {
        if *self.is_audio_enabled.lock().unwrap() {
            self.play_sound_from_text(&format!("{}", &smart_action_value), "en")
        } else {
            println!(
                "Audio is disabled. Skipping sound for smart action: {}",
                smart_action_value,
            );
            Ok(())
        }
    }

    pub fn play_audio_file(&self, audio: Audio) {
        println!("Playing audio file: {}", audio.file());

        // ffplay -v 0 -nodisp -autoexit dictate-text-on.mp3
        if let Err(e) = Command::new("ffplay")
            .arg("-v")
            .arg("0")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(audio.file())
            .spawn()
        {
            eprintln!("Failed to play audio file '{}': {}", audio.file(), e);
        }
    }
}
