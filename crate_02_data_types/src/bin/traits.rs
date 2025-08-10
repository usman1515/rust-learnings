fn main() {
    println!("\x1b[0;32m ========== traits ========== \x1b[0m\n");

    // display trait
    println!("{}", 5);
    println!("{}", 22.0/7.0);
    println!("{}", "hello world");

    // debug trait
    let seasons: [&str; 4] = ["summer", "autumn", "winter", "spring"];
    println!("Seasons: {:?}", seasons);

    println!("Seasons: {seasons:?}");

    println!("Seasons pretty print: {seasons:#?}");

    // debug macro
    dbg!(2 + 2);

    dbg!(seasons);
}
