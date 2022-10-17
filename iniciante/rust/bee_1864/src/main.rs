use std::io;

fn main() {
    let message = "LIFE IS NOT A PROBLEM TO BE SOLVED";

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().unwrap();

    let chars = message.chars();
    let sub : String = chars.into_iter().take(input).collect();

    println!("{}", sub);
}
