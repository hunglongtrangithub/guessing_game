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
    utils::clear_screen();
    println!("Find Median");
    loop {
        println!(
        "Enter a list of integers separated by commas, or just press Enter to return to the main menu"
    );

        let input = utils::read_input();

        if input.trim() == "" {
            break;
        }

        let arr_result: Result<Vec<i32>, _> =
            input.split(",").map(|x| x.trim().parse::<i32>()).collect();

        match arr_result {
            Ok(arr) => {
                if arr.is_empty() {
                    println!("You entered an empty list. Try again.");
                    continue;
                }
                println!("Array: {:?}", arr);
                println!("Median: {}", find_median(arr));
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid list of integers.");
                continue;
            }
        }
    }
}
