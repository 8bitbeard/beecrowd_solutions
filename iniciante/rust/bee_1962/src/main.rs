use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin().read_line(&mut tests).expect("Failed to read the line");
    let tests: usize = match tests.trim().parse::<usize>() {
        Ok(x) => x,
        Err(_) => 0
    };

    for _i in 1..=tests {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Filed to read the line");
        let input: i32 = input.trim().parse::<i32>().unwrap();
        let output: i32 = 2015 - input;

        if output < 1 {
            println!("{} A.C.", output.abs() + 1)
        } else {
            println!("{} D.C.", output)
        }
    }
}
