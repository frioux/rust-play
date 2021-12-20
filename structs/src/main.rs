#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
}

fn build_user(email: String, username: String) -> User {
    let active = true;
    User {
        email,
        username,
        active,
        sign_in_count: 1,
    }
}
