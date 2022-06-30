fn main() {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut c: u128;
    for _index in 1..100 {
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}
