use std::io;

fn main() {
    let mut test_cases = String::new();
    io::stdin()
        .read_line(&mut test_cases)
        .expect("Failed to read the line!");
    let test_cases: u32 = test_cases.trim().parse().unwrap();

    for _i in 0..test_cases {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let n: u32 = input.trim().parse().unwrap();

        match n {
            0 => println!("Fib(0) = 0"),
            1 => println!("Fib(1) = 1"),
            _ => {
                let mut a: [u64; 2] = [0, 1];
                for _i in 1..n {
                    let s = a.iter().sum();
                    a.reverse();
                    a[1] = s;
                }
                println!("Fib({}) = {}", n, a[1]);
            }
        }
    }
}
