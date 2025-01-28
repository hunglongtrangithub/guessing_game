use crate::utils;

pub fn temp_converter() {
    utils::clear_screen();
    println!("Temperature Converter");
    loop {
        println!("Enter the temperature in degrees, or press Enter to go back:");
        let degrees = utils::read_input();
        if degrees.trim().is_empty() {
            break;
        }
        let degrees = match degrees.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Enter the unit to convert to (C/F):");
        let unit = utils::read_input();

        match unit.trim().to_lowercase().as_str() {
            "f" => {
                let converted = 1.8 * degrees + 32.0;
                println!("{degrees} degrees C = {converted:.2} degrees F");
            }
            "c" => {
                let converted = (degrees - 32.0) / 1.8;
                println!("{degrees} degrees F = {converted:.2} degrees C");
            }
            _ => {
                println!("Invalid unit. Try again!")
            }
        }
    }
}
