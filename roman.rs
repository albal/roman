use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: roman_numerals <number>");
        return;
    }

    let input = &args[1];
    let number = match input.parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number.", input);
            return;
        }
    };

    // if number > 3999 {
    //     println!("Error: Number must be less than or equal to 3999.");
    //     return;
    // }

    let roman_numerals = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut result = String::new();
    let mut num = number;

    for (rn, value) in roman_numerals {
        while num >= value {
            result.push_str(rn);
            num -= value;
        }
    }

    println!("{} in Roman numerals is: {}", number, result);
}
