use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: usize = amount.trim().parse().unwrap();

    let mut won: u32 = 0;
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim();

        match input.contains("CD") {
            true => (),
            false => won += 1,
        }
    }

    println!("{}", won);
}
