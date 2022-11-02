use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: f64 = input.trim().parse().unwrap();

    let n = (((1.0 + 5.0_f64.sqrt()) / 2.0).powf(input)
        - ((1.0 - 5.0_f64.sqrt()) / 2.0).powf(input))
        / 5.0_f64.sqrt();

    println!("{:.1}", n);
}
