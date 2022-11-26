use std::io;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Faled to read values");
    let values: Vec<u32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut inputs: Vec<String> = Vec::new();
    for _i in 0..values[0] {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Faled to read values");

        let input = input.trim().to_lowercase();
        inputs.push(input);
    }

    let mut file: Vec<String> = Vec::new();
    for _i in 0..values[1] {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Faled to read values");

        let input = input.trim().to_lowercase();
        file.push(input);
    }

    for e in inputs.iter() {
        let rev = e.chars().rev().collect::<String>();
        match file.iter().any(|x| x.contains(e) || x.contains(&rev)) {
            true => println!("Sheldon come a fruta {}", e),
            false => println!("Sheldon detesta a fruta {}", e),
        };
    }
}
