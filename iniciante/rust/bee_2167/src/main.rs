use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read tests");
    let amount: usize = amount.trim().parse().unwrap();

    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read tests");
    let values: Vec<i32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut output = 0;
    for i in 1..amount {
        if values[i] < values[i - 1] {
            output = i + 1;
            break;
        }
    }

    println!("{}", output);
}
