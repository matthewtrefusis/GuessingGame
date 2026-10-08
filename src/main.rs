use rand::{Rng, RngExt};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    // Create secret number
    let secret_number: u32 = rand::rng().random_range(1..=100);

    // Create counter to count number of guesses
    let mut count: u32 = 0;

    // Guessing loop
    loop {
        count += 1;

        println!("Input your guess");
        // Create guess string
        let mut guess: String = String::new();

        // Store user input in guess string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Covert guess string to an integer
        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");

        println!("You guessed {}", guess);

        // Selection to output wether number is higher, lower or correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("Correct!!!");
                println!("It took you {} guesses", count);
                break;
            }
        }
    }
}
