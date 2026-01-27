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
    stuff::lifetimes();
    println!();
    stuff::error_handling();
    guessgame::guess_game();
}
