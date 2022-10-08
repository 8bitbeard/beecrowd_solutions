use std::io;

const SIZE: usize = 20;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    for _i in 0..SIZE {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        v.push(input);
    }

    for i in 0..SIZE / 2 {
        let buffer = v[SIZE - 1- i];
        v[SIZE - 1- i] = v[i];
        v[i] = buffer;
    }

    for (pos, e) in v.iter().enumerate() {
        println!("N[{}] = {}", pos, e);
    }
}
