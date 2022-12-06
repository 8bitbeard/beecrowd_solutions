use std::collections::HashMap;
use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: Vec<&str> = input.trim().split(' ').collect();

        let mut game_state = HashMap::new();

        game_state.insert(input[1], input[0]);
        game_state.insert(input[3], input[2]);

        let mut game_total = String::new();
        io::stdin()
            .read_line(&mut game_total)
            .expect("Failed to read line");
        let game_total: Vec<u32> = game_total
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let game_total: u32 = game_total.iter().sum::<u32>();

        match game_total % 2 {
            0 => println!("{}", game_state.get("PAR").unwrap()),
            _ => println!("{}", game_state.get("IMPAR").unwrap()),
        }
    }
}
