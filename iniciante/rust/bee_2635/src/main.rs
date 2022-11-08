use std::io;
use std::cmp::Ordering;

fn main() {
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read the line");
    let words: usize = words.trim().parse::<usize>().unwrap();

    let mut inputs: Vec<String> = Vec::new();
    for _i in 1..=words {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read strategy");
        let input = input.trim().to_string();

        inputs.push(input);
    }

    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse::<usize>().unwrap();

    for _i in 0..tests {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input = input.trim();

        let mut matches: Vec<&String> = inputs
            .iter()
            .filter(|x| x.starts_with(input))
            .collect::<Vec<_>>();

        matches.sort_by(|a, b| {
            let ac = a.chars().count();
            let bc = b.chars().count();
            if ac < bc {
                Ordering::Less
            } else if ac == bc {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        match matches.len() {
            0 => {
                println!("-1");
                continue;
            }
            x => println!("{} {}", x, matches.last().unwrap().chars().count()),
        }
    }
}
