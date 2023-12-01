fn main() {
    let arr = [1, 2, 3];
    match arr {
        [0, y, z] => println!("{y}, {z}"),
        [.., 3] => println!("Bingo"),
        _ => println!("fuck"),
    }
}
