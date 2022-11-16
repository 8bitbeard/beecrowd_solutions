use std::io;

fn main() {
    let mut l = String::new();
    io::stdin().read_line(&mut l).expect("Failed to read line");
    let l: u32 = l.trim().parse::<u32>().unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: u32 = c.trim().parse::<u32>().unwrap();

    println!("{}", l * c + ((l - 1) * (c - 1)));
    println!("{}", (l - 1) * 2 + (c - 1) * 2);
}
