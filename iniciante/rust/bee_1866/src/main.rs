use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(x) => println!("{}", x % 2),
            Err(_) => println!("O valor digitado é inválido!")
        };
    }
}
