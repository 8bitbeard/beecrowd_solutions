use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read the line");
    let a: i32 = a.trim().parse::<i32>().unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read the line");
    let b: u8 = b.trim().parse::<u8>().unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read the line");
    let c: u32 = c.trim().parse::<u32>().unwrap();

    println!("A = {}, B = {}, C = {}", a, b, c);
    println!("A = {:>10}, B = {:>10}, C = {:>10}", a, b, c);
    println!("A = {:>010}, B = {:>010}, C = {:>010}", a, b, c);
    println!("A = {:<10}, B = {:<10}, C = {:<10}", a, b, c);
}
