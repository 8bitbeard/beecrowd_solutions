use std::io;

fn main() {
    for i in 0..10 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input = input.trim();

        match i {
            2 | 6 | 8 => println!("{}", input),
            _ => continue,
        }
    }
}
