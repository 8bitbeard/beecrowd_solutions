use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = input.trim().parse().unwrap();

    let output = match input {
        1 => "1".to_string(),
        _ => {
            let mut sequence: Vec<u32> = vec![1, 1];
            if input > 2 {
                for i in 2..input {
                    sequence.push(sequence[i as usize - 2] + sequence[i as usize - 1]);
                }
            }
            sequence.reverse();
            sequence
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }
    };

    println!("{}", output);
}
