use std::io;

const TIME: f64 = 0.01;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read input");
    let tests = match tests.trim().parse::<u32>() {
        Ok(x) => x,
        Err(_) => return,
    };

    for _i in 0..tests {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        let chars = input
            .chars()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        println!("{:.2}", TIME * chars.len() as f64);
    }
}
