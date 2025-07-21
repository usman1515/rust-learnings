fn main() {
    println!("========== interpolation_with_curly_braces ==========");

    let apples = 20;
    let oranges = 30;

    let _add_fruits = apples + oranges;

    // interpolation with curly braces
    println!("Original apples:  {}", apples);
    println!("Original oranges: {}", oranges);

    println!("Updated apples:   {}", apples+10);

    println!("Original apples:  {apples}");
    println!("Original oranges: {oranges}");

    println!("There are {} apples and {} oranges.", apples, oranges);

    println!("There are {0} red apples, {1} oranges, {0} green apples.", apples, oranges);
}
