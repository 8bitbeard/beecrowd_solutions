use std::io;

fn main() {
    let mut l = String::new();
    io::stdin().read_line(&mut l).expect("Failed to read line");
    let l: i32 = l.trim().parse::<i32>().unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: i32 = c.trim().parse::<i32>().unwrap();

    println!(
        "{}",
        match (c - l).abs() % 2 {
            0 => 1,
            _ => 0,
        }
    );
}
