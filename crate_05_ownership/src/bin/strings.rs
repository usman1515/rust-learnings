fn main() {
    println!("\x1b[0;32m========== dynamic strings ========== \x1b[0m\n");
    let _name: &str = "john";

    // empty string stored on the heap at runtime
    let some_text: String = String::new();
    println!("some text: {}", some_text);

    // string of fixed size stored on the heap at runtime
    let candy: String = String::from("Butterscotch");
    println!("candy: {}", candy);

    println!("\x1b[0;32m========== push_str method on strings ========== \x1b[0m\n");
    let mut name: String = String::from("John");
    println!("original name:    {}", name);
    name.push_str(" ");
    name.push_str("Doe");
    println!("updated name:     {}", name);
}
