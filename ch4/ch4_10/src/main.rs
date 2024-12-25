fn main() {
    let s = String::from("hello");
    println!("{}",&s[0..2]);
    println!("{}",&s[0..=2]);
    println!("{}",&s[..2]);
    println!("{}",&s[3..s.len()]);
    println!("{}",&s[3..]);
    println!("{}",&s[..]);
}
