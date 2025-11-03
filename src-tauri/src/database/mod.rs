pub mod database {
    use serde_json;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::coffee::coffee::Coffee;

    #[tauri::command]
    pub fn add_new_entry(connection: Arc<Mutex<rusqlite::Connection>>, coffee: Coffee){

    }

    #[tauri::command]
    pub fn update_entry(connection: Arc<Mutex<rusqlite::Connection>>, coffee: Coffee){

    }

    #[tauri::command]
    pub fn get_entry(connection: Arc<Mutex<rusqlite::Connection>>, index: u32){

    }

    #[tauri::command]
    pub fn delete_entry(connection: Arc<Mutex<rusqlite::Connection>>, index: u32){

    }
}