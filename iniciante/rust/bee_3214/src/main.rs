use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut empty_bottles: u32 = input[0] + input[1];
    let mut drank_bottles: u32 = 0;
    let changeable: u32 = input[2];

    while empty_bottles >= changeable {
        let changed_bottles = empty_bottles / changeable;
        empty_bottles = (empty_bottles % changeable) + changed_bottles;
        drank_bottles += changed_bottles;
    }

    println!("{}", drank_bottles);
}
