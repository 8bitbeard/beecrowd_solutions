use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the line");
    let amount: usize = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim().parse::<u32>().unwrap();

        println!("{}", match input {
            (0..=8000) => "Inseto!",
            _ => "Mais de 8000!"
        });
    }
}
