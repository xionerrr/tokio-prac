use std::io;

mod controllers;
mod models;
mod services;

fn main() {
    let mut guess = String::new();

    println!("Guess the number!");
    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let c = controllers::test::Controllers(3, 5);

    let d = c.add();
}
