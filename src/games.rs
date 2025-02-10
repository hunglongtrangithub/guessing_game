mod christmas_lyrics;
mod fibonacci;
mod guessing_game;
mod temp_converter;

use crate::utils;

pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Select the program to run:");
        println!("1. Guessing Game");
        println!("2. Temperature Converter");
        println!("3. Fibonacci Series");
        println!("4. Christmas Carol Lyrics");
        println!("0. Exit");

        let selection = utils::read_input();
        let selection = match selection.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match selection {
            1 => guessing_game::launch(),
            2 => temp_converter::launch(),
            3 => fibonacci::launch(),
            4 => christmas_lyrics::launch(),
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }

        println!("Do you want to run another game? (y/n)");
        match utils::read_input().trim() {
            "y" => continue,
            _ => break,
        }
    }
}
