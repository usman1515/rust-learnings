fn main() {
    println!("\x1b[0;32m ========== strings and raw strings ========== \x1b[0m\n");

    println!("this is a newline char: \\n");
    println!("this is a tabline char: \t some text");

    println!("Use \\ for file paths as well");

    let current_path: &str = "~/rust-learnings/crate_02_data_types/src/bin/";
    println!("str literal:  {}", current_path);

    // raw strings
    let current_path: &str = r"~/rust-learnings/crate_02_data_types/src/bin/";
    println!("raw string:   {}", current_path);
}

