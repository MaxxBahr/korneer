mod coffee;
mod database;

use crate::coffee::coffee::{Coffee, FieldValue};

use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

struct DatabaseConncetion {
    connection: Connection,
}

#[tauri::command]
fn new_coffee(name: String, url: String) -> bool {
    let coffee = Coffee::new(name, url);
    false
}

#[tauri::command]
fn edit_existing_coffee(value: FieldValue) -> bool {
    // catch coffee from database
    // edit its values with the corresponding function
    // coffee.edit_value(value);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![new_coffee])
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(DatabaseConncetion {
                connection: Connection::open("coffee.db").unwrap(),
            })));
            let shell = app.shell();
            let output = tauri::async_runtime::block_on(async move {
                shell
                    .command("sh")
                    .args(["-c","sqlite3 coffee.db < schema.sql"])
                    .output()
                    .await
                    .unwrap()  
            });
            if output.status.success(){
                println!("Result: {:?}", String::from_utf8(output.stdout));
            }else{
                println!("Exit with code: {}", output.status.code().unwrap());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
