use std::collections::HashMap;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(45);
    v.push(2);
    v.push(1);
    v.push(78);
    v.push(-983);
    v.push(763);
    v.push(0);
    v.push(0);
    v.push(0);  
    v.sort();
    let mut counter = HashMap::new();
    for i in &v {
        let count = counter.entry(i).or_insert(0);
        *count += 1;
    }
    let mut modv = 0;
    let mut modc = 0;
    for (key, value) in counter {
        if value > modc {
            modc = value;
            modv = *key;
        }
    }
    println!("Median is:{}\nMod is:{}, {}", v[(v.len())/2], modv, modc);
}
