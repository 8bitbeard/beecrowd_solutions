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

    let (d, f): (u32, u32) = (input[0], input[1]);

    match (d, f) {
        (_, _) if d > f => println!("Eu odeio a professora!"),
        (_, _) if d + 3 <= f => println!("Muito bem! Apresenta antes do Natal!"),
        (_, _) if d + 2 < 24 => {
            println!("Parece o trabalho do meu filho!");
            println!("TCC Apresentado!");
        }
        (_, _) => {
            println!("Parece o trabalho do meu filho!");
            println!("Fail! Entao eh nataaaaal!");
        }
    }
}
