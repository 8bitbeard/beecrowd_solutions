use std::io;

fn main() {
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read number a");
    let a: i32 = a.trim().parse::<i32>().unwrap();

    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read number b");
    let b: i32 = b.trim().parse::<i32>().unwrap();

    println!("{}", a % b);
}
