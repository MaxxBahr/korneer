// Operations on the database
pub mod database {
    use rusqlite::Params;
    use serde_json;
    use std::sync::Arc;
    use std::sync::Mutex;
    use crate::coffee::coffee::Coffee;

    #[tauri::command]
    pub fn add_new_entry(connection: Arc<Mutex<rusqlite::Connection>>, coffee: Coffee){
        let temp_con = connection.lock().unwrap();
        let _ = temp_con.execute("INSERT INTO coffee 
                            (name, 
                             rating, 
                             url, 
                             grind_size, 
                             grind_time, 
                             extraction_time, 
                             taste
                            )
                            VALUES
                            (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
                            (
                                coffee.name, 
                                coffee.rating, 
                                coffee.url, 
                                coffee.grind_size, 
                                coffee.grind_time, 
                                coffee.extraction_time, 
                                coffee.taste
                            )
                        );
    }

    #[tauri::command]
    pub fn update_entry<T: Params>(connection: Arc<Mutex<rusqlite::Connection>>, name: String, column: String, value: T){
        let temp_con = connection.lock().unwrap();
        let _ = temp_con.execute(format!("UPDATE coffee
                            SET {} = value1, column2 = value2, ...
                            WHERE name = {} ",column, name).as_str(), 
                            (value)
                        );

    }

    #[tauri::command]
    pub fn get_entry(connection: Arc<Mutex<rusqlite::Connection>>, index: u32){

    }

    #[tauri::command]
    pub fn delete_entry(connection: Arc<Mutex<rusqlite::Connection>>, index: u32){

    }
}