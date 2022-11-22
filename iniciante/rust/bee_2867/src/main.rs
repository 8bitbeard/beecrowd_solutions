use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the line");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input: Vec<f64> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let value = 1 + (input[1] * input[0].log10()) as u32;

        println!("{}", value);
    }
}
