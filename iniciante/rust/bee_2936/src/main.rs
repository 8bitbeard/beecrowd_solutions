use std::io;

const GRAMS: [u32; 5] = [300, 1500, 600, 1000, 150];

fn main() {
    let mut total: u32 = 225;
    for i in 0..5 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: u32 = input.trim().parse::<u32>().unwrap();

        total += GRAMS[i] * input;
    }

    println!("{}", total);
}
