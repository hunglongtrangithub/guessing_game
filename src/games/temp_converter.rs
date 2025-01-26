use crate::games::utils;
use std::io;

pub fn temp_converter() {
    utils::clear_screen();
    println!("Temperature Converter");
    loop {
        println!("Enter the temperature in degrees:");
        let mut degrees = String::new();
        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line");
        let degrees = match degrees.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("Enter the unit to convert to (C/F):");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        match unit.trim().to_lowercase().as_str() {
            "f" => {
                let converted = 1.8 * degrees + 32.0;
                println!("{degrees} degrees C = {converted:.2} degrees F");
                break;
            }
            "c" => {
                let converted = (degrees - 32.0) / 1.8;
                println!("{degrees} degrees F = {converted:.2} degrees C");
                break;
            }
            _ => {
                println!("Invalid unit. Try again!")
            }
        }
    }
}
