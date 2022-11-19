use std::io;

fn main() {
    let mut words: Vec<String> = Vec::new();
    for _i in 0..2 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim().to_string();

        words.push(input);
    }

    words.sort();

    for w in words.iter() {
        println!("{}", w)
    }
}
