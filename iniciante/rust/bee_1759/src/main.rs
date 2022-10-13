use std::io;

fn main() {

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line!");
    let number = match number.trim().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return,
    };

    let v: Vec<&str> = vec!["Ho"; number];
    let output = v.join(" ");

    println!("{}!", output);
}
