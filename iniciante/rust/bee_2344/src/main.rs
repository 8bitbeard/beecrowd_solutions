use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    let input: u32 = input.trim().parse().unwrap();

    let grade = match input {
        0 => "E",
        (1..=35) => "D",
        (36..=60) => "C",
        (61..=85) => "B",
        _ => "A"
    };

    println!("{}", grade);
}
