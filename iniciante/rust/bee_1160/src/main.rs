use std::io;

fn main() {
    let mut test_cases = String::new();

    io::stdin()
        .read_line(&mut test_cases)
        .expect("Failed to read line");

    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _i in 1..=test_cases {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: Vec<f64> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let mut pop_a: u32 = input[0] as u32;
        let mut pop_b: u32 = input[1] as u32;

        let mut years: u32 = 0;

        while pop_a <= pop_b && years < 101 {
            pop_a += (pop_a as f64 * input[2] / 100.0).floor() as u32;
            pop_b += (pop_b as f64 * input[3] / 100.0).floor() as u32;

            years += 1;
        }

        match years {
            (0..=100) => println!("{} anos.", years),
            _ => println!("Mais de 1 seculo."),
        }
    }
}
