use std::fs::{self,File};
use std::io::{self,Read,ErrorKind};
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(& mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let file = File::open("hello.txt");
    match file {
        Ok(t) => t,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("Can't create file {:?}", e),
            },
            other_e => panic!("Error: {other_e}"),
        },
    };
    println!("{:?}", read_username_from_file());
    println!("{:?}", read_username_from_file_short());
    println!("{:?}", read_username_from_file_shorter());
    println!("{:?}", read_username_from_file_shortest());
}
