use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<&str> = input.trim().split(' ').collect();

        match input[0] {
            "Thor" => println!("Y"),
            _ => println!("N")
        }
    }
}
