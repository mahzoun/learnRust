enum Result {
    ok(i32),
    error(String),
}

fn divide_in_two(n: i32) -> Result {
    if n%2 == 0 {
        Result::ok(n/2)
    } else {
        Result::error("{n} is not even".to_string())
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::ok(half) => println!("{half}"),
        Result::error(msg) => println!("{msg}"),
    }
}