use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let input: u32 = input.trim().parse().unwrap();

        match input {
            0 => break,
            _ => {
                let v: Vec<String> = (1..=input).map(|x| x.to_string()).collect();

                let joined = v.join(" ");
                println!("{}", joined);
            }
        }
    }
}
