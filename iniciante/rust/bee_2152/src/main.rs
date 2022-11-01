use std::io;

fn main() {
    loop {
        let mut tests = String::new();
        io::stdin()
            .read_line(&mut tests)
            .expect("Failed to read tests");
        let tests = match tests.trim().parse::<u32>() {
            Ok(x) if x > 0 => x,
            _ => break,
        };

        for _i in 0..tests {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let input: Vec<u32> = input
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            println!(
                "{:02}:{:02} - A porta {}!",
                input[0],
                input[1],
                if input[2] == 1 { "abriu" } else { "fechou" }
            );
        }
    }
}
