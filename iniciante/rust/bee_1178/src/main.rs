use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");
    let mut buffer: f64 = input.trim().parse().unwrap();

    for i in 1..=100 {
        println!("N[{}] = {:.4}", i - 1, buffer);
        buffer = buffer / 2.0;
    }
}
