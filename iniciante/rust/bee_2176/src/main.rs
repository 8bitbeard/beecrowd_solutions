use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read tests");
    let input  = input.trim();

    match input.matches("1").count() % 2 {
        0 => println!("{}0", input),
        _ => println!("{}1", input),
    }
}
