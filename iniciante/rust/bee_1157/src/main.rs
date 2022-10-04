use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().unwrap();

    for number in 1..=(input/2) {
        if input % number == 0 {
            println!("{}", number);
        }
    }

    println!("{}", input);
}
