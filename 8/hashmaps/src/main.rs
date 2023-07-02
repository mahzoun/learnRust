use std::collections::HashMap;
fn main() {
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10.1 );
    score.insert(String::from("Red"), 20.0);
    let sc = score.get(&String::from("Blue")).copied().unwrap_or(0.0);
    score.insert(String::from("Red"), 300.0);
    score.entry(String::from("Yellow")).or_insert(48.1);
    for (key, value) in &score {
        println!("{key}: {value}");
    }    
    let word = "bana nana bana bana nana hi ";
    for w in word.split_whitespace() {
        let count = score.entry(w.to_string()).or_insert(0.0);
        *count += 1.0;
    }
    println!("{:?}", score);
}
