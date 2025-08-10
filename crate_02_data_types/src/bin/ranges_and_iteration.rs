fn main() {
    println!("\x1b[0;32m ========== ranges and range iteration ========== \x1b[0m\n");

    // let month: Range<u32> = 1..12;
    let month = 1..12;      // doesnt include 12
    println!("Months: {:?}", month);

    // loops
    println!("\nMonths Exclusive (Default)");
    for i in month {
        // todo!();
        println!("Current month: {}", i);
    }

    let month = 1u32..=12u32;     // includes 12 and is typecasted to u32
    println!("\nMonths: {:?}", month);

    // loops
    println!("\nMonths Inclusive (Default)");
    for i in month {
        // todo!();
        println!("Current month: {}", i);
    }

    let alphabets = 'a'..='z';
    println!("\nAll the alphabets");
    for i in alphabets {
        println!("Alphabet: {}", i);
    }

    // generics
    let _month_list_1: std::ops::Range<u32> = 1..12;            // doesnt include 12
    let _month_list_2: std::ops::RangeInclusive<u32> = 1..=12;  // includes 12
}
