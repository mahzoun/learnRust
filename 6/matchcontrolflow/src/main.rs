#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("LUCKY!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Coin from state {:?}!", state);
            25
        }
        _ => 0
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    value_in_cents(c);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
