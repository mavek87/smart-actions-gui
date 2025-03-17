use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Language {
    UNSET,
    ENGLISH,
    ITALIAN,
    SPANISH,
    FRENCH,
}

impl Language {
    pub fn value(&self) -> &'static str {
        match self {
            Language::UNSET => "Unset",
            Language::ENGLISH => "English",
            Language::ITALIAN => "Italian",
            Language::SPANISH => "Spanish",
            Language::FRENCH => "French",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::UNSET => "",
            Language::ENGLISH => "en",
            Language::ITALIAN => "it",
            Language::SPANISH => "es",
            Language::FRENCH => "fr",
        }
    }

    pub fn from_str(s: &str) -> Option<Language> {
        match s {
            "unset" | "UNSET" | "Unset" => Some(Language::UNSET),
            "en" | "EN" | "English" | "english" => Some(Language::ENGLISH),
            "it" | "IT" | "Italian" | "italian" => Some(Language::ITALIAN),
            "es" | "ES" | "Spanish" | "spanish" => Some(Language::SPANISH),
            "fr" | "FR" | "French" | "french" => Some(Language::FRENCH),
            _ => None,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
