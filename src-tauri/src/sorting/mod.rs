mod sort{
    pub fn sort_coffee(connection: Arc<Mutex<rusqlite::Connection>>) -> Vec<Coffee>{
        let mut unsorted_coffee: Vec<Coffee> = get_entry(connection, "".to_string(), "".to_string(), "".to_string());
        my_sort(&unsorted_coffee, 0, unsorted_coffee.len());
        unsorted_coffee
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
        let pivot = arr[high as usize];
        let mut i = low - 1;

        for j in low..high{
            if arr[j] < pivot {
                i += 1;
                arr.swap(i as usize, j as usize);
            }
        }

        arr.swap((i+1) as usize, high as usize);
        return i + 1;
    }
}