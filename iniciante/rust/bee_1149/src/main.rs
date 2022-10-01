use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let input: Vec<&str> = input.trim().split(' ').collect();
    let input: Vec<i32> = input.iter().map(|&x| x.parse::<i32>().unwrap()).collect();

    let v: Vec<i32> = (input[0]..input[0] + input.last().unwrap()).collect();

    println!("{}", v.iter().sum::<i32>());
}
