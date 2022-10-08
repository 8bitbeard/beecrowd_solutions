use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");
    let input: u32 = input.trim().parse().unwrap();

    for i in 0..1000 {
        println!("N[{}] = {}", i, i % input);
    }
}
