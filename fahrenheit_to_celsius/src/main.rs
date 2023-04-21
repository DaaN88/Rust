use std::io;

fn main() {
    println!("please enter the temperature in Fahrenheit.");

    let mut fahrenheit_temp_as_str: String = String::new();

    io::stdin()
        .read_line(&mut fahrenheit_temp_as_str)
        .expect("Failed to read line");

    let fahrenheit_temp: f64 = match fahrenheit_temp_as_str.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Something wrong"),
    };

    let celsius_temp: f64 = (fahrenheit_temp - 32.0) / 1.8;

    println!("{}", celsius_temp);
}
