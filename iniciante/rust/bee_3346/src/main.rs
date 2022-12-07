use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: Vec<f64> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let x = (100.0 * (1.00 + input[0] / 100.0) * (1.0 + input[1] / 100.0)) - 100.0;

    println!("{:.6}", x);
}
