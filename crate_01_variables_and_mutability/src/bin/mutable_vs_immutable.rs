fn main() {
    println!("\x1b[0;32m ========== mutable_vs_immutable ========== \x1b[0m\n");

    // mutable vs immutable
    let mut grapes = 60;
    println!("Original grapes:  {}", grapes);

    grapes = 70;
    println!("Updated grapes:   {}", grapes);
}
