fn main() {
    println!("========== mutable_vs_immutable ==========");

    // mutable vs immutable
    let mut grapes = 60;
    println!("Original grapes:  {}", grapes);

    grapes = 70;
    println!("Updated grapes:   {}", grapes);
}
