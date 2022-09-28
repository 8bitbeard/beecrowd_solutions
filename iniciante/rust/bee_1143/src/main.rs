use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let input: u32 = input.trim().parse().unwrap();
    
    for i in (1..=input) {
        println!("{} {} {}", i, u32::pow(i, 2), u32::pow(i, 3));
    }
}
