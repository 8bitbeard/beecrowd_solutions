use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: u32 = amount.trim().parse::<u32>().unwrap();

    let mut approved = true;
    let mut votes: u32 = 0;
    for i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: u32 = input.trim().parse::<u32>().unwrap();

        match i {
            0 => votes = input,
            _ if input > votes => approved = false,
            _ => (),
        }
    }
    println!("{}", if approved { "S" } else { "N" });
}
