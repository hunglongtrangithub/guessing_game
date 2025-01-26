mod christmas_lyrics;
mod fibonacci;
mod guessing_game;
mod temp_converter;
mod utils;

use christmas_lyrics::christmas_lyrics;
use fibonacci::fibonacci;
use guessing_game::guessing_game;
use std::io;
use temp_converter::temp_converter;

pub fn launch_games() {
    loop {
        utils::clear_screen();
        println!("Select the program to run:");
        println!("0. Guessing Game");
        println!("1. Temperature Converter");
        println!("2. Fibonacci Series");
        println!("3. Christmas Carol Lyrics");
        println!("4. Exit");

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        let selection = match selection.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match selection {
            0 => guessing_game(),
            1 => temp_converter(),
            2 => fibonacci(),
            3 => christmas_lyrics(),
            4 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }

        println!("Do you want to run another program? (y/n)");
        let mut run_again = String::new();
        io::stdin()
            .read_line(&mut run_again)
            .expect("Failed to read line");
        if run_again.trim() != "y" {
            break;
        }
    }

    println!("Goodbye!");
}
