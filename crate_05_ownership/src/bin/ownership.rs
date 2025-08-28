fn main() {
    println!("\x1b[0;32m========== scopes and ownership ========== \x1b[0m\n");
    let _age: i32 = 25;  // age is owner of value 25

    println!("\x1b[0;32m========== moves and ownership ========== \x1b[0m\n");
    let name1: String = String::from("John");
    println!("name1: {}", name1);
    let name2: String = name1;
    println!("name2: {}", name2);
    // println!("name1: {}", name1);    // will give error here if uncommented

    println!("\x1b[0;32m========== drop() function ========== \x1b[0m\n");
    let name3: String = String::from("Jane");
    println!("name3: {}", name3);
    drop(name3);
    // println!("name3: {}", name3);    // will give error here if uncommented

    println!("\x1b[0;32m========== clone() function ========== \x1b[0m\n");
    let name4: String = String::from("Joe");
    let name5: String = name4.clone();
    println!("name4: {}", name4);
    drop(name4);
    println!("name5: {}", name5);
}

