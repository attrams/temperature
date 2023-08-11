use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to the temperature converter program");
    loop {
        print_options();
        
        print!("\nEnter your option: ");
        stdout().flush().unwrap();

        let mut input_option = String::new();

        stdin()
            .read_line(&mut input_option)
            .expect("Please enter a number");

        let input_option: u32 = match input_option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        loop {
            print!("Enter the temperature: ");
            stdout().flush().unwrap();

            let mut input_temperature = String::new();

            stdin()
                .read_line(&mut input_temperature)
                .expect("Please enter temperature");

            let input_temperature: f64 = match input_temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if input_option == 1 {
                let celsius = fahrenheit_to_celsius(input_temperature);

                println!("\n{input_temperature}℉ to Celsius is {celsius}℃.");
            } else {
                let fahrenheit = celsius_to_fahrenheit(input_temperature);

                println!("\n{input_temperature}℃ to Fahrenheit is {fahrenheit}℉.");
            }

            break;
        }
    }
}

fn fahrenheit_to_celsius(temperature_in_fahrenheit: f64) -> f64 {
    let celsius = (temperature_in_fahrenheit - 32.0) * (5.0 / 9.0);
    return celsius;
}

fn celsius_to_fahrenheit(temperature_in_celsius: f64) -> f64 {
    let fahrenheit = (temperature_in_celsius * (9.0 / 5.0)) + 32.0;
    return fahrenheit;
}

fn print_options() {
    println!("\nSelect the converter you want to use");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
}
