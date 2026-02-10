mod sort{
    pub fn sort_coffee(connection: Arc<Mutex<rusqlite::Connection>>) -> Vec<Coffee>{
        let unsorted_coffee: Vec<Coffee> = get_entry(connection, "".to_string(), "".to_string(), "".to_string());
    }

    // TODO: sort by ranking or name
    fn my_sort(arr: &mut Vec<Coffee>, low: i32, high: i32){
        if low < high{
            let pi = partition(arr, low, high);

            my_sort(arr, low, pi - 1);
            my_sort(arr, pi + 1, high);
        }
    }

    fn partition(arr: &mut Vec<Coffee>, low: i32, high: i32) -> i32{
        let pivot = arr[high];
        let mut i = low - 1;

        for j in low..(high-1){
            if arr[j] < pivot {
                i += 1;
                arr.swap(arr[i], arr[j]);
            }
        }

        arr.swap(arr[i+1], arr[high]);
        return i + 1;
    }
}