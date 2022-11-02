use std::io;

fn main() {
    loop {
        let mut values = String::new();
        io::stdin()
            .read_line(&mut values)
            .expect("Failed to read tests");
        let values: Vec<u64> = values
            .trim()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        if values[0] == 0 && values[1] == 0 {
            break;
        }

        println!("{}", values[0] * values[1]);
    }
}
