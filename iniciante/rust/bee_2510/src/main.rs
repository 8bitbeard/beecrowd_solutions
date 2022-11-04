use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: u32 = tests.trim().parse().unwrap();

    for _i in 0..tests {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input = input.trim();

        match input == "Batmain" {
            true => println!("N"),
            false => println!("Y"),
        }
    }
}
