use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i64 = input.trim().parse::<i64>().unwrap();

    let output = 1
        + (((input - 1) * input) / 2)
        + (((input) * (input - 1) * (input - 2) * (input - 3)) / 24);

    println!("{}", output);
}
