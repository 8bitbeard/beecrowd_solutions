use std::io;

fn main() {

    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Failed to read line!");
    let value: f64 = value.trim().parse().unwrap();

    match value {
        (0.00..=25.00) => println!("Intervalo [0,25]"),
        (25.01..=50.00) => println!("Intervalo (25,50]"),
        (50.01..=75.00) => println!("Intervalo (50,75]"),
        (75.01..=100.00) => println!("Intervalo (75,100]"),
        _ => println!("Fora de intervalo")
    }
}
