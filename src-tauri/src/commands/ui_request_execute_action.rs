use tauri::command;
use crate::domain::action::SmartAction;

#[command]
pub fn ui_request_execute_action(json_action: String) {
    // println!("AAAAAAAAAAAAAAAAAAAAAAAAAAA\n\n\n");

    // println!("{}", &json_action);

    let smart_action: SmartAction = serde_json::from_str(&json_action).expect("Failed to parse JSON");
    println!("?:{:?}", smart_action);
    // println!("?:{}", action);
}