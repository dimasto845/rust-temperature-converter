use std::io;

const OFFSET: f64 = 32.0;
const WEIGHT_FACTOR: f64 = 5.0/9.0;

fn main() {
    println!("Enter value:");

    let mut value = String::new();

    io::stdin().read_line(&mut value)
        .expect("Failed to read line");

    let value: f64 = value.trim().parse()
        .expect("Cannot parse input.");

    println!("Enter destination tempterature type. (c or f)");

    let mut output_type = String::new();

    io::stdin().read_line(&mut output_type)
        .expect("Failed to read line");

    let output_type = output_type.trim();

    let mut result = 0.0;

    if output_type == "c"{
        result = fahrenheit_to_celcius(value);
    } else if output_type == "f"{
        result = celcius_to_fahrenheit(value);
    } else {
        println!("Invalid destination temperature type.");
    }

    println!("Input {}.", value);
    println!("Result {}°{}.", result, output_type);
}

fn celcius_to_fahrenheit(temp:f64)->f64{
    temp / WEIGHT_FACTOR + OFFSET
}

fn fahrenheit_to_celcius(temp:f64)->f64{
    (temp - OFFSET) * WEIGHT_FACTOR
}
