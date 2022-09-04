use std::io;

fn main() {

    let mut seconds = String::new();

    io::stdin().read_line(&mut seconds).expect("Failed to read line!");
    let seconds: u32 = seconds.trim().parse().unwrap();

    let ranges: [u32; 3] = [3600, 60, 1];
    let mut values: Vec<u32> = Vec::new();

    let mut remainder: u32 = seconds;
    for range in &ranges {
        values.push(remainder / range);
        remainder = remainder % range;
    }

    println!("{}:{}:{}", values[0], values[1], values[2]);
}
