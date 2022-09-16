use std::io;

fn main() {

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().unwrap();

    for n in (1..=10) {
        println!("{} x {} = {}", n, number, n * number);
    }
}
