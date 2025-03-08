use crate::domain::smart_action::{SmartAction, SmartActionState};
use std::sync::Mutex;

pub struct SmartActionStateManager {
    pub smart_action_state: Mutex<SmartActionState>,
}

impl SmartActionStateManager {
    pub fn new(smart_action: SmartAction) -> Self {
        SmartActionStateManager {
            smart_action_state: Mutex::new(SmartActionState::new(smart_action)),
        }
    }

    pub fn change_smart_action(&self, new_smart_action: SmartAction) {
        let mut current_smart_action = self.smart_action_state.lock().unwrap();
        *current_smart_action = SmartActionState::new(new_smart_action);
    }
}
