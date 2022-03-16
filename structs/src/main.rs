struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someonename9732"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("raBeta");
    let user2 = build_user(String::from("my@test.mail"), String::from("someuser123"));

    println!("{} {}", user1.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
