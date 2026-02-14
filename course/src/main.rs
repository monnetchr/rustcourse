use basics;
use myutils::stuff;
use myutils::testing;
//use guessgame

fn main() {
    basics::basics();
    println!();
    stuff::collections();
    stuff::lifetimes();
    testing::add(2, 2);
    println!();
    //stuff::error_handling();
    //guessgame::guess_game();
    intmut::intmut();
    intmut::weakrefs();
}
