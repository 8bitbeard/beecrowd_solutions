use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    let input = input.trim();

    let v: Vec<&str> = input.split(&['.', '-'][..]).collect();

    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
    println!("{}", v[3]);
}
