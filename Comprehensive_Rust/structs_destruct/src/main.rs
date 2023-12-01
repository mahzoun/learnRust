struct Foo {
    x: (u32, u32), 
    y: u32,
}
fn main() {
    let foo = Foo{x: (1,2), y: 3};
    match foo {
        Foo{x:(1, b), y: c} => println!("1, {b}, {c}"),
        Foo{y, ..} => println!("{y}"),
    }
}
