use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let input: u32 = input.trim().parse().unwrap();
    
    for i in (1..=input) {
        let a = u32::pow(i, 2);
        let b = u32::pow(i, 3);
        println!("{} {} {}", i, a, b);
        println!("{} {} {}", i, a + 1, b + 1);
    }
}
