mod guessgame;
mod basics;
mod myutils {
    pub mod stuff;
}
use myutils::stuff;

fn main() {
    basics::basics();
    println!();
    stuff::collections();
    stuff::error_handling();
    println!();
    guessgame::guess_game();
}
