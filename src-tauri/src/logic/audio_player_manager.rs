use crate::domain::constants::AUDIO_FOLDER;
use crate::domain::smart_action::SmartActionStatus;
use std::process::Command;
//
// Tool used to generate voices:
//
// https://ttsmp3.com/
//
// US English / Sally
//

#[derive(Clone)]
pub struct AudioPlayerManager {
    is_audio_enabled: bool,
}

impl AudioPlayerManager {
    pub fn new(is_audio_enabled: bool) -> Self {
        Self { is_audio_enabled }
    }

    // pub fn set_audio_enabled(&mut self, is_audio_enabled: bool) {
    //     self.is_audio_enabled = is_audio_enabled;
    // }

    // TODO: bug in select auto running before recording
    pub fn play_sound_for_smart_action(
        &self,
        smart_action_value: &str,
        smart_action_status: Option<SmartActionStatus>,
    ) {
        if self.is_audio_enabled {
            let audio_file = Self::find_audio_file(smart_action_value, smart_action_status);
            Self::play_audio_file(&audio_file);
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

    fn find_audio_file(
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

    fn play_audio_file(audio_file_path: &str) {
        println!("Playing audio file: {}", audio_file_path);

        // ffplay -v 0 -nodisp -autoexit dictate-text-on.mp3
        Command::new("ffplay")
            .arg("-v")
            .arg("0")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(audio_file_path)
            .spawn()
            .expect(&format!("Failed to start '{}'", audio_file_path));
    }
}
