mod find_median;

use crate::utils;
pub fn launch_exercises() {
    loop {
        utils::clear_screen();
        println!("Exercises:");
        println!("1. Find Median");
        println!("0. Back");

        let selection = utils::read_input();

        let selection = match selection.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection");
                continue;
            }
        };
        println!("Selection: {}", selection);
        match selection {
            1 => find_median::launch(),
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }
    }
}
