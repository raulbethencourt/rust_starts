struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someonename9732"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("raBeta");
    let user2 = build_user(String::from("my@test.mail"), String::from("someuser123"));

    // struct update syntax
    let user3 = User {
        username: String::from("difool"),
        ..user1
    };

    println!("{} {} {}", user1.username, user2.email, user3.email);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
