use std::io;

fn main() {

    let mut age_in_days = String::new();

    io::stdin().read_line(&mut age_in_days).expect("Failed to read line!");
    let age_in_days: u32 = age_in_days.trim().parse().unwrap();

    let ranges: [u32; 3] = [365, 30, 1];
    let mut values: Vec<u32> = Vec::new();

    let mut remainder: u32 = age_in_days;
    for range in &ranges {
        values.push(remainder / range);
        remainder = remainder % range;
    }

    println!("{} ano(s)", values[0]);
    println!("{} mes(es)", values[1]);
    println!("{} dia(s)", values[2]);
}
