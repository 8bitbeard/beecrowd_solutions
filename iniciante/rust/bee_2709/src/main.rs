use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        let amount = match amount.trim() {
            "" => break,
            x => x.parse::<u32>().unwrap(),
        };

        let mut jar: Vec<u64> = Vec::new();
        for _i in 0..amount {
            let mut coin = String::new();
            io::stdin()
                .read_line(&mut coin)
                .expect("Failed to read line");
            let coin = coin.trim().parse::<u64>().unwrap();

            jar.push(coin);
        }

        let mut step = String::new();
        io::stdin()
            .read_line(&mut step)
            .expect("Failed to read line");
        let step: usize = step.trim().parse::<usize>().unwrap();

        let mut total: u64 = 0;
        for i in (0..amount).rev().step_by(step) {
            total += jar[i as usize];
        }

        match is_prime(&total) {
            true => println!("You’re a coastal aircraft, Robbie, a large silver aircraft."),
            false => println!("Bad boy! I’ll hit you."),
        }
    }
}

fn is_prime(number: &u64) -> bool {
    let iter = *number / 2;
    let mut divisors: u32 = 1;
    for i in 1..=iter {
        if *number % i == 0 {
            divisors += 1;
        }
    }

    divisors == 2
}
