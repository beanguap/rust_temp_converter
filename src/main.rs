// 1. Use stdin
// 2. I am checking to see if its F-C or C-F
// 3. Convert using forumla
// 4. Print results
// 5. c = (f - 32.0) * 5.0 / 9.0 f = (c * 9.0 / 5.0) + 32.0;

use std::io;

fn main() {
    println!("Converting C -> F / F -> C temperatures using formula");

    // Loop to continuously prompt for input
    loop {
        println!("Please enter temperature to be converted:");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp) //.read_line(&mut temp) store user input in temp
            .expect("Failed to read temp");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered:{}", temp);

        println!("f_to_c(temp): {} Celsius", f_to_c(temp));
        println!("c_to_f(temp): {} Fahrenheit", c_to_f(temp));
        break;
    }
}

// Functions

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
