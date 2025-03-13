use std::io;

fn main() {
    let mut input = String::new();
    let mut tokens: Vec<String> = Vec::new();

    println!("Enter your expression: ");
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }
        if input == "=" {
            break;
        };

        if is_number(input) || is_valid_operator(input) {
            tokens.push(input.to_string());
        } else {
            println!(
                "Invalid input: '{}'. Please enter a number or operator (+, -, *, %)",
                input
            );
            continue;
        }
    }

    let result = evaluate(&tokens);
    println!("Result: {}", result);
}

fn is_valid_operator(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "%") && op.len() == 1
}

fn is_number(string: &str) -> bool {
    string.parse::<f64>().is_ok()
}

fn evaluate(tokens: &[String]) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        if is_number(token) {
            stack.push(token.parse::<f64>().unwrap());
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "%" => a % b,
                _ => panic!("Invalid operator"),
            };
            stack.push(result);
        }
    }

    return stack.pop().unwrap(); // pop().unwrap() will return the last element of the stack
}

