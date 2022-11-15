use std::io;

use std::collections::HashSet;
use std::iter::FromIterator;

fn vec_to_set(vec: Vec<u32>) -> HashSet<u32> {
    HashSet::from_iter(vec)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the distance");
    let _input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut special = String::new();
    io::stdin()
        .read_line(&mut special)
        .expect("Failed to read the distance");
    let special: Vec<u32> = special
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut owned = String::new();
    io::stdin()
        .read_line(&mut owned)
        .expect("Failed to read the distance");
    let owned: Vec<u32> = owned
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let owned: HashSet<u32> = vec_to_set(owned);

    let output = owned
        .into_iter()
        .filter(|x| special.contains(x))
        .collect::<Vec<u32>>();

    println!("{}", special.len() - output.len());
}
