use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut output: (u32, u32) = (0, 0);
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();

        match input[1] {
            "M" => output.0 += 1,
            _ => output.1 += 1,
        }
    }

    println!("{} carrinhos", output.0);
    println!("{} bonecas", output.1);
}
