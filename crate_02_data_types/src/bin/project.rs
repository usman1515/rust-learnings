fn main() {

    // Declare an i32 variable assigned to 1337. Use the underscore character to add a visual
    // separator between the numbers.
    let _num1: i32 = 1337;

    // Cast the i32 to an i16 integer and assign the result to a separate variable.
    let num2: i16 = _num1 as i16;

    // Declare a floating-point value of your choosing. Print out the number with 3 digits of
    // precision.
    let exp: f64 = 2.71828;
    println!("Exponent: {:3}", exp);

    // Declare a 'with_milk' variable set to a Boolean. Declare a 'with_sugar` variable set to a
    // Boolean.
    let mut with_milk: bool;
    let mut with_sugar: bool;

    // Declare a 'is_my_type_of_coffee` variable. It should be set to true if the coffee has both
    // milk and sugar.
    with_milk = false;  with_sugar = false;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    println!("is_my_type_of_coffee: {}", is_my_type_of_coffee);

    with_milk = false;  with_sugar = true;
    println!("is_my_type_of_coffee: {}", with_milk && with_sugar);

    with_milk = true;  with_sugar = false;
    println!("is_my_type_of_coffee: {}", with_milk && with_sugar);

    with_milk = true;  with_sugar = true;
    println!("is_my_type_of_coffee: {}", with_milk && with_sugar);

    // Declare an `is_acceptable_coffee` variable. It should be set to true if the coffee has
    // either milk or sugar.
    with_milk = false;  with_sugar = false;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    println!("\nis_acceptable_coffee: {}", is_acceptable_coffee);

    with_milk = false;  with_sugar = true;
    println!("is_acceptable_coffee: {}", with_milk || with_sugar);

    with_milk = true;  with_sugar = false;
    println!("is_acceptable_coffee: {}", with_milk || with_sugar);

    with_milk = true;  with_sugar = true;
    println!("is_acceptable_coffee: {}", with_milk || with_sugar);

    // Declare an array with four i8 integers of your choosing Print out the array in its Debug
    // representation.
    let my_num_list: [i8; 4] = [1,2,3,4];

    println!("my_num_list: {:?}",   my_num_list);
    println!("my_num_list: {:#?}",  my_num_list);
    dbg!("my_num_list: {}",         my_num_list);

    // Declare a tuple consisting of the integer, float, a Boolean, and the array that you
    // previously declared. Print out the tuple in its Debug representation.
    let my_tuples: (i16, f64, bool, [i8; 4]) = (num2, exp, with_milk, my_num_list);
    dbg!("debugging my_tuples: {}", my_tuples);
    println!("pretty print my_tuples: {:#?}", my_tuples);
}
