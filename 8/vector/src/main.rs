#[derive(Debug)]
fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut vv = vec![1, 2, 3, 4];
    v.push(5);
    v.push(10);
    let x = v[1];
    println!("{x}");
    vv.push(10);
    let y: Option<&i32> = vv.get(10);
    match y {
        Some(y) => println!("{y}"),
        None => println!("fuck"),
    }
    vv.push(10);
    for t in &mut v {
        *t += 100000;
        println!("{t}");
    }
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let Row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Float(4.5),
        Spreadsheetcell::Text(String::from("fuck")),
    ];
}
