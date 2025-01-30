use std::io;

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor for an empty person
    fn empty() -> Person {
        Person {
            name: String::new(),
            age: 0,
        }
    }

    fn introduce(&self) {
        println!("Hello {}, {}", self.name, self.age);
    }

    // Note the &mut self because we're modifying the instance
    fn ask_user(&mut self) {
        let mut input = String::new();
        println!("What's your name?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        self.name = input.trim().to_string();

        println!("What's your age?");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the age from the input
        self.age = input.trim().parse().expect("Please enter a valid number");
    }
}

fn main() {
    let mut user = Person::empty();

    user.ask_user();
    user.introduce();
}
