use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        // mut means this variable is muttable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the previous value of guess with a new one
        // Shadowing lets us reuse guess variable name rather than forcing us to create two unique variables
        // Such as guess_str and guess. This can be used to convert a type with another type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN!!!");
                break;
            }
        }
    }
}
