fn main() {
    convert_fahrenheit_to_celcius();

    convert_celcius_to_fahrenheit();
}

fn convert_fahrenheit_to_celcius() {
    let fahrenheit_temp = 32.0;

    let fahrenheit_temp_converted_to_celcius = convert_to_celcius(fahrenheit_temp);

    println!("{fahrenheit_temp} F = {fahrenheit_temp_converted_to_celcius} C");
}

fn convert_to_celcius(fahrenheit_temp: f64) -> f64 {
    println!("Converting Fahrenheit to Celcius");

    (fahrenheit_temp - 32.0) / 1.8
}

fn convert_celcius_to_fahrenheit() {
    let celcius_temp = 60.2;

    let celcius_temp_converted_fahrenheit = convert_to_fahrenheit(celcius_temp);

    println!("{celcius_temp} C = {celcius_temp_converted_fahrenheit} F");
}

fn convert_to_fahrenheit(celcius_temp: f64) -> f64 {
    println!("Converting Celcius to Fahrenheit");

    (celcius_temp * 1.8) + 32.0
}
