use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let n: u32 = input.trim().parse().unwrap();

    let numbers: Vec<u32> = (1..=n).rev().collect();

    let mut total = 1;
    for number in numbers {
        total *= number;
    }

    println!("{}", total);
}
