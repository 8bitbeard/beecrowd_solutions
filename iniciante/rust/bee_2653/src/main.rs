use std::collections::HashSet;
use std::io;

fn main() {
    let mut jewel_types: HashSet<String> = HashSet::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "" => break,
            x => jewel_types.insert(x.to_string()),
        };
    }

    println!("{}", jewel_types.len());
}
