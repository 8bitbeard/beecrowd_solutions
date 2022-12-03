use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input = input.trim();

    println!(
        "{} es de Mala Suerte",
        match input.contains("13") {
            true => format!("{}", input),
            false => format!("{} NO", input),
        }
    )
}
