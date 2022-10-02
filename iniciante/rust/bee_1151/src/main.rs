use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let n: u32 = input.trim().parse().unwrap();

    let mut fibb: Vec<u32> = Vec::new();

    for i in 0..n {
        if i > 1 {
            fibb.push(fibb[(i - 2) as usize] + fibb[(i - 1) as usize]);
        } else {
            fibb.push(i);
        }
    }

    let vs: Vec<String> = fibb.iter().map(|x| x.to_string()).collect();
    let joined = vs.join(" ");

    println!("{}", joined);
}
