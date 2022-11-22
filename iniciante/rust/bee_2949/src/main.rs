use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut output: [u32; 5] = [0; 5];
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();

        match input[1] {
            "A" => output[3] += 1,
            "E" => output[2] += 1,
            "H" => output[1] += 1,
            "M" => output[4] += 1,
            _ => output[0] += 1,
        }
    }

    println!("{} Hobbit(s)", output[0]);
    println!("{} Humano(s)", output[1]);
    println!("{} Elfo(s)", output[2]);
    println!("{} Anao(oes)", output[3]);
    println!("{} Mago(s)", output[4]);
}
