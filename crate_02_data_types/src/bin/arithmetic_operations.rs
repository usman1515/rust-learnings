fn main() {
    println!("\x1b[0;32m ========== arithmetic operations ========== \x1b[0m\n");

    let num1: f32 = 20.0;
    let num2: f32 = 30.0;

    let add: f32 = num1 + num2;
    let sub: f32 = num1 - num2;
    let mul: f32 = num1 * num2;
    let div: f32 = num1 / num2;
    let rem: f32 = num1 % num2;

    println!("add:  {add:.5}");
    println!("sub:  {sub:.5}");
    println!("mul:  {mul:.5}");
    println!("div:  {div:.5}");
    println!("rem:  {rem:.5}");

    // augmented assignment variable
    let mut year: i32 = 2025;
    year = year + 1;
    println!("year: {}", year);
    year += 1;
    println!("year: {}", year);
    year -= 2;
    println!("year: {}", year);
}
