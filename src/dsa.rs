use crate::utils;
mod deque;
mod heap;
mod stack;

pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Data structures and algorithms");
        println!("1. Stack");
        println!("2. Deque");
        println!("3. Heap");
        println!("4. Binary Search Tree");
        println!("0. Back");

        let selection = crate::utils::read_input();
        let selection = match selection.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        match selection {
            1 => stack::launch(),
            2 => deque::launch(),
            3 => heap::launch(),
            4 => println!("Graph"),
            0 => return,
            _ => {
                println!("Invalid selection");
                return;
            }
        }
    }
}
