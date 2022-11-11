use std::io;

fn main() {
    let mut values: Vec<String> = Vec::new();
    for _i in 0..3 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();

        values.push(input);
    }

    println!("A = {}, B = {}, C = {}", values[0], values[1], values[2]);
    println!("A = {}, B = {}, C = {}", values[1], values[2], values[0]);
    println!("A = {}, B = {}, C = {}", values[2], values[0], values[1]);
}
