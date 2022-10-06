use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        match input {
            0 => break,
            _ => (),
        }

        let mut numbers: Vec<i32> = Vec::new();

        let mut iteration = input;

        while numbers.len() < 5 as usize {
            if iteration % 2 == 0 {
                numbers.push(iteration);
            }
            iteration += 1;
        }

        println!("{}", numbers.iter().sum::<i32>());
    }
}
