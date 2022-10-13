use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read line!");
        let _amount = match amount.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        let mut v = String::new();
        io::stdin().read_line(&mut v).expect("Failed to read line!");
        let v: Vec<u32> = v.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

        let max_value = v.iter().max().unwrap();

        let level: u32 = match max_value {
            (0..=9) => 1,
            (10..=19) => 2,
            _ => 3
        };

        println!("{}", level);
    }
}
