use colored::*;
use rand::random_range;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("Guess a number between 1 and 100!");
    let secret_number = random_range(1..=100);
    loop {
        match user_input() {
            Ok(guess) => match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small.".red()),
                Ordering::Greater => println!("{}", "Too big.".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    break;
                }
            },
            Err(_) => {
                println!(
                    "{}",
                    "Please enter a valid number between 1 and 100!".cyan()
                );
                continue;
            }
        }
    }
}

fn user_input() -> Result<u32, std::num::ParseIntError> {
    let mut guess = String::new();
    println!("Enter your guess: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess.trim().parse();
}
