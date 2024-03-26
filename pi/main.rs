fn main() {
    let mut pi: f64 = 0.0;

    for i in 0..10000000i32 {
        if i % 2 == 0 {
            pi += 4.0 / (2 * i + 1) as f64;
        } else {
            pi -= 4.0 / (2 * i + 1) as f64;
        }
        println!("PI is {}", pi);
    }

    println!("PI is {}", pi);
}
