use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let _input: Vec<u32> = match input.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<u32>().unwrap()).collect(),
        };

        let mut reads = String::new();
        io::stdin()
            .read_line(&mut reads)
            .expect("Failed to read the line");
        let reads: Vec<f64> = reads
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let median = reads.iter().sum::<f64>() / reads.len() as f64;

        let sum_square: Vec<f64> = reads.iter().map(|x| (x - median).powf(2.0)).collect();

        let output = (sum_square.iter().sum::<f64>() / (reads.len() - 1) as f64).sqrt();

        println!("{:.5}", output);
    }
}
