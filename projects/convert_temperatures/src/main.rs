use std::io;

fn main() {
    // Uses Variables and Types to Convert Temperatures
    let mut fahrenheit = String::new();
    let fahrenheit_freeze: u8 = 32;
    
    println!("Enter Temperature in Fahrenheit: ");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    println!("Converting {fahrenheit} to celcius...");

    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");

    let celcius = (fahrenheit - fahrenheit_freeze as f32) * 5.0/9.0;
    println!("The temperature of {fahrenheit}°F is {celcius}°C");
}
