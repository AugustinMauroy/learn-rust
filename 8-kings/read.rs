use std::io::Read;

struct King {
    name: String,
    next: i32,
}

fn main() {
    let mut file = std::fs::File::open("kings.dat").unwrap();

    let mut king = King {
        name: String::new(),
        next: 0,
    };

    while {
        // read name
        let mut name = [0u8; 16];
        file.read_exact(&mut name).unwrap();
        king.name = String::from_utf8(name.to_vec()).unwrap();
        println!("Name: {}", king.name);

        // read next
        let mut next = [0u8; 4];
        file.read_exact(&mut next).unwrap();
        king.next = i32::from_ne_bytes(next);

        king.next != -1
    } {}

    drop(file);
}
