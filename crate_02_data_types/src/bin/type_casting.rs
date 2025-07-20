fn main() {
    println!("========== type casting ==========\n");

    let distance_u32: u32 = 250;
    println!("Distance (32 bit): {}", distance_u32);

    let distance_u8: u8 = distance_u32 as u8;
    println!("Distance (8  bit): {}", distance_u8);


    let pi_f64: f64 = 22.0/7.0;
    println!("Pi (f64): {}", pi_f64);

    let pi_f32: f32 = pi_f64 as f32;
    println!("Pi (f32): {}", pi_f32);

    let pi_u32: u32 = pi_f32 as u32;
    println!("Pi (u32): {:.3}", pi_u32);

    let pi_f32: f32 = pi_u32 as f32;
    println!("Pi (f32): {:.4}", pi_f32);
}


