use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input.pop();
    let input = input.to_string();

    let (a, b, c, d) = parse_input(input);
    println!("{}{:.6}{}{}", a, b, c, d);
    println!("{}\t{:.6}\t{}\t{}", a, b, c, d);
    println!("{:>10}{:>10.6}{:>10}{:>10}", a, b, c, d);
}

fn parse_input(input: String) -> (String, f32, String, String) {
    let idx = input.find(" ").unwrap();
    let (a, input) = (&input[..idx], &input[idx + 1..]);

    let idx = input.find(" ").unwrap();
    let (b, input) = (&input[..idx].parse::<f32>().unwrap(), &input[idx + 1..]);

    let idx = input.find(" ").unwrap();
    let (c, d) = (&input[..idx], &input[idx + 1..]);

    (a.to_string(), *b, c.to_string(), d.to_string())
}
