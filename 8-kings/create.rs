use std::io::Write;

struct King {
    name: String,
    next: i32,
}

impl King {
    fn new(name: String, next: i32) -> King {
        King { name, next }
    }
}

fn main() {
    let kings = vec![
        King::new("Leopold I".to_string(), 2),
        King::new("Leopold II".to_string(), 3),
        King::new("Albert I".to_string(), 4),
        King::new("Leopold III".to_string(), 5),
        King::new("Baudouin".to_string(), 6),
        King::new("Albert II".to_string(), 7),
        King::new("Philippe".to_string(), -1),
    ];

    let mut file = std::fs::File::create("kings.dat").unwrap();

    for king in &kings {
        let mut name = king.name.as_bytes().to_vec();
        name.resize(16, 0);
        let next = king.next.to_ne_bytes();

        file.write_all(&name).unwrap();
        file.write_all(&next).unwrap();
    }

    drop(file);

    println!("File created successfully!");
}
