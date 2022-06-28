use std::io;

fn main() {
    let c = '😻';
    println!("{}", c);
    let tup: (char, u128) = ('*', 0xffffffffffffffffffffffffffffffff);
    println!("{}", tup.1);
    let a: [i32; 5] = [0, 0, 0, 0, 0];
    let b = [0; 10];
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index not a number");

    let element = b[index];
    println!("the element is {}", element);
}
