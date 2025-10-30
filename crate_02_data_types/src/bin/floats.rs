fn main() {
    println!("========== floats ==========\n");

    let pi: f32 = 22.0 / 7.0;
    println!("Pi (32 bit): {}", pi);

    let pi: f64 = 22.0 / 7.0;
    println!("Pi (64 bit): {}", pi);

    println!("Pi (64 bit) floor: {}", pi.floor());
    println!("Pi (64 bit) ceil:  {}", pi.ceil());
    println!("Pi (64 bit) round: {}", pi.round());

    // formatting floats with float specifiers
    println!("Pi (64 bit): {pi:.4}");
    println!("Pi (64 bit): {:.4}", pi);
}

