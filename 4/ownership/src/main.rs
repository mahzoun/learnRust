fn main() {
    {
        let s = "hello";
        let mut ss = String::from(s);
        ss.push_str(", world!");
        let _s2 = ss.clone();
        println!("{}", ss);
    }
    let s = String::from("Hello!");
    takes_ownership(s);
    let s = gives_ownership();
    println!("{}", s);
    let (s2, len) = calculate_len(s);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("From give function");
    some_string
}

fn calculate_len(some_string: String) -> (String, usize) {
    let l = some_string.len();
    (some_string, l)
}