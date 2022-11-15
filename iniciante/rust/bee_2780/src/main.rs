use std::io;

fn main() {
    let mut dist = String::new();
    io::stdin()
        .read_line(&mut dist)
        .expect("Failed to read the distance");
    let dist = dist.trim().parse::<u32>().unwrap();

    println!(
        "{}",
        match dist {
            0..=800 => 1,
            801..=1400 => 2,
            _ => 3,
        }
    );
}
