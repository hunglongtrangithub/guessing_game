use crate::utils;
use rand::Rng;
use std::cmp::Ordering;

pub fn launch() {
    utils::clear_screen();
    println!("Welcome to guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess:");

        let guess = utils::read_input();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
