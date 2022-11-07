#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "Area of Rectangle {},{} is {} and width is {}",
        rect.width,
        rect.height,
        rect.area(),
        rect.width()
    );
    if rect1.can_hold(&rect2) {
        println!("Rectangle {:#?} can hold Rectangle {:#?}", rect1, rect2);
    } else {
        println!(
            "Rectangle {:#?} can *NOT* hold Rectangle {:#?}",
            rect1, rect2
        );
    }
    let sq = Rectangle::square(20);
}
