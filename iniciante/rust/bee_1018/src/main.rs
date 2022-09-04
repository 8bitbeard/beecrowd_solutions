use std::io;

fn main() {

    let mut input_value = String::new();

    io::stdin().read_line(&mut input_value).expect("Failed to read line!");
    let input_value: u32 = input_value.trim().parse().unwrap();

    let bills: [u32; 7] = [100, 50, 20, 10, 5, 2, 1];
    let mut values: Vec<u32> = Vec::new();

    let mut remainder: u32 = input_value;
    for bill in &bills {
        values.push(remainder / bill);
        remainder = remainder % bill;
    }


    println!("{}", input_value);
    println!("{} nota(s) de R$ 100,00", values[0]);
    println!("{} nota(s) de R$ 50,00", values[1]);
    println!("{} nota(s) de R$ 20,00", values[2]);
    println!("{} nota(s) de R$ 10,00", values[3]);
    println!("{} nota(s) de R$ 5,00", values[4]);
    println!("{} nota(s) de R$ 2,00", values[5]);
    println!("{} nota(s) de R$ 1,00", values[6]);
}
