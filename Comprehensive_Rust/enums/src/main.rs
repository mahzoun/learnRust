fn generate_random_number() -> i32 {
    4
}

#[derive(Debug)]
enum CoinFlip{
    HEAD,
    TAIL,
}

fn flip_coin() -> CoinFlip {
    if generate_random_number() % 2 == 0{
        CoinFlip::HEAD
    } else {
        CoinFlip::TAIL
    } 
}
fn main() {
    let a:CoinFlip = flip_coin();
    println!("Coin Flip is {:?}", a);
}
