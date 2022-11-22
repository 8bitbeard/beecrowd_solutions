use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: Vec<f64> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let (dist, d1, d2) = (n[0], n[1], n[2]);

    println!("{:.2}", dist / (d1 + d2));
}
