#[derive(Debug)]
enum WebEvent{
    PageLoad,
    KeyPress(char),
    Click{x: u32, y: u32},
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page is loaded"),
        WebEvent::KeyPress(c) => println!("Key {c} is pressed"),
        WebEvent::Click{x, y} => println!("Clicked at {x}, {y}"),
    }

}
fn main() {
    let load = WebEvent::PageLoad;
    let keypress = WebEvent::KeyPress('x');
    let clicked = WebEvent::Click{x: 10, y: 20};
    inspect(load);
    inspect(keypress);
    inspect(clicked);
}
