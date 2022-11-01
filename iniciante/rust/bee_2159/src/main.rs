use std::io;

const FACTOR: f64 = 1.25506;

fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read tests");
        let n = match n.trim().parse::<f64>() {
            Ok(x) => x,
            _ => break,
        };

        let first = n / n.ln();

        println!("{:.1} {:.1}", first, first * FACTOR);
    }
}
