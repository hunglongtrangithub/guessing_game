use std::io;
use std::io::Write;
use termion::clear;

/// Clear the screen
pub fn clear_screen() {
    print!("{}", clear::All);
    io::stdout().flush().expect("Failed to flush stdout");
}

/// Read input from the user
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
