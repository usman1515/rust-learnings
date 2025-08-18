use std::os::unix::process;

fn main() {
    println!("\x1b[0;32m ========== project ========== \x1b[0m\n");

    // Define a `color_to_number` function that accepts a 'color' parameter (a string). Use if,
    // else if, and else statements to return a corresponding numeric value based on the following
    // rules:
    // 1. If the color is "red", return 1.
    // 2. If the color is "green", return 2.
    // 3. If the color is "blue", return 3.
    // 4. If the color is any other string, return 0.

    println!("Color is red:     {}", color_to_number_if_else("red"));
    println!("Color is green:   {}", color_to_number_if_else("green"));
    println!("Color is blue:    {}", color_to_number_if_else("blue"));
    println!("Color is orange:  {}", color_to_number_if_else("orange"));

    // Refactor the function above to use the `match` statement instead of if, else if, and else.

    println!();
    println!("Color is red:     {}", color_to_number_match("red"));
    println!("Color is green:   {}", color_to_number_match("green"));
    println!("Color is blue:    {}", color_to_number_match("blue"));
    println!("Color is orange:  {}", color_to_number_match("orange"));

    // Define a `factorial` function that calculates the factorial of a number. The factorial is
    // the product of multiplying a number by every incremental number leading up to it, starting
    // from 1. Examples:
    // The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120 so factorial(5) should return 120.
    // The factorial of 4 is 4 * 3 * 2 * 1 = 24 so factorial(4) should return 24.
    let mut num: i32 = 5;

    // Implement two solutions/functions for the problem.
    // The first solution should not use recursion.
    println!("\nFactorial loop of {} is: {}\n", num, factorial_loop(num));

    // The second solution should use recursion.
    num = 6;
    println!("Factorial recu of {} is: {}\n", num, factorial_recur(num));
}

fn color_to_number_if_else(color: &str) -> i32 {
    if color == "red" {
        return 1;
    }
    else if color == "green" {
        return 2;
    }
    else if color == "blue" {
        return 3;
    }
    else {
        return 0;
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => return 1,
        "green" => return 2,
        "blue" => return 3,
        _ => return 0,
    }
}

fn factorial_loop(num: i32) -> i32 {
    let mut fact: i32 = 1;
    for i in 1..=num {
        fact *= i;
    }
    return fact;
}

fn factorial_recur(num: i32) -> i32 {
    if num <= 1 {
        return 1;
    }
    else {
        return num * factorial_recur(num-1);
    }
}

