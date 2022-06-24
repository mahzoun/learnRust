use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please enter the guest.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
