fn main() {
    let mut string = String::new();

    println!("Enter a word: ");
    
    std::io::stdin().read_line(&mut string).expect("Failed to read line");

    let string = string.trim();

    let string_length = string.len();
    let mut is_palindrome = true;

    for (i, c) in string.chars().enumerate() {
        if c != string.chars().nth(string_length - i - 1).unwrap() {
            is_palindrome = false;
            break;
        }
    }

    println!("{} is {} a palindrome", string, if is_palindrome { "" } else { "not" });
}
