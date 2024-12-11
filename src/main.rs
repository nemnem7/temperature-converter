use std::io;

const FREEZING_POINT_F: f64 = 32.0;
const SCALE_FACTOR_C_TO_F: f64 = 9.0 / 5.0;
const SCALE_FACTOR_F_TO_C: f64 = 5.0 / 9.0;

fn main() {
    loop {
        // Prompt user for type of conversion
        println!(
            "Choose conversions:
    1) Celsius to Fahrenheit
    2) Fahrenheit to Celsius
    0) Exit"
        );

        // Store the type of conversion
        let mut convert_choice = String::new();

        io::stdin()
            .read_line(&mut convert_choice)
            .expect("Failed to read line");
        let convert_choice: u8 = match convert_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match convert_choice {
            // Prompt user for temperature
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => {
                // println!("Temperature in Fahrenheit to convert: ");
                let temperature = input_temperature("Temperature in Fahrenheit to convert: ");
                // Convert temperature to Celsius
                let temperature = fahrenheit_to_celsius(temperature);
                println!("Temperature in Celsius: {:.1}° ", temperature);
                continue;
            }
            2 => {
                // Prompt user for temperature
                // println!("Temperature in Celsius to convert: ");
                let temperature = input_temperature("Temperature in Celsius to convert: ");
                // Convert and print temperature to Fahrenheit
                println!(
                    "Temperature in Fahrenheit: {:.1}°",
                    celsius_to_fahrenheit(temperature)
                );
                continue;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}

fn fahrenheit_to_celsius(temp_fahrenheit: f64) -> f64 {
    (temp_fahrenheit - FREEZING_POINT_F) * SCALE_FACTOR_F_TO_C
}
fn celsius_to_fahrenheit(temp_celsius: f64) -> f64 {
    (temp_celsius * SCALE_FACTOR_C_TO_F) + FREEZING_POINT_F
}

fn input_temperature(temperature: &str) -> f64 {
    println!("{}", temperature);
    // Store the temperature
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Please type a number");
    return temperature;
}
