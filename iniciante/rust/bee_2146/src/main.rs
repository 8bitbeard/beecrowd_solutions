use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = match input.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        let output: u32 = input - 1;

        println!("{}", output);
    }
}
