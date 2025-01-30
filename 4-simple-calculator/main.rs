/**
 * 1. `self`` - Imports the io module itself
 * 2. `Write`` - Imports the Write trait from the io module
 */
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut number: u32 = input.trim().parse().expect("Failed to parsed the number");

    number = (number * 2) + 5;

    println!("The final number is: {}", number);
}
