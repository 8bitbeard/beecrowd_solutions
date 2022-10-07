use std::io;

fn main() {
    let mut test_cases = String::new();

    io::stdin()
        .read_line(&mut test_cases)
        .expect("Failed to read line");

    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _i in 1..=test_cases {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        let mut divisors: Vec<i32> = Vec::new();

        for i in 1..=input / 2 {
            if input % i == 0 {
                divisors.push(i);
            }
        }

        if divisors.len() == 1 {
            println!("{} eh primo", input)
        } else {
            println!("{} nao eh primo", input);
        }
    }
}
