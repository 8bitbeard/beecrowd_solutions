use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input.pop();

    let idx = input.find(",").unwrap();
    let (a, b) = input.split_at(idx);

    println!("{}", a);
    println!("{}", &b[1..]);
}
