use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name");

    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim();

    println!("Hello {} !", name);
}
