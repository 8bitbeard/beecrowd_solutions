use std::io;

const MINUTES_PER_KM: u32 = 2;

fn main() {

    let mut distance = String::new();

    io::stdin().read_line(&mut distance).expect("Failed to read line!");
    let distance: u32 = distance.trim().parse().unwrap();

    let time: u32 = distance * MINUTES_PER_KM;

    println!("{} minutos", time);
}
