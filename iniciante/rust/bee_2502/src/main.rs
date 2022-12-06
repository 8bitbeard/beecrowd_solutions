use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read the line");

        let numbers: Vec<u32> = match numbers.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<u32>().unwrap()).collect(),
        };

        let (c, n) = (numbers[0], numbers[1]);

        let mut cipher_a = String::new();
        io::stdin()
            .read_line(&mut cipher_a)
            .expect("Failed to read the line");
        let cipher_a: Vec<char> = cipher_a.trim().chars().collect();

        let mut cipher_b = String::new();
        io::stdin()
            .read_line(&mut cipher_b)
            .expect("Failed to read the line");
        let cipher_b: Vec<char> = cipher_b.trim().chars().collect();

        let mut hm: HashMap<char, char> = HashMap::new();
        for i in 0..c {
            hm.insert(cipher_b[i as usize], cipher_a[i as usize]);
            hm.insert(cipher_a[i as usize], cipher_b[i as usize]);
        }

        for _i in 0..n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input: Vec<char> = input.trim().chars().collect();
            let mut output: Vec<char> = Vec::new();
            for c in input {
                let upper = c.to_ascii_uppercase();
                output.push(match hm.get(&upper) {
                    Some(x) if c.is_uppercase() => *x,
                    Some(x) => x.to_ascii_lowercase(),
                    None => c,
                });
            }
            println!("{}", output.into_iter().collect::<String>());
        }
        println!("");
    }
}
