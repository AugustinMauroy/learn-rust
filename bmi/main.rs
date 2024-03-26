fn main() {
    let mut weight: String;
    let mut height: String;

    println!("What is your weight in kg?");
    weight = String::new();
    std::io::stdin().read_line(&mut weight).unwrap();

    println!("What is your height?");
    height = String::new();
    std::io::stdin().read_line(&mut height).unwrap();

    let weight: f32 = weight.trim().parse().unwrap();
    let height: f32 = height.trim().parse().unwrap();

    // Convert height to meters if it is in cm
    // it's work because if height is less than 3, it's in cm
    let height = if height > 3.0 { height / 100.0 } else { height };

    let bmi = weight / (height * height);

    println!("Your BMI is {}", bmi);
    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 25.0 {
        println!("Normal");
    } else if bmi < 30.0 {
        println!("Overweight");
    } else {
        println!("Obese");
    }
}
