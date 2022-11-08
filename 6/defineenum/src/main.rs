#[derive(Debug)]

enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    Changecolor(i32, i32, i32),
}
impl message {
    fn call(&self) {

    }
}
fn main() {
    let home = IPAddrKind::V4(127, 0, 0, 1);
    let loopback = IPAddrKind::V6(String::from("::1"));
    let m = message::Write(String::from("Hello!"));
    m.call();
    let some_number = Some(5);
    let another_number = Some(6);
    let no_number: Option<i32> = None;
}

fn route(ip_kind: IPAddrKind) {

}