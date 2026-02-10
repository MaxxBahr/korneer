mod sort{
    pub fn sort_coffee<T: Params + rusqlite::ToSql>(connection: Arc<Mutex<rusqlite::Connection>>, criteria: T) -> Vec<Coffee>{
        let unsorted_coffee: Vec<Coffee> = get_entry(connection, "".to_string(), criteria);
        // Implement quick sort        
    }

    fn my_sort<T: PartialEq + Eq + PartialOrd + Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T>{

    }
}