use std::io;

fn main() {
    let mut tea_number = String::new();
    io::stdin().read_line(&mut tea_number).expect("Failed to read the tea number!");
    let tea_number = tea_number.trim();

    let mut guesses = String::new();
    io::stdin().read_line(&mut guesses).expect("Failed to read the guesses!");
    let guesses: Vec<&str> = guesses.trim().split(' ').collect();


    println!("{}", guesses.iter().filter(|&x| *x == tea_number).count());
}
