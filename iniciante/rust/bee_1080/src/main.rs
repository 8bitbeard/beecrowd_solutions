use std::io;

fn main() {

    let mut counter: u32 = 0;

    let mut output: (i32, u32) = (0, 0);

    while counter < 100 {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let input: i32 = input.trim().parse().unwrap();

        if input > output.0 {
            output.0 = input;
            output.1 = counter + 1;
        }

        counter += 1;
    }

    println!("{}", output.0);
    println!("{}", output.1);

}
