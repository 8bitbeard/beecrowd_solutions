use std::io;

const PI: f64 = 3.14159;

fn main() {

    let mut radius = String::new();

    io::stdin().read_line(&mut radius).expect("Failed to read line!");
    let radius: f64 = radius.trim().parse().unwrap();

    let volume = 4.0 / 3.0 * PI * radius * radius * radius;

    println!("VOLUME = {:.3}", volume);
}
