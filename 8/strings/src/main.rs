fn main() {
    let mut s = String::new();
    let ss = String::from("fuck fuck");
    let mut hello = String::from("你好");
    let moto = String::from("زن زندگی آزادی");
    hello.push_str("jkhdsنتیتا");
    println!("{hello}");
    let s1 = String::from("زن");
    let s2 = String::from("زندگی");
    let s3 = String::from("آزادی");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{s}");
    for c in s.chars() {
        println!("{c}");
    }
    for b in s.bytes() {
        println!("{b}");
    }
}
