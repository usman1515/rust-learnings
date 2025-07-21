fn main() {
    println!("========== logical operations ==========\n");

    // booleans
    let mut flag_ready: bool = true;

    println!("flag_ready:   {}", flag_ready);
    flag_ready = false;
    println!("flag_ready:   {}", flag_ready);

    let mut age: i32 = 17;
    let mut is_adult = 18 < age;
    println!("Person is 18+:    {}", is_adult);
    age = 25;
    is_adult = 18 < age;
    println!("Person is 18+:    {}", is_adult);

    // boolean inversion
    age = 20;
    let can_drive: bool = age > 18;
    let cant_drive: bool = !can_drive;
    println!("person of age {} can drive:   {}", age, can_drive);
    age = 15;
    println!("person of age {} cant drive:  {}", age, can_drive);

    // equality and inequality
    let num1: i32 = 25;
    let num2: i32 = 30;
    println!("is A ({}) == B ({}): {}", num1, num2, num1 == num2);
    println!("is A ({}) != B ({}): {}", num1, num2, num1 != num2);

    // logical operators
    let input_1: u8 = 0xde;
    let input_2: u8 = 0xad;

    let and_gate: u8 = input_1 & input_2;
    let or_gate: u8 = input_1 | input_2;
    let xor_gate: u8 = input_1 ^ input_2;
    println!("a ({:x}) & b ({:x}) = {:x}", input_1, input_2, and_gate);
    println!("a ({:x}) | b ({:x}) = {:x}", input_1, input_2, or_gate);
    println!("a ({:x}) ^ b ({:x}) = {:x}", input_1, input_2, xor_gate);

    // || and && can only work with boolean data types
    let state1: bool = true;
    let state2: bool = false;
    println!("state {} && state {} = {}", state1, state2, state1 && state2);
    println!("state {} || state {} = {}", state1, state2, state1 || state2);
}
