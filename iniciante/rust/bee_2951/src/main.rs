use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (amount, points): (u32, u32) = (input[0], input[1]);

    let mut runes: HashMap<String, i32> = HashMap::new();
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().split(' ').collect::<Vec<&str>>();

        runes.insert(input[0].to_string(), input[1].parse::<i32>().unwrap());
    }

    let mut runes_amount = String::new();
    io::stdin()
        .read_line(&mut runes_amount)
        .expect("Failed to read the amount");

    let mut said_runes = String::new();
    io::stdin()
        .read_line(&mut said_runes)
        .expect("Failed to read the said runes");
    let said_runes: Vec<String> = said_runes
        .trim()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let output: i32 = said_runes
        .iter()
        .map(|x| runes.get(x).unwrap())
        .sum::<i32>();

    println!("{}", output);
    println!(
        "{}",
        match output < points as i32 {
            true => "My precioooous",
            false => "You shall pass!",
        }
    );
}
