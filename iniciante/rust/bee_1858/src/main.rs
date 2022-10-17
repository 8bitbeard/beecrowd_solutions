use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line!");
    let _n = match n.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    let mut list = String::new();
    io::stdin().read_line(&mut list).expect("Failed to read line!");
    let list: Vec<u32> = list.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

    let min_value = list.iter().min().unwrap();
    let index = list.iter().position(|&r| r == *min_value).unwrap();

    println!("{}", index + 1);
}
