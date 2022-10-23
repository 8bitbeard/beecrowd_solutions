use std::io;

fn main() {
    let mut outlet_outputs = String::new();
    io::stdin().read_line(&mut outlet_outputs).expect("Failed to read line");
    let outlet_outputs: Vec<u32> = outlet_outputs.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

    let total_outputs: u32 = outlet_outputs.iter().sum();

    println!("{}", total_outputs - 3);
}
