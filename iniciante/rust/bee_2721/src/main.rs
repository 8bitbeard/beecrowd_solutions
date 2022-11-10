use std::io;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read the line");
    let values: Vec<u32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let s = values.iter().sum::<u32>();

    println!("{}", match s % 9 {
        1 => "Dasher",
        2 => "Dancer",
        3 => "Prancer",
        4 => "Vixen",
        5 => "Comet",
        6 => "Cupid",
        7 => "Donner",
        8 => "Blitzen",
        _ => "Rudolph"
    });
}
