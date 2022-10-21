use std::io;

fn main() {
    let mut sticks = String::new();
    io::stdin().read_line(&mut sticks).expect("Failed to read line");
    let mut sticks: Vec<u32> = sticks.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

    sticks.sort();

    if sticks[3] < sticks[2] + sticks[1] || sticks[2] < sticks[0] + sticks[1] {
        println!("S");
        return;
    }
    println!("N");
}
