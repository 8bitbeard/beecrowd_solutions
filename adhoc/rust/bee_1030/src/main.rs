use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed");
    let amount: usize = amount.trim().parse().unwrap();

    for i in 1..=amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let input: Vec<usize> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let (n, k): (usize, usize) = (input[0], input[1]);

        let mut persons: Vec<usize> = (1..=n).collect();

        let mut idx = 0;
        for _j in 1..n {
            idx = (idx + k - 1) % persons.len();
            persons.remove(idx);
        }

        println!("Case {}: {}", i, persons[0]);
    }
}
