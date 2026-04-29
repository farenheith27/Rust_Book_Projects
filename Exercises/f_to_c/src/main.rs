// To convert °C to °F: C = (F - 32) * 5 / 9
// To convert °F to °C: F = C * 9 / 5 + 32
use std::io;

fn f_to_c -> f64 (temperature: f64) {
    (temperature - 32) * 5 / 9
}

fn c_to_f -> f64 (temperature: f64) {
    temperature * 9 / 5 + 32
}

fn main () {
    let temperature: f64;
    let converted_temperature: f64;
    let unit: char;

    loop {
        println!("Insert temperature: ");
        io::stdin().read_line(&temperature).exprect("Failed to read line");
        let temperature: f64 = temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }
    }    

    loop {
        println!("Write 'F' for Fahrenheith or 'C' for Celsius: ")
        io::stdin().read_line(&unit).expect("Failed to read line");
        let unit: char = unit.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        }
        if unit == 'F' {
            converted_temperature = f_to_c(temperature);
        } else if unit == 'C' {
            converted_temperature = c_to_f(temperature);
        } else {
            continue;
        }
    }
}