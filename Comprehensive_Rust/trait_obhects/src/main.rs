struct Cat{
    name: String,
}

struct Dog {
    name: String,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Dog name is {}", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Cat name is {}", self.name)
    }
}

fn main() {
    let pets : Vec<Box<dyn Pet>> = vec![
        Box::new(Cat {name: String::from("Waaaaaaaaaaaaaaaaaaa6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666677777777777777777777777777777777777777777777777777777777777777777777777777777777777777777777778888888888888888888888888888888888888888888888xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   lter")}),
        Box::new(Dog {name: String::from("Jessie")}),
    ];
    for pet in pets {
        println!("Pet name is {}", pet.talk());
    }
    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}
