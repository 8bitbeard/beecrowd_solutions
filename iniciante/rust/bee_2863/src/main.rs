use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the line");
        let amount: usize = match amount.trim().parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        let mut times: Vec<u32> = Vec::new();
        for _i in 0..amount {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let input: f64 = input.trim().parse().unwrap();

            let value: u32 = (input * 100.0) as u32;

            times.push(value);
        }

        times.sort();

        println!("{}", times[0] as f64 / 100.0);
    }
}
