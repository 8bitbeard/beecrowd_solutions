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

    let output = input[1] / (input[0] + 2.0);

    println!("{:.2}", output);
}
