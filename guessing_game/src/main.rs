use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guessed number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is {secret_number}");

    let mut guesss = String::new();

    io::stdin()
        .read_line(&mut guesss)
        .expect("Failed to read line");

    println!("You guessed: {guesss}");
}
