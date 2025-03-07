use std::sync::{Arc, Mutex};
use tauri::menu::MenuItem;
use tauri::Wry;

pub struct MenuManager {
    action_name_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    start_action_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    stop_action_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    is_action_started: bool,
}

impl MenuManager {
    pub fn new(
        action_name_menu_item: Arc<Mutex<MenuItem<Wry>>>,
        start_menu_item: Arc<Mutex<MenuItem<Wry>>>,
        stop_menu_item: Arc<Mutex<MenuItem<Wry>>>,
    ) -> Self {
        Self {
            action_name_menu_item,
            start_action_menu_item: start_menu_item,
            stop_action_menu_item: stop_menu_item,
            is_action_started: false,
        }
    }

    pub fn set_action_name_text(&mut self, text: String) {
        self.action_name_menu_item
            .lock()
            .unwrap()
            .set_text(text)
            .unwrap();
    }

    pub fn set_action_started(&mut self) {
        self.is_action_started = true;
        self.switch_action_state_in_menu();
    }

    pub fn set_action_stopped(&mut self) {
        self.is_action_started = false;
        self.switch_action_state_in_menu();
    }

    fn switch_action_state_in_menu(&mut self) {
        self.start_action_menu_item
            .lock()
            .unwrap()
            .set_enabled(!self.is_action_started)
            .unwrap(); // Disabilita Stop
        self.stop_action_menu_item
            .lock()
            .unwrap()
            .set_enabled(self.is_action_started)
            .unwrap(); // Abilita Start
    }
}

impl Clone for MenuManager {
    fn clone(&self) -> Self {
        MenuManager {
            action_name_menu_item: self.action_name_menu_item.clone(),
            start_action_menu_item: self.start_action_menu_item.clone(),
            stop_action_menu_item: self.stop_action_menu_item.clone(),
            is_action_started: self.is_action_started.clone(),
        }
    }
}
