use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read_line");
    let tests: usize = match tests.trim().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return,
    };

    for _i in 0..tests {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line");
        let input: u64 = input.trim().parse::<u64>().unwrap();

        let binary = format!("{:b}", input);
        let mut ones = binary.split('0').collect::<Vec<&str>>();

        ones.sort();

        println!("{}", ones.last().unwrap().len());
    }
}
