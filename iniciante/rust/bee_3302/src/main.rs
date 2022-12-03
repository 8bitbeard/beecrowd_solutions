use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the input");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    for i in 1..=amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input: u32 = input.trim().parse::<u32>().unwrap();

        println!("resposta {}: {}", i, input)
    }
}
