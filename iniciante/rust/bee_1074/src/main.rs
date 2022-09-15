use std::io;

fn main() {

    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    let amount: u32 = amount.trim().parse().unwrap();
    let mut vec: Vec<i32> = Vec::new();
    let mut counter: u32 = 0;

    while counter < amount {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        vec.push(input);
        counter += 1;
    };

    for number in vec {
        match number {
            0 => println!("NULL"),
            _ => {
                let one = if number % 2 == 0 { "EVEN" } else { "ODD" };
                let two = if number < 0 { "NEGATIVE" } else { "POSITIVE" };
                println!("{} {}", one, two);
            }
        }
    }
}
