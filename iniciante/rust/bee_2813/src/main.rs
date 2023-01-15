use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse().unwrap();

    let mut inputs: Vec<bool> = Vec::new();
    for _i in 0..tests {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input = input.trim();
        let idx = input.find(' ').unwrap();

        let (ida, volta): (&str, &str) = (&input[..idx], &input[idx + 1..]);

        inputs.push(if ida == "sol" { false } else { true });
        inputs.push(if volta == "sol" { false } else { true });
    }

    let mut output: (u32, u32) = (0, 0);
    let mut bring: bool = false;
    for (pos, e) in inputs.iter().enumerate() {
        match e {
            false => {
                bring = false;
            }
            true => match bring {
                true => {}
                false => {
                    if pos % 2 == 0 {
                        output.0 += 1;
                        bring = true;
                    } else {
                        output.1 += 1;
                        bring = true;
                    }
                }
            },
        }
    }

    println!("{:?}", output);
}
