fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}
fn main() {
    let foo = 23456789;
    let pair = duplicate(foo);
    println!("{pair:?}");
}
