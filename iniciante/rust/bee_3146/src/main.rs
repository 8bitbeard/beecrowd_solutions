use std::io;

const PI: f64 = 3.14;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: f64 = input.trim().parse::<f64>().unwrap();

    let output = 2.0 * PI * input;

    println!("{:.2}", output);
}
