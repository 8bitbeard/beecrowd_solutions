use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();

    println!("{}", input.iter().position(|&x| x == "1").unwrap() + 1);
}
