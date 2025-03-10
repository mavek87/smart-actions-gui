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

const AUDIO_FOLDER: &str = "audio";

impl AudioPlayerManager {
    pub fn new(is_audio_enabled: bool) -> Self {
        Self { is_audio_enabled }
    }

    pub fn set_audio_enabled(&mut self, is_audio_enabled: bool) {
        self.is_audio_enabled = is_audio_enabled;
    }

    pub fn play_sound_for_smart_action(
        &mut self,
        smart_action_value: String,
        smart_action_status: SmartActionStatus,
    ) {
        let smart_action_value = smart_action_value.clone();
        if self.is_audio_enabled {
            let audio_file = Self::find_audio_file_for_smart_action_status(
                smart_action_value,
                smart_action_status,
            );
            Self::play_audio_file(audio_file);
        } else {
            println!(
                "Audio is disabled. Skipping audio status: {} for smart action: {}",
                smart_action_status, smart_action_value,
            );
        }
    }

    fn find_audio_file_for_smart_action_status(
        smart_action_value: String,
        smart_action_status: SmartActionStatus,
    ) -> String {
        let audio_file;

        if smart_action_status == SmartActionStatus::SELECTED {
            audio_file = format!("{}_{}.mp3", smart_action_value, smart_action_status);
        } else {
            audio_file = format!("{}.mp3", smart_action_status);
        }

        let audio_file = format!("{}/{}", AUDIO_FOLDER, audio_file);

        audio_file
    }

    fn play_audio_file(audio_file_path: String) {
        println!("Playing audio file: {}", audio_file_path);

        // ffplay -v 0 -nodisp -autoexit dictate-text-on.mp3
        Command::new("ffplay")
            .arg("-v")
            .arg("0")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(&audio_file_path)
            .spawn()
            .expect(&format!("Failed to start '{}'", audio_file_path));
    }
}
