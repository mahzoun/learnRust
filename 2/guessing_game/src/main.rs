use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret);
    println!("Please enter the guest.");
    let mut guest = String::new();
    io::stdin()
        .read_line(&mut guest)
        .expect("Failed to read the line");
    println!("You guessed: {}", guest);
}
