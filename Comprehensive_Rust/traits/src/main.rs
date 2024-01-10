#[derive(Debug)]
struct Cat {
    name: String,
    age: i32,
}

struct Dog {
    name: String,
    age: i32,
}

trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Hallo! {}", self.talk());
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Miew, my name is {}", self.name)
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Wuff, my name is {}", self.name)
    }
}


fn main() {
    let cat = Cat {name: String::from("Hank"), age: 1222};
    let dog = Dog {name: String::from("Walter"), age: 2111};
    cat.greet();
    dog.greet();
}
