use std::io;

const BILLS: [u32; 6] = [2, 5, 10, 20, 50, 100];

fn main() {
    loop {
        let mut values = String::new();
        io::stdin()
            .read_line(&mut values)
            .expect("Failed to read the line!");
        let values: Vec<u32> = values
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if values[0] == 0 && values[1] == 0 {
            break;
        }

        let change = values[1] - values[0];
        let mut indexes: (usize, usize) = (0, 5);
        let mut possible: bool = false;
        for _i in 1..=5 {
            let sum = BILLS[indexes.0] + BILLS[indexes.1];
            if change > sum {
                indexes.0 += 1;
            } else if change < sum {
                indexes.1 -= 1;
            } else {
                possible = true;
                break;
            }
        }

        match possible {
            true => println!("possible"),
            false => println!("impossible"),
        }
    }
}
