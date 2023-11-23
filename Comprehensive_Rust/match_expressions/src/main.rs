fn main() {
    let t = match std::env::args().next().as_deref() {
        Some("cat") => println!("print stuff"),
        Some("ls") => println!("list stuff"),
        None => println!("no programm"),
        _ => println!("unknown program"),
    };
    println!("t:{:?}",t);
}
