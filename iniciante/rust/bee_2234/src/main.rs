use std::io;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read the values");
    let values: Vec<f64> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    println!("{:.2}", values[0] / values[1]);
}
