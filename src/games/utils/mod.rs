use std::io;
use std::io::Write;
use termion::clear;

pub fn clear_screen() {
    print!("{}", clear::All);
    io::stdout().flush().unwrap();
}
