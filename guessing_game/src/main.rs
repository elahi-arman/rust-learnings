extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        // Result can be either an Ok or Err
        //      if it's Ok, then it'll be returned into bytes
        //      if it's Err, then it'll be caught in the expect
        let bytes = io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Rust let's us shadow in this way, kinda dangerous
        // switching from an expect to a match express = move from
        // crashing on error to handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could't parse that number");
                continue;
            }
        };

        println!("You guessed: {}, taking up {} bytes", guess, bytes);

        // Ordering enum is either Less, Greater, Equal
        //      constructed of an arm - a pattern + code to execute
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
