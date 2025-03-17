use crate::domain::language::Language;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct LanguageManager {
    current_language: Arc<Mutex<Language>>,
}

impl LanguageManager {
    pub fn new(current_language: Language) -> Self {
        Self {
            current_language: Arc::new(Mutex::new(current_language)),
        }
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language.lock().unwrap().clone()
    }

    pub fn set_current_language(&self, new_language: Language) {
        self.set_current_language_as_str(new_language.value());
    }

    pub fn set_current_language_as_str(&self, new_language: &str) {
        let language = Language::from_str(new_language).unwrap_or(Language::UNSET);
        *self.current_language.lock().unwrap() = language;
    }
}
