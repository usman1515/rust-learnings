fn main() {
    println!("\x1b[0;32m ========== characters and arrays ========== \x1b[0m\n");

    let first_char = 'a';       // single quotes means its a char
    let _second_char = "a";     // double quotes makes it a string

    println!("Char is alphabetic:   {}", first_char.is_alphabetic());
    println!("Char is lowercase:    {}", first_char.is_lowercase());
    println!("Char is uppercase:    {}", first_char.is_uppercase());

    // arrays
    let mut num_list: [i32; 5] = [4, 8, 12, 16, 20];
    let colors: [&str; 3] = ["red", "blue", "green"];

    println!("array num_list len:   {}", num_list.len());
    println!("array colors len:     {}", colors.len());

    //reading from arrays
    println!("num_list[3]:          {}", num_list[2]);

    //writing to arrays
    num_list[2] = 24;
    println!("num_list[3]:          {}", num_list[2]);
}


