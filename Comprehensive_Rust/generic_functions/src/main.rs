fn sum<T: std::ops::Add<Output = T>> (a: T, b: T) -> T {
    a + b
} 
fn main() {
    let a:i32 = -23;
    let b:i32 = 920801;
    let c = 2.46;
    let d = 4.000000000000000000001;
    println!("{}, {}", sum(a, b), sum(c, d));
}
