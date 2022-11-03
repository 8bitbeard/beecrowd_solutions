use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut sides = String::new();
    io::stdin()
        .read_line(&mut sides)
        .expect("Failed to read the name");
    let mut sides: Vec<u64> = sides
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    sides.sort();
    let (a, b, c): (u64, u64, u64) = (sides[0], sides[1], sides[2]);

    if a + b <= c {
        println!("Invalido");
        return;
    }

    let hs: HashSet<u64> = HashSet::from_iter(sides.iter().cloned());
    match hs.len() {
        1 => println!("Valido-Equilatero"),
        2 => println!("Valido-Isoceles"),
        _ => println!("Valido-Escaleno"),
    };

    match c.pow(2) == b.pow(2) + a.pow(2) {
        true => println!("Retangulo: S"),
        false => println!("Retangulo: N"),
    };
}
