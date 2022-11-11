fn main() {
    println!("Hello, world!");
}use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    let input = input.trim();

    let split: Vec<&str> = input.split(".").collect::<Vec<&str>>();

    println!(
        "{}.{}",
        split[1].parse::<u32>().unwrap(),
        split[0].parse::<u32>().unwrap()
    );
}
