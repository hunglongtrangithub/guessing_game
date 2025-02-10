use crate::utils;
mod linked_list;

pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Data structures and algorithms");
        println!("1. Linked List");
        println!("2. Stack");
        println!("3. Queue");
        println!("4. Binary Search Tree");
        println!("5. Graph");
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
            1 => linked_list::launch(),
            2 => println!("Stack"),
            3 => println!("Queue"),
            4 => println!("Binary Search Tree"),
            5 => println!("Graph"),
            0 => return,
            _ => {
                println!("Invalid selection");
                return;
            }
        }
    }
}
