use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut total: f64 = 0.0;
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input: Vec<f64> = input
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        total += input[0] / input[1];
    }

    println!(
        "{}",
        match total <= 1.0 {
            true => "OK",
            false => "FAIL",
        }
    );
}
