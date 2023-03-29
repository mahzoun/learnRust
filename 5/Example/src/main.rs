#[derive(Debug)]

struct Rectangle {
    w: u32, 
    l: u32,
}

fn main() {
    let rect1 = Rectangle{w: 10, l:20};
    println!("{:#?}", rect1.area());
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.l
    }
}