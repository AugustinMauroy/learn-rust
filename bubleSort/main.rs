use std::io;

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    println!("How many elements you want to enter?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    for i in 0..n {
        println!("Enter element {}:", i + 1);
        let mut ele = String::new();
        io::stdin().read_line(&mut ele).expect("Failed to read line");
        let ele: i32 = ele.trim().parse().expect("Please type a number!");
        vec.push(ele);
    }

    println!("Before sorting: {:?}", vec);

    for i in 0..n {
        for j in 0..n - i - 1 {
            if vec[j as usize] > vec[(j + 1) as usize] {
                vec.swap(j as usize, (j + 1) as usize);
            }
        }
    }

    println!("After sorting: {:?}", vec);
}
