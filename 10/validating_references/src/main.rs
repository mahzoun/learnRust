use std::fmt::Display;

fn longest_with_annotation<'a, T: Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s = longest_with_annotation("hi", "bye", "wow");
    println!("{:?}", s);
}