fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn main() {
    let x: i32 = 24;
    let y = 25i64;
    println!("{x} * {y} = {}",multiply(x, y.try_into().unwrap()));
}
