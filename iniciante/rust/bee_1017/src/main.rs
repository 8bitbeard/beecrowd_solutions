use std::io;

const KM_PER_LITER: f64 = 12.0;

fn main() {

    let mut hours = String::new();
    let mut average_speed = String::new();

    io::stdin().read_line(&mut hours).expect("Failed to read line!");
    let hours: u32 = hours.trim().parse().unwrap();

    io::stdin().read_line(&mut average_speed).expect("Failed to read line!");
    let average_speed: u32 = average_speed.trim().parse().unwrap();

    let distance: u32 = hours * average_speed;

    let fuel_consumption: f64 = distance as f64 / KM_PER_LITER;

    println!("{:.3}", fuel_consumption);
}
