use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input: usize = input.trim().parse().unwrap();

    let b = (input * input) / 2;
    let w = if input % 2 == 0 { b } else { b + 1 };

    println!("{} casas brancas e {} casas pretas", w, b);
}
