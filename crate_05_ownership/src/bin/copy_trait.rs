fn main() {
    println!("\x1b[0;32m========== copy trait ========== \x1b[0m\n");

    let mut time: i32 = 2025;
    let mut year: i32 = time;
    println!("Time: {}", time);
    println!("Year: {}", year);

    time = 2026;
    println!("Time: {}", time);
    println!("Year: {}", year);

    year = 2027;
    println!("Time: {}", time);
    println!("Year: {}", year);
}

