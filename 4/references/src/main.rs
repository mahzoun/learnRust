fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", &r1, &r2);
    let len = calculate_length(&mut s);
    println!("length of '{}' is {}", s, len);
    let ref_to_nothing = dangle();
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world!");
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}