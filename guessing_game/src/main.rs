use std::io;


fn main() {
    println!("Guess the number!");
    println!("Please input you guessed number: ");

    let mut guesss = String::new();

    io::stdin()
        .read_line(&mut guesss)
        .expect("Failed to read line");

    println!("You guessed: {guesss}");
}
