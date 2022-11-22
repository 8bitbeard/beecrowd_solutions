use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: u32 = amount.trim().parse::<u32>().unwrap();

    let mut values: (u32, u32) = (0, 0);
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();
        let v: u32 = input[1].parse::<u32>().unwrap();

        match input[0] {
            "G" => values.0 += v,
            _ => values.1 += v,
        }
    }
    println!(
        "{}",
        match values.1 >= values.0 {
            true => "A greve vai parar.",
            false => "NAO VAI TER CORTE, VAI TER LUTA!",
        }
    )
}
