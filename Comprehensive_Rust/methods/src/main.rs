#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self {name: String::from(name), laps: Vec::new()}
    }

    fn add_lap(&mut self, lap:i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for lap in &self.laps {
            println!("{}", lap);
        }
    }

    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race  = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(80);
    race.finish();
}