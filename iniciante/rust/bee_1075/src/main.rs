use std::io;

fn main() {

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = number.trim().parse().unwrap();

    for n in (2..=10000).step_by(number) {
        println!("{}", n);
    }
}
