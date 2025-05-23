mod add_employee;
mod find_median;
mod pig_latin;

use crate::utils;
pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Exercises:");
        println!("1. Find Median");
        println!("2. Pig Latin");
        println!("3. Add Employee");
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
            2 => pig_latin::launch(),
            3 => add_employee::launch(),
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }

        println!("Do you want to run another exercise? (y/n)");
        match utils::read_input().trim() {
            "y" => continue,
            _ => break,
        }
    }
}
