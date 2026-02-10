// Operations on the database
pub mod database {
    use rusqlite::MappedRows;
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
                            SET {} = ?1
                            WHERE name = {} ",column, name).as_str(), 
                            (value)
                        );

    }

    #[tauri::command]
    pub fn get_entry<T: Params + rusqlite::ToSql>(connection: Arc<Mutex<rusqlite::Connection>>,column: String, value: T) -> Result<Vec<Coffee>, rusqlite::Error>{
        let temp_con = connection.lock().unwrap();
        let query = if column == "".to_string(){
            "SELECT * FROM coffee".to_string()
        }else{
            format!("
                    SELECT *
                    FROM coffee
                    WHERE {column} = ?1")
        };
        let mut stmt = temp_con.prepare(query.as_str())?;

        let result = stmt.query_map([value], |row| {
            Ok(Coffee { 
                name: row.get(0)?, 
                rating: row.get(1)?, 
                url: row.get(2)?, 
                grind_size: row.get(3)?, 
                grind_time: row.get(4)?, 
                extraction_time: row.get(5)?, 
                taste: row.get(6)? 
            })
        })?;
        let mut res_vec = Vec::new();
        for row in result{
            res_vec.push(row?);
        }
        Ok(res_vec)
    }

    #[tauri::command]
    pub fn delete_entry(connection: Arc<Mutex<rusqlite::Connection>>, name: String){
        let temp_con = connection.lock().unwrap();
        let _ = temp_con.execute("DELETE
                                  FROM coffee
                                  WHERE name = ?1", (name,));  
    }
}