// Struct
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple-like Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like Struct
struct Empty {}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(user1.email, user1.username);

    let user3 = User {
        active: false,
        sign_in_count: 1337,
        ..user2
    };

    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);
    let it = Empty {};

    // numbered accessors
    println!("origin: {} {} {}", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
