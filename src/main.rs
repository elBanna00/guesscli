use std::io;

use colored::Colorize;
fn main() {
    println!(
        "{}",
        "Welcome to Guesscli!\nPlease input your guess:".green()
    );
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess, Please try again");
    println!("You guessed: {}", guess);
}
