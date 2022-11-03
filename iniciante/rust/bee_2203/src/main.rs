use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let input: Vec<f64> = input
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let distance: f64 =
            ((input[2] - input[0]).powf(2.0) + (input[3] - input[1]).powf(2.0)).sqrt();
        let ran_dist = 1.5 * input[4];

        if distance + ran_dist > input[5] + input[6] {
            println!("N");
        } else {
            println!("Y");
        }
    }
}
