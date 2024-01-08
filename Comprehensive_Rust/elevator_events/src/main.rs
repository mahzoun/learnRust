/// The car has arrived on the given floor.
#[derive(Debug)]
enum Direction {
    Up, 
    Down,
}
#[derive(Debug)]
enum Event {
    door_opened,
    door_closed,
    arrived(i32), 
    lobbypressed(Button),
}
#[derive(Debug)]
enum Button {
    CarFloor(i32),
    LobbyCall(i32, Direction),

}
fn car_arrived(floor: i32) -> Event {
    Event::arrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::door_opened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::door_closed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::lobbypressed(Button::LobbyCall(floor, dir))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::lobbypressed(Button::CarFloor(floor))
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}