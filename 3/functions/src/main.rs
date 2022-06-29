use std::io;
fn main() {
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read x");
    let y = x.trim().parse().expect("please enter a number");

    println!("{}", factorial(y));
}

fn factorial(x: u128) -> u128 {
    if x == 0 {
        1
    } else {
        x*factorial(x-1)
    }

}
