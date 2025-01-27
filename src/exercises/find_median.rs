use crate::utils;

fn find_median(arr: Vec<i32>) -> f64 {
    let mut arr = arr;
    arr.sort();
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

pub fn launch() {
    loop {
        println!("Find Median");
        println!(
        "Enter a list of numbers separated by commas, or type 'exit' to return to the main menu"
    );
        let input = utils::read_input();
        // TODO: Parse the input into a vector of integers
        if input.trim() == "exit" {
            break;
        }
        let arr = vec![1, 2, 3, 4, 5];
        println!("Array: {:?}", arr);
        println!("Median: {}", find_median(arr));
    }
}
