use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input: f64 = match input.trim().parse::<f64>() {
            Ok(x) => x,
            Err(_) => break,
        };

        let output = (input / 100.0).ceil() as u32;

        println!("{}", output);
    }
}
