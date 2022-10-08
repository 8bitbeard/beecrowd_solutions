use std::io;

const SIZE: usize = 100;

fn main() {
    let mut v: Vec<(u32, f64)> = Vec::new();

    for i in 0..SIZE {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = input.trim().parse().unwrap();

        if input <= 10.0 {
            v.push((i as u32, input))
        }
    }

    for t in v {
        println!("A[{}] = {:.1}", t.0, t.1)
    }
}
