struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 0,
    };

    user1.email = String::from("newemail@example.com");
    println!("Updated email: {}", user1.email);

    let user2 = build_user(String::from("hisemail@example.com"), String::from("hisusername123"));
    println!("User2 - Username: {}, Email: {}", user2.username, user2.email);

    let user3 = copy_user(user1);
    println!("User3 - Username: {}, Email: {}", user3.username, user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

// Note that user received is no longer valid after this function call because it moves ownership of strings.
fn copy_user(user: User) -> User {
    User {
        active: true,
        sign_in_count: 0,
        ..user
    }
}
