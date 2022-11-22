use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: Vec<u32> = amount
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let total_signs: u32 = amount[0] * amount[1];

    let mut values: Vec<u32> = Vec::new();
    for i in 1..10 {
        values.push(((total_signs * i) as f64 / 10.0).ceil() as u32);
    }

    let output = values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}
