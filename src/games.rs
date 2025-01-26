use convert_case::{Case, Casing};
use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod utils {
    use std::io;
    use std::io::Write;
    use termion::clear;

    pub fn clear_screen() {
        print!("{}", clear::All);
        io::stdout().flush().unwrap();
    }
}

fn guessing_game() {
    utils::clear_screen();
    println!("Welcome to guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
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

fn temp_converter() {
    utils::clear_screen();
    println!("Temperature Converter");
    loop {
        println!("Enter the temperature in degrees:");
        let mut degrees = String::new();
        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");
        let degrees = match degrees.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Enter the unit to convert to (C/F):");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        match unit.trim().to_lowercase().as_str() {
            "f" => {
                let converted = 1.8 * degrees + 32.0;
                println!("{degrees} degrees C = {converted:.2} degrees F");
                break;
            }
            "c" => {
                let converted = (degrees - 32.0) / 1.8;
                println!("{degrees} degrees F = {converted:.2} degrees C");
                break;
            }
            _ => {
                println!("Invalid unit. Try again!")
            }
        }
    }
}

fn fibonacci() {
    utils::clear_screen();
    println!("Fibonacci Series");
    loop {
        println!("Enter the number of terms:");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n = match n.trim().parse::<u8>() {
            Ok(num) => {
                if num > 99 {
                    println!("Please enter a number less than 100");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let mut a: u128 = 0;
        let mut b: u128 = 1;
        let mut i: u8 = 0;
        while i < n {
            print!("{a} ");
            let c = a + b;
            a = b;
            b = c;
            i += 1;
        }
        println!();
        break;
    }
}

fn christmas_lyrics() {
    utils::clear_screen();
    println!("Christmas Carol Lyrics: Twelve Days of Christmas\n");
    let combined = [
        ("first", "a", "partridge in a pear tree"),
        ("second", "two", "turtle doves"),
        ("third", "three", "French hens"),
        ("fourth", "four", "calling birds"),
        ("fifth", "five", "golden rings"),
        ("sixth", "six", "geese a-laying"),
        ("seventh", "seven", "swans a-swimming"),
        ("eighth", "eight", "maids a-milking"),
        ("ninth", "nine", "ladies dancing"),
        ("tenth", "ten", "lords a-leaping"),
        ("eleventh", "eleven", "pipers piping"),
        ("twelfth", "twelve", "drummers drumming"),
    ];
    for i in 0..combined.len() {
        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            combined[i].0
        );
        for j in (0..=i).rev() {
            let lyric = if j == 0 {
                if i == 0 {
                    format!("{} {}.", combined[j].1, combined[j].2)
                } else if i == combined.len() - 1 {
                    format!("and {} {}!", combined[j].1, combined[j].2)
                } else {
                    format!("and {} {}.", combined[j].1, combined[j].2)
                }
            } else {
                format!("{} {},", combined[j].1, combined[j].2)
            };
            println!("{}", lyric.to_case(Case::Sentence));
        }
        println!();
    }
}

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
