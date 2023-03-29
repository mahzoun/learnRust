fn main() {
    let config_max = Some(3u8);
    let mut counter = 0;
    if let Some(max) = config_max {
        println!("Maximum is {}, {:?}", max, config_max);
    } else {
        counter += 1;
    }
    println!("Counter is {}", counter)
}
