use std::io;

fn main() {
    loop {
        let mut tests = String::new();
        io::stdin()
            .read_line(&mut tests)
            .expect("Failed to read tests");
        let tests = match tests.trim().parse::<u32>() {
            Ok(x) if x > 0 => x,
            _ => break,
        };

        for _i in 0..tests {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let input: u32 = input.trim().parse::<u32>().unwrap();

            let output: u32 = match input % 2 {
                0 => (input - 2) * 2 + 2,
                _ => (input - 1) * 2 + 1,
            };

            println!("{}", output);
        }
    }
}
