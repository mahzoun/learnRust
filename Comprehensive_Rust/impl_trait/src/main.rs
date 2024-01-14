fn plus_one(x: impl Into<i32>) -> i32 {
    x.into() + 1
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}
fn main() {
    let many = plus_one(42_i8);
    println!("{many}");
    let many_more = plus_one(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(1);
    println!("debuggable: {debuggable:?}");
}
