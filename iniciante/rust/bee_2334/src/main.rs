use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = match input.trim() {
            "-1" => break,
            "0" => {
                println!("0");
                continue
            }
            x => x,
        };

        let mut chars: Vec<u32> = input
            .chars()
            .into_iter()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();
        chars.reverse();

        let numbers: Vec<u32> = vec![9, 0, 1, 2, 3, 4, 5, 6, 7, 8];

        let mut change: bool = true;
        for i in 0..chars.len() {
            chars[i] = match (change, chars[i]) {
                (true, 0) => {
                    change = true;
                    numbers[chars[i] as usize]
                }
                (true, _) => {
                    change = false;
                    numbers[chars[i] as usize]
                }
                _ => continue,
            }
        }
        chars.reverse();

        if chars.len() > 1 && chars[0] == 0 {
            chars.remove(0);
        }

        let output = chars
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        println!("{}", output);
    }
}
