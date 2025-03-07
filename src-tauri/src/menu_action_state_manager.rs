use std::sync::{Arc, Mutex};
use tauri::menu::MenuItem;
use tauri::Wry;

pub struct MenuActionStateManager {
    start_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    stop_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    is_action_started: bool
}

impl MenuActionStateManager {
    pub fn new(
        start_menu_item: Arc<Mutex<MenuItem<Wry>>>,
        stop_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    ) -> Self {
        Self {
            start_menu_item,
            stop_menu_item,
            is_action_started: false
        }
    }

    pub fn set_action_started(&mut self) {
        self.is_action_started = true;
        self.switch_menu_items_states();
    }

    pub fn set_action_stopped(&mut self) {
        self.is_action_started = false;
        self.switch_menu_items_states();
    }

    fn switch_menu_items_states(&mut self) {
        self.start_menu_item
            .lock()
            .unwrap()
            .set_enabled(!self.is_action_started)
            .unwrap(); // Disabilita Stop
        self.stop_menu_item
            .lock()
            .unwrap()
            .set_enabled(self.is_action_started)
            .unwrap(); // Abilita Start
    }
}