use std::io;

fn main() {
    println!("Please input fahrenheit temperature.");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");
    println!("fahrenheit temperature: {}", fahrenheit);
    let celsius: f32 = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("celsius temperature: {}", celsius);
}
