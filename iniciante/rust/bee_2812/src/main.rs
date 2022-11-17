use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse().unwrap();

    for _i in 0..tests {
        let mut size = String::new();
        io::stdin()
            .read_line(&mut size)
            .expect("Failed to read the line");
        let size: usize = size.trim().parse().unwrap();

        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read the line");
        let numbers: Vec<u32> = numbers
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut odd: Vec<u32> = numbers.into_iter().filter(|x| x % 2 != 0).collect();
        odd.sort();
        odd.reverse();

        let mut ordered: Vec<u32> = Vec::new();
        for j in 0..odd.len() {
            ordered.push(odd[j]);
            ordered.push(odd[odd.len() - j - 1]);
        }

        let output = ordered[..odd.len()]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", output);
    }
}
