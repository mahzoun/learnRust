#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: i8,
    points: i8,
}

fn main() {
    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = String::from("jack");
    println!("{:?}, {:?}", p1, p2);
}