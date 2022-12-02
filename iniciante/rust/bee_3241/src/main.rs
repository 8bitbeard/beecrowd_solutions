use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        println!(
            "{}",
            match input.contains('+') {
                true => {
                    let v = input
                        .split('+')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    v.iter().sum::<i32>().to_string()
                }
                false => "skipped".to_string(),
            }
        )
    }
}
