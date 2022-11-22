use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut output: u32 = 0;
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => (),
            _ => output += 1,
        }
    }

    println!("{}", output);
}
