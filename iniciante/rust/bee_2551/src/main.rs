use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the line");
        let amount: u32 = match amount.trim() {
            "" => break,
            x => x.parse::<u32>().unwrap(),
        };

        let mut record: f64 = 0.0;

        for i in 0..amount {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input: Vec<f64> = input
                .trim()
                .split(' ')
                .map(|x| x.parse::<f64>().unwrap())
                .collect();

            let average: f64 = input[1] / input[0];

            if average > record {
                println!("{}", i + 1);
                record = average;
            }
        }
    }
}
