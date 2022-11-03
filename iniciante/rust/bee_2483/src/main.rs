use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    let input: u32 = input.trim().parse().unwrap();


    println!("Feliz nat{}l!", (0..input).map(|_| "a").collect::<String>());
}
