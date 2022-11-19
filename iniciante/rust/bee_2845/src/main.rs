use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the input");
    let _amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let mut input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    input.sort();

    let mut iter = input.last().unwrap() + 1;
    loop {
        for e in input.iter() {
            if gcd(*e, iter) == 1 {
                println!("{}", iter);
                return;
            }
        }
        iter += 1;
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut max = b;
    let mut min = a;
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
