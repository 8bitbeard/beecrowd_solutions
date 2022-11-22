use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let n = input.trim().parse::<u32>().unwrap();

    println!("{}", (n + 1) * (n + 2) / 2);
}
