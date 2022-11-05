use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: f64 = match input.trim() {
            "" => break,
            x => x.parse::<f64>().unwrap(),
        };

        println!("{}", input.log2() as u32);
    }
}
