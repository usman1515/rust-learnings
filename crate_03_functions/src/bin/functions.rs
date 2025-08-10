fn open_store_default() {
    println!("Opening my pizza store.");
}
fn bake_pizza() {
    println!("Baking a pizza.");
}
fn sell_pizza() {
    println!("Thank you for your purchase.");
}

fn open_store_in(neighborhood: &str) {
    println!("Opening my pizza store in {}.", neighborhood);
}
fn baking_pizza(order: i32, topping: &str) {
    println!("Baking {} pizzas with {} toppings.", order, topping);
}

fn num_square(num: i32) -> i32 {
    return num * num;
}
fn num_cube(num: i32) -> i32 {
    return num * num * num;
}
fn num_power(num: i32, power: u32) -> i32 {
    num.pow(power)
}

fn main() {
    println!("\x1b[0;32m========== introduction to functions ========== \x1b[0m\n");
    open_store_default();
    bake_pizza();
    sell_pizza();

    open_store_default();
    bake_pizza();
    sell_pizza();

    println!("\x1b[0;32m========== parameters and arguments ========== \x1b[0m\n");
    open_store_in("Heidelberg");
    baking_pizza(5, "mushrooms");

    println!("\x1b[0;32m========== explicit return values ========== \x1b[0m\n");
    let mut a: i32 = 5;
    println!("Square of {} =    {}", a, num_square(a));
    println!("Cube of {} =      {}", a, num_cube(a));

    println!("\x1b[0;32m========== implicit return values ========== \x1b[0m\n");
    a = 6;
    println!("Num {} of power {} = {}", a, 3, num_power(a, 3));
    println!("Num {} of power {} = {}", a, 5, num_power(a, 5));

    println!("\x1b[0;32m========== blocks in function ========== \x1b[0m\n");

    let product: i32 = {
        let b: i32 = 7;
        println!("Multiply {} * {} = {}", a, b, a*b);
        a*b
    };
    println!("Product = {}", product);

}

