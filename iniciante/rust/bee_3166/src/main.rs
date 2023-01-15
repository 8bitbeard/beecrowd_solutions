use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut words: Vec<String> = Vec::new();
    for _i in 0..input[0] {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_string();
        words.push(input);
    }

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for j in 0..input[1] {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_string();
        matrix.push(input.chars().collect::<Vec<char>>());
    }
}
