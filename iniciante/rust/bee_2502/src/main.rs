use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read the line");
    let numbers: Vec<u32> = numbers
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (c, n) = (numbers[0], numbers[1]);

    let mut cipher_a = String::new();
    io::stdin()
        .read_line(&mut cipher_a)
        .expect("Failed to read the line");
    let cipher_a: Vec<char> = cipher_a.trim().chars().collect();

    let mut cipher_b = String::new();
    io::stdin()
        .read_line(&mut cipher_b)
        .expect("Failed to read the line");
    let cipher_b: Vec<char> = cipher_b.trim().chars().collect();

    for _i in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: Vec<char> = input.trim().chars().collect();
        for c in input {
            let a = c.to_uppercase() as char;
            println!("{}", cipher_b.contains(&a));
        }
    }
}
