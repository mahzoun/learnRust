fn collatz(a:u128) -> u128 {
    if a == 1 {
        println!("Done!");
        return 1;
    } else if a%2 == 0 {
        println!("{a}");
        return collatz(a/2);
    } else {
        println!("{a}");
        return collatz(3*a + 1);
    }
}
fn main() {
    let _x = collatz(1313131313131313131313131313131313131);
}
