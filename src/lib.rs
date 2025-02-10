pub mod dsa;
pub mod exercises;
pub mod games;
pub mod utils;

pub fn launch() {
    loop {
        utils::clear_screen();
        // check if user wants to play a game or solve an exercise
        println!("Do you want to play a game or solve an exercise?");
        println!("1. Play a game");
        println!("2. Solve an exercise");
        println!("3. Data structures and algorithms");
        println!("0. Exit");
        let game_or_exercise = utils::read_input();

        let game_or_exercise = match game_or_exercise.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match game_or_exercise {
            1 => games::launch(),
            2 => exercises::launch(),
            3 => dsa::launch(),
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }
    }
    println!("Thanks for playing! See you next time!")
}
