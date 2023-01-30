use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input you guessed number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guesss = String::new();

        io::stdin()
            .read_line(&mut guesss)
            .expect("Failed to read line");

        let guesss: u32 = match guesss.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guesss}");

        match guesss.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
        }
    }
}
