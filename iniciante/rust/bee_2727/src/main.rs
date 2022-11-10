use std::io;

fn main() {
    let chars = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    let mut encryp: Vec<String> = Vec::new();

    for i in 0..chars.len() {
        let t: (usize, usize) = ((i / 3) + 1, (i % 3) + 1);
        let s = ".".repeat(t.1);
        let v = vec![s; t.0].join(" ");

        encryp.push(v);
    }

    loop {
        let mut tests = String::new();
        io::stdin()
            .read_line(&mut tests)
            .expect("Failed to read line");
        let tests: u32 = match tests.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        for _i in 0..tests {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_string();

            let idx = encryp.iter().position(|r| r == &input).unwrap();

            println!("{}", chars[idx]);
        }
    }
}
