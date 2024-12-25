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
    let user2 = User{
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1
    };
    println!("{}",user1.username);
    println!("{}",user1.email);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.active);
    println!("--------------------");
    println!("{}",user2.username);
    println!("{}",user2.email);
    println!("{}",user2.sign_in_count);
    println!("{}",user2.active);
}
