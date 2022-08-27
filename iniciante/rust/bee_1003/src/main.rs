use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read line!");

    let a: i32 = a.trim().parse().unwrap();

    io::stdin().read_line(&mut b).expect("Failed to read line!");

    let b: i32 = b.trim().parse().unwrap();

    let sum = a + b;

    println!("SOMA = {}", sum);
}
