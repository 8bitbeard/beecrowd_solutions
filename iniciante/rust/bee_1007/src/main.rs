use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read line!");
    let a: i32 = a.trim().parse().unwrap();

    io::stdin().read_line(&mut b).expect("Failed to read line!");
    let b: i32 = b.trim().parse().unwrap();

    io::stdin().read_line(&mut c).expect("Failed to read line!");
    let c: i32 = c.trim().parse().unwrap();

    io::stdin().read_line(&mut d).expect("Failed to read line!");
    let d: i32 = d.trim().parse().unwrap();

    let diff: i32 = a * b - c * d;

    println!("DIFERENCA = {}", diff);
}
