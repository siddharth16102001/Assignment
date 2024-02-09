fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_index = len / 2;
        (arr[mid_index - 1] + arr[mid_index]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let sorted_array1 = [1, 2, 3, 4, 5];
    let sorted_array2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of sorted_array1: {}", find_median(&sorted_array1));
    println!("Median of sorted_array2: {}", find_median(&sorted_array2));
}
