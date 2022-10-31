use std::io;

fn main() {
    let mut times = String::new();
    io::stdin()
        .read_line(&mut times)
        .expect("Failed to read the line!");
    let times: Vec<i32> = times
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("{}", (24 + times.iter().sum::<i32>()) % 24);
}
