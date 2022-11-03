use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read the values");
    let mut values: Vec<u32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    values.sort();
    let t: HashSet<u32> = HashSet::from_iter(values.iter().cloned());

    if t.len() < 3 || values[0] + values[1] == values[2] {
        println!("S");
    } else {
        println!("N");
    }
}
