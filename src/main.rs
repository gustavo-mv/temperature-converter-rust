use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
fn main() {
    println!("\n ******************* Temperature Converter ******************* \n
   || 1- Fahrenheit to Celsius ||  2- Celsius to Fahrenheit ||");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading the line");

    match choice.trim() {
        "1" => {
            println!("Type the Fahrenheit temperature:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error reading the line");
            let fahrenheit: f64 = input.trim().parse().expect("Error in conversion");
            let celsius: f64 = fahrenheit_to_celsius(fahrenheit);
            println!("{:.2} Fahrenheit is equal to {:.2} Celsius.", fahrenheit, celsius);
        }
        "2" => {
            println!("Type the Celsius temperature:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error reading the line");
            let celsius: f64 = input.trim().parse().expect("Error converting to number");
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{:.2} Celsius is equal to {:.2} Fahrenheit.", celsius, fahrenheit);
        }
        _ => {
            println!("Invalid option. Please, choose between 1 or 2...");
        }
    }
}
