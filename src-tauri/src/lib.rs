mod coffee;
mod database;

use crate::coffee::coffee::{Coffee, FieldValue};
use crate::database::database::add_new_entry;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

struct DatabaseConncetion {
    connection: Connection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_new_entry])
        .setup(|app| {
            let shell = app.shell();
            let platform = tauri_plugin_os::platform();
            let system_exe: &str;
            if platform == "windows"{
                system_exe = "sqlite3.exe";
            }else{
                system_exe = "sqlite3";
            }                
            let output = tauri::async_runtime::block_on(async move {
                shell
                    .command("sh")
                    // Use string slice for OS seperation
                    .args(["-c", format!("{} coffee.db < schema.sql", system_exe).as_str()])
                    .output()
                    .await
                    .unwrap()
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
