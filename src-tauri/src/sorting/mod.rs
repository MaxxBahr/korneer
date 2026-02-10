mod sort{
    pub fn mysort(connection: Arc<Mutex<rusqlite::Connection>>, criteria: T) -> Vec<Coffee>{
        let temp_con = connection.lock().unwrap();
    }
}