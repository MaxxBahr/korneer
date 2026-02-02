mod coffee;
mod database;

use crate::coffee::coffee::{Coffee, FieldValue};

use std::sync::Mutex;
use std::sync::Arc;
use rusqlite::Connection;
use tauri::Manager;

struct DatabaseConncetion{
    connection: Connection,
}

#[tauri::command]
fn new_coffee(name: String, url: String)-> bool{
    let coffee = Coffee::new(name, url);
    false
}

#[tauri::command]
fn edit_existing_coffee(value: FieldValue) -> bool{
    // catch coffee from database
    // edit its values with the corresponding function
    // coffee.edit_value(value);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![new_coffee])
        .setup(|app|{
            app.manage(
                Arc::new(
                    Mutex::new(DatabaseConncetion
                        {
                        connection: Connection::open("coffee.db").unwrap(),
                        }
                    )   
                )
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
