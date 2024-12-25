struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn builduser(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}

fn main() {
    let user1 = builduser(String::from("function@mail.com"),String::from("function user"));
    println!("{}",user1.username);
    println!("{}",user1.email);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.active);
}
