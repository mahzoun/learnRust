use std::fs::File;
use std::io::Read;

fn main() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut content = String::new();
            if let Ok(bytes) = file.read_to_string(&mut content) {
                println!("Dear Diary {content} {bytes}");
            } else {
                println!("Cannot read file");
            }
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}