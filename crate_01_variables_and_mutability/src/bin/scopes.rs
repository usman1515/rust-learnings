fn main() {
    println!("\x1b[0;32m ========== scopes ========== \x1b[0m\n");

    // scopes
    let coffee_price = 2.35;
    {
        println!("Coffee price nested scope:     {}", coffee_price);
        let coffee_price = 1.99;    // this creates a new variable. doesnt behave like scope.
        println!("Coffee price update scope:     {}", coffee_price);
    }
    println!("Coffee price outside scope:   {}", coffee_price);
}
