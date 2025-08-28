fn main() {
    println!("\x1b[0;32m========== references and borrowing ========== \x1b[0m\n");
    let my_stack_value: i32 = 10;
    let my_int_ref: &i32 = &my_stack_value;   // stack addr of my_stack_value stored in my_int_ref

    println!("my_stack_value:   {}", my_stack_value);
    println!("my_int_ref:       {}\n", my_int_ref);

    let my_heap_value: String = String::from("Mercedes");
    let my_heap_ref: &String = &my_heap_value;

    println!("my_heap_value:    {}", my_heap_value);

    println!("\x1b[0;32m========== dereferencing ========== \x1b[0m\n");
    println!("my_heap_ref:      {}", *my_heap_ref);     // standard way of printing a reference
    println!("my_heap_ref:      {}", my_heap_ref);      // print trait is smart enough to know youre printing a reference


}

