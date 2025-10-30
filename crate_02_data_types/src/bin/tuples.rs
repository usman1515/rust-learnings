fn main() {
    println!("========== tuples ==========\n");

    let employee1 = ("Andrea", 60, "Secretary");
    let employee2: (&str, u32, &str) = ("Dirk", 65, "Professor");

    println!("----- Printing by accessing elements -----");
    println!("Employee 1:\nname: {} \nage: {} \njob: {}", employee1.0, employee1.1, employee1.2);
    println!("\nEmployee 2:\nname: {} \nage: {} \njob: {}", employee2.0, employee2.1, employee2.2);

    println!("\n----- Printing entire tuple -----");
    println!("Employee 1: {:?}", employee1);
    println!("Employee 2: {:?}", employee2);

    println!("\n----- Pretty printing entire tuple -----");
    println!("Employee 1: {:#?}", employee1);
    println!("Employee 2: {:#?}", employee2);

    println!("----- Assigning tuple elements to variables -----");
    let (name, age, job) = employee1;
    println!("Employee 1: name: {} age: {} job: {}", name, age, job);

    let (name, age, job) = employee2;
    println!("Employee 2: name: {} age: {} job: {}", name, age, job);

}

