#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Po bbbbbbbbbbbbbbbbbbbbb       int<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
}
