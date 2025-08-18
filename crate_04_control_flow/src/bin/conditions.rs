fn main() {
    println!("\x1b[0;32m ========== if statements ========== \x1b[0m\n");
    if true {
        println!("This line will print if TRUE")
    }
    if false {
        println!("This line will print if FALSE")
    }

    println!("\x1b[0;32m ========== else if statements ========== \x1b[0m\n");
    let season: &str = "spring";
    if season == "summer" {
        println!("Season is {}", season);
    }
    else if season == "autumn" {
        println!("Season is {}", season);
    }
    else if season == "winter" {
        println!("Season is {}", season);
    }
    else {
        println!("Season is {}", "spring");
    }

    println!("\x1b[0;32m ========== else statements ========== \x1b[0m\n");
    let mut num: i32 = 57;
    if num % 2 == 0 {
        println!("Num {} is EVEN", num);
    }
    else {
        println!("Num {} is ODD", num);
    }

    println!("\x1b[0;32m ========== conditional expression ========== \x1b[0m\n");
    let res: &str = if num %2 == 0 {"num is EVEN"} else {"num is ODD"};
    println!("Result: {}", res);

    println!("\x1b[0;32m ========== match expression ========== \x1b[0m\n");
    let day: &str = "mon";

    let weather: &str = match day {
        "mon" => "sunny",
        "tue" => "rainy",
        "wed" => "humid",
        "thr" => "cloudy",
        "fri" => "dry",
        "sat" => "hot",
        "sun" => "cold",
        _ => "unknown",
    };
    println!("weather on {} is {}", day, weather);

    println!("\x1b[0;32m ========== match expression with multiple conditions ========== \x1b[0m\n");
    num = 58;

    match num % 10 {
        0 | 2 | 4 | 6 | 8 => println!("Num {} is EVEN", num),
        1 | 3 | 5 | 7 | 9 => println!("Num {} is ODD", num),
        _ => println!("unknown"),
    };

    println!("\x1b[0;32m ========== loop and break keyword ========== \x1b[0m\n");
    let mut seconds: i32 = 10;

    loop {
        if seconds <= 0 {
            println!("LIFTOFF");
            break;
        }
        println!("T minus {} sec to liftoff", seconds);
        seconds -= 1;
    }

    println!("\x1b[0;32m ========== continue keyword ========== \x1b[0m\n");
    seconds = 20;
    loop {
        if seconds <= 0 {
            println!("LIFTOFF");
            break;
        }
        else if seconds % 2 == 0 {
            println!("current count:    {}", seconds);
            seconds -= 3;
            continue;
        }
        println!("sec to liftoff:   {}", seconds);
        seconds -= 1;
    }

    println!("\x1b[0;32m ========== while loop ========== \x1b[0m\n");
    seconds = 15;
    while seconds >= 0 {
        println!("sec to liftoff:   {}", seconds);
        seconds -= 1;
    }

    println!("\x1b[0;32m ========== for loop 0-5 ========== \x1b[0m\n");

    // 0..N exclusive range
    for i in 0..5 {
        println!("current count exclusive:  {}", i);
    }
    println!(" ");

    // 0..N inclusive range
    for i in 0..=5 {
        println!("current count inclusive:  {}", i);
    }
    println!(" ");

    // step increment loop
    for i in (0..5).step_by(2) {
        println!("current count step 2:     {}", i);
    }

    println!("\x1b[0;32m ========== recursion ========== \x1b[0m\n");
    num = 5;
    println!("Factorial of {} is: {}", num, factorial(num));
}

fn factorial(num: i32) -> i32 {
    if num <= 1 {
        return 1;
    }
    else {
        return num * factorial(num-1);
    }
}
