use std::io;

fn main() {
    let pi: f64 = 3.14159;

    let mut radius = String::new();

    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line!");

    let radius: f64 = radius.trim().parse().unwrap();

    let area = pi * f64::powf(radius, 2.0);

    println!("A={:.4}", area);
}
