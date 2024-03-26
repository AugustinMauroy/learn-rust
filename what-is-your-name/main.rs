use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap().expect("Failed to read line");
    println!("Hello, {}", name);
}
