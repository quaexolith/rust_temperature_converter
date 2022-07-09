fn main() {
    convert_fahrenheit_to_celsius();

    convert_celsius_to_fahrenheit();
}

fn convert_fahrenheit_to_celsius() {
    let fahrenheit_temp = 32.0;

    let fahrenheit_temp_converted_to_celsius = convert_to_celsius(fahrenheit_temp);

    println!("{fahrenheit_temp} F = {fahrenheit_temp_converted_to_celsius} C");
}

fn convert_to_celsius(fahrenheit_temp: f64) -> f64 {
    println!("Converting Fahrenheit to Celsius");

    (fahrenheit_temp - 32.0) / 1.8
}

fn convert_celsius_to_fahrenheit() {
    let celsius_temp = 60.2;

    let celsius_temp_converted_fahrenheit = convert_to_fahrenheit(celsius_temp);

    println!("{celsius_temp} C = {celsius_temp_converted_fahrenheit} F");
}

fn convert_to_fahrenheit(celsius_temp: f64) -> f64 {
    println!("Converting Celsius to Fahrenheit");

    (celsius_temp * 1.8) + 32.0
}
