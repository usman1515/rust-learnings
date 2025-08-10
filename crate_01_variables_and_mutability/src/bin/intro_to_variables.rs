fn main() {
    println!("\x1b[0;32m ========== intro_to_variables ========== \x1b[0m\n");
    let apples = 20;
    let oranges = 30;

    let _add_fruits = apples + oranges;

    // compiler directives
    #[allow(unused_variables)]
    let sub_fruits = apples - oranges;
    let mul_fruits = apples * oranges;
    let div_fruits = apples / oranges;
}
