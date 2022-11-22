use std::io;

fn main() {
    let mut counter: u32 = 1;
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the amount");
        let _amount: u32 = match amount.trim().parse::<u32>() {
            Ok(x) if x != 0 => x,
            _ => break,
        };

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim();

        let numbers = input
            .split(&['+', '-'][..])
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let signals = input
            .chars()
            .filter(|x| ['+', '-'].contains(x))
            .collect::<Vec<char>>();

        let mut total: i32 = numbers[0];
        for (pos, sig) in signals.iter().enumerate() {
            match sig {
                '+' => total += numbers[pos + 1],
                _ => total -= numbers[pos + 1],
            }
        }

        println!("Teste {}", counter);
        println!("{}", total);
        println!("");

        counter += 1;
    }
}
