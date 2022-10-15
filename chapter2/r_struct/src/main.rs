struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u32,
}

fn main() {
    let user1 = build_user(String::from("some@example.com"), String::from("ABC"));

    let user2 = User {
        email: String::from("another@some.com"),
        ..user1
    };
    println!("user.username:{}", user2.username);
    println!("user.username:{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        _active: true,
        _sign_in_count: 1,
    }
}
