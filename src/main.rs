mod guessgame;
mod basics;
use rustcourse::myutils::stuff;
use rustcourse::myutils::testing;

fn main() {
    basics::basics();
    println!();
    stuff::collections();
    stuff::lifetimes();
    testing::add(2, 2);
    println!();
    stuff::error_handling();
    guessgame::guess_game();
}
