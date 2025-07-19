fn main() {
    println!("========== variable_shadowing ==========");

    // variable shadowing
    let grams_of_protein = "100.345";
    println!("Total grams of protein (str): {}", grams_of_protein);

    let grams_of_protein = 100.345;
    println!("Total grams of protein (flt): {}", grams_of_protein);

    let grams_of_protein = 100;
    println!("Total grams of protein (int): {}", grams_of_protein);
}
