use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().chars().collect::<Vec<char>>()[0];

    println!("{}", input as u8 - 64);
}
