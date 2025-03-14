use std::sync::{Arc, Mutex};
use tauri::menu::MenuItem;
use tauri::Wry;

#[derive(Clone)]
pub struct MenuManager {
    action_name_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    start_action_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    stop_action_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    is_action_started: Arc<Mutex<bool>>,
}

impl MenuManager {
    pub fn new(
        action_name_menu_item: MenuItem<Wry>,
        start_menu_item: MenuItem<Wry>,
        stop_menu_item: MenuItem<Wry>,
    ) -> Self {
        Self {
            action_name_menu_item: Arc::new(Mutex::new(action_name_menu_item)),
            start_action_menu_item: Arc::new(Mutex::new(start_menu_item)),
            stop_action_menu_item: Arc::new(Mutex::new(stop_menu_item)),
            is_action_started: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_action_name_text(&self, text: &str) {
        self.action_name_menu_item
            .lock()
            .unwrap()
            .set_text(text)
            .unwrap();
    }

    pub fn set_action_started(&self) {
        {
            let guard_is_action_started = self.is_action_started.lock().unwrap();
            *guard_is_action_started = true;
        }
        self.switch_action_state_in_menu();
    }

    pub fn set_action_stopped(&self) {
        {
            let guard_is_action_started = self.is_action_started.lock().unwrap();
            *guard_is_action_started = false;
        }
        self.switch_action_state_in_menu();
    }

    // TODO: here there is a deadlock with guard_is_action_started if one of the next unwrap fails
    fn switch_action_state_in_menu(&self) {
        let guard_is_action_started = self.is_action_started.lock().unwrap();
        self.start_action_menu_item
            .lock()
            .unwrap()
            .set_enabled(!*guard_is_action_started)
            .unwrap(); // Disabilita Stop
        self.stop_action_menu_item
            .lock()
            .unwrap()
            .set_enabled(*guard_is_action_started)
            .unwrap(); // Abilita Start
    }
}
