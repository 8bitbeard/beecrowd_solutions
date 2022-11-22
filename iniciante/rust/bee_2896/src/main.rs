use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<u32> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if input[0] < input[1] {
            println!("{}", input[0]);
        } else {
            println!("{}", input[0] / input[1] + input[0] % input[1]);
        }
    }
}
