use std::time::SystemTime;

fn main() {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut rng = StdRng::seed_from_u64(seed);
    let random_number: u32 = rng.gen_range(0..=100);

    println!("Nombre aléatoire : {}", random_number);
}
