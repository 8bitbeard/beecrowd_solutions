use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (a, b, c): (i32, i32, i32) = (input[0], input[1], input[2]);

    let happy = if a > b && b <= c {
        true
    } else if a < b && b >= c {
        false
    } else if a < b && (c - b).abs() < (b - a).abs() {
        false
    } else if a < b && (c - b).abs() >= (b - a).abs() {
        true
    } else if a > b && (c - b).abs() < (b - a).abs() {
        true
    } else if a > b && (c - b).abs() >= (b - a).abs() {
        false
    } else if a == b && b < c {
        true
    } else {
        false
    };

    println!(
        "{}",
        match happy {
            true => ":)",
            false => ":(",
        }
    )
}
