#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    println!("Rectangle is {:?}", rect);
    println!(
        "The area of Trinagle is {}",
        area(&rect)
    );
    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}