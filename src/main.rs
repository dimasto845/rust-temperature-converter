const SUBTRACT_NUM: f64 = 32.0;
const WEIGHT_FACTOR: f64 = 5.0/9.0;

fn main() {
    let boiling = celcius_to_fahrenheit(100.0);
    let freezing = fahrenheit_to_celcius(32.0);
    println!("Boiling in fahrenheit is {}.", boiling);
    println!("Freezing in celcius is {}.", freezing);
}

fn celcius_to_fahrenheit(temp:f64)->f64{
    temp / WEIGHT_FACTOR + SUBTRACT_NUM
}

fn fahrenheit_to_celcius(temp:f64)->f64{
    (temp - SUBTRACT_NUM) * WEIGHT_FACTOR
}
