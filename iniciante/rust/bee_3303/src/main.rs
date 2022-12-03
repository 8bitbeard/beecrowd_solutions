use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input = input.trim();

    println!(
        "{}",
        match input.chars().count() >= 10 {
            true => "palavrao",
            false => "palavrinha",
        }
    )
}
