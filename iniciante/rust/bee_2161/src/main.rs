use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read tests");
    let number: usize = number.trim().parse().unwrap();

    let mut factor: f64 = 0.0;
    for _i in 0..number {
        factor = 1.0 / (6.0 + factor)
    }

    println!("{:.10}", 3.0 + factor);
}
