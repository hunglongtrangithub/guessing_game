use guessing_game::{exercises, games, utils};

fn main() {
    loop {
        utils::clear_screen();
        // check if user wants to play a game or solve an exercise
        println!("Do you want to play a game or solve an exercise?");
        println!("1. Play a game");
        println!("2. Solve an exercise");
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
            1 => games::launch_games(),
            2 => exercises::launch_exercises(),
            0 => break,
            _ => {
                println!("Invalid selection");
                continue;
            }
        }
    }
    println!("Thanks for playing! See you next time!")
}
