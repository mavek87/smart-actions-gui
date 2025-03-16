use std::fmt;
use std::fmt::Display;

pub enum MouseCursorIcon {
    DEFAULT,
    WAITING,
}

impl MouseCursorIcon {
    pub fn value(&self) -> &'static str {
        match self {
            MouseCursorIcon::DEFAULT => "left_ptr",
            MouseCursorIcon::WAITING => "watch",
        }
    }
}

impl Display for MouseCursorIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
