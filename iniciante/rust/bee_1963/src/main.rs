use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<f64> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let (old, new): (f64, f64) = (input[0], input[1]);

    println!("{:.2}%", ((new / old) - 1.0) * 100.0);
}
