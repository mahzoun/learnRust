struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    println!("user email is {}", user1.email);
    let user2 = User {
        email: String::from("someone2@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("someone3@example.com"),
        ..user2
    };
    println!("user 2 email is {}", user3.username);
}

fn BuildUser(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
