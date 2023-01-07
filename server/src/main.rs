use std::{cmp::Ordering, io};

use controllers::test::FullName;
use rand::Rng;

mod controllers;
mod models;
mod services;

#[derive(Debug)]
struct Test {
    pub name: String,
    pub other_names: FullName,
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Guess the number!");
        println!("Please input your guess.");

        println!("{}", u8::MAX);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().parse::<u32>();

        match guess {
            Ok(guess) => {
                println!("You guessed: {}", guess);

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => println!("You win!"),
                }

                let full_name = controllers::test::Controllers::test("Test 23".to_string());

                let abc = Test {
                    name: full_name.name.clone(),
                    other_names: full_name,
                };

                println!("{:#?}", abc)
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
