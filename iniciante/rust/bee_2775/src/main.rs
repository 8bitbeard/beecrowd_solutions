use std::collections::HashMap;
use std::io;

fn main() {
    'outer: loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read amount");
        let amount: usize = match amount.trim() {
            "" => break 'outer,
            x => x.parse().unwrap()
        };

        let mut packages = String::new();
        io::stdin()
            .read_line(&mut packages)
            .expect("Failed to read packages");
        let mut packages: Vec<usize> = packages
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut times = String::new();
        io::stdin()
            .read_line(&mut times)
            .expect("Failed to read packages");
        let times: Vec<usize> = times
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut hm: HashMap<usize, usize> = HashMap::new();
        for it in packages.iter().zip(times.iter()) {
            let (ai, bi) = it;
            hm.insert(*ai, *bi);
        }

        let mut total: usize = 0;
        let mut done: bool = true;
        loop {
            for i in 1..amount {
                let prev = packages[i - 1];
                let curr = packages[i];
                if prev > curr {
                    let buff = curr;
                    packages[i] = prev;
                    packages[i - 1] = buff;
                    total += hm.get(&curr).unwrap();
                    total += hm.get(&prev).unwrap();
                    done = false;
                }
            }
            if done {
                break;
            }
            done = true;
        }

        println!("{}", total);
    }
}
