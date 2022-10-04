use std::io;

fn main() {
    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: usize = amount.trim().parse().unwrap();

    let mut v: Vec<Vec<i32>> = Vec::new();

    while v.len() < amount {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: Vec<i32> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        v.push(input);
    }

    for n in v {
        let mut numbers: Vec<i32> = Vec::new();

        let mut iteration = n[0];

        while numbers.len() < n[1] as usize {
            if iteration % 2 != 0 {
                numbers.push(iteration);
            }
            iteration += 1;
        }

        println!("{}", numbers.iter().sum::<i32>());
    }
}
