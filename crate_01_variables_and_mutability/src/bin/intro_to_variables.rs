fn main() {
    println!("========== intro_to_variables ==========");
    let apples = 20;
    let oranges = 30;

    let _add_fruits = apples + oranges;

    // compiler directives
    #[allow(unused_variables)]
    let sub_fruits = apples - oranges;
    let mul_fruits = apples * oranges;
    let div_fruits = apples / oranges;
}
