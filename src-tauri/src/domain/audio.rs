use crate::domain::constants::AUDIO_FOLDER;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Audio {
    START,
    STOP,
}

impl Audio {
    pub fn file(&self) -> String {
        match self {
            Audio::START => format!("{}/{}", AUDIO_FOLDER, "start.mp3"),
            Audio::STOP => format!("{}/{}", AUDIO_FOLDER, "stop.mp3"),
        }
    }
}

impl Display for Audio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.file())
    }
}
