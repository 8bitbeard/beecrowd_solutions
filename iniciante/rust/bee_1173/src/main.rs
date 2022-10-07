use std::io;

const SIZE: usize = 10;

fn main() {
    let mut v: Vec<u32> = Vec::new();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut buffer: u32 = input.trim().parse().unwrap();

    for i in 0..SIZE {
        v.push(buffer);
        println!("N[{}] = {}", i, buffer);
        buffer *= 2;
    }
}
