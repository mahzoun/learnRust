fn collatz(a:u128) -> u128 {
    let x = match a {
        1 => 1,
        a if a%2 == 0 => collatz(a/2),
        a if a%2 != 0 => collatz(a*3 + 1), 
        _ => 0,
    };
    x
}
fn main() {
    let _x = collatz(1313131313131313131313131313131313131);
    println!("{_x}");
}
