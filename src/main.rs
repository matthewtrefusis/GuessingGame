use rand::{Rng, RngExt};

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);

    
}
