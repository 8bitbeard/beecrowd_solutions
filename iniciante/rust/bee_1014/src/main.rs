use std::io;

fn main() {

    let mut distance = String::new();
    let mut fuel_spent = String::new();

    io::stdin().read_line(&mut distance).expect("Failed to read line!");
    let distance: f64 = distance.trim().parse().unwrap();

    io::stdin().read_line(&mut fuel_spent).expect("Failed to read line!");
    let fuel_spent: f64 = fuel_spent.trim().parse().unwrap();

    let average_consumption: f64 = distance / fuel_spent;

    println!("{:.3} km/l", average_consumption);
}
