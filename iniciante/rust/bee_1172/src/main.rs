use std::io;

const SIZE: usize = 10;

fn main() {
    let mut v: Vec<u32> = Vec::new();

    for _i in 1..=SIZE {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        if input > 0 {
            v.push(input as u32);
        } else {
            v.push(1);
        }
    }

    for (pos, number) in v.iter().enumerate() {
        println!("X[{}] = {}", pos, number);
    }
}
