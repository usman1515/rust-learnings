fn main() {
    println!("========== methods ==========\n");

    let num1: i32 = -17;
    println!("num1: {}", num1.abs());

    let sample_str: &str = "\t\tlorem ipsum\t\t";
    println!("sample_str original: {}", sample_str);
    println!("sample_str trimmed:  {}", sample_str.trim());

    let num2: i32 = 5;
    println!("num2: {} of power 3 is {}", num2, num2.pow(3));
}
