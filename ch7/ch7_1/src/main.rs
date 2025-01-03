struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User{
        username: String::from("test user"),
        email: String::from("info@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{}",user1.username);
    println!("{}",user1.email);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.active);
}
