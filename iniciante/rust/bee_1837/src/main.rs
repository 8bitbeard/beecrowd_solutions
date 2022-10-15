use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");

    let input: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut r: i32 = 0;
    loop {
        if (input[0] - r) % input[1] == 0 {
            break;
        }
        r += 1;
    }

    let q = (input[0] - r) / input[1];

    println!("{} {}", q, r);
}
