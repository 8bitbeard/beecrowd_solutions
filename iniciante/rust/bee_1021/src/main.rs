use std::io;

fn main() {

    let mut value = String::new();

    io::stdin().read_line(&mut value).expect("Failed to read line!");
    let value: f64 = value.trim().parse().unwrap();

    let full_value: u32 = (value * 100.0) as u32;

    let valid_values: [u32; 12] = [10000, 5000, 2000, 1000, 500, 200, 100, 50, 25, 10, 5, 1];

    let mut separation: Vec<u32> = Vec::new();

    let mut remainder: u32 = full_value;
    for x in &valid_values {
        separation.push(remainder / x);
        remainder = remainder % x;
    }

    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", separation[0]);
    println!("{} nota(s) de R$ 50.00", separation[1]);
    println!("{} nota(s) de R$ 20.00", separation[2]);
    println!("{} nota(s) de R$ 10.00", separation[3]);
    println!("{} nota(s) de R$ 5.00", separation[4]);
    println!("{} nota(s) de R$ 2.00", separation[5]);
    println!("MOEDAS:");
    println!("{} moeda(s) de R$ 1.00", separation[6]);
    println!("{} moeda(s) de R$ 0.50", separation[7]);
    println!("{} moeda(s) de R$ 0.25", separation[8]);
    println!("{} moeda(s) de R$ 0.10", separation[9]);
    println!("{} moeda(s) de R$ 0.05", separation[10]);
    println!("{} moeda(s) de R$ 0.01", separation[11]);
}
