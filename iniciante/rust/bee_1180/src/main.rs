use std::io;

fn main() {
    let mut vector_size = String::new();
    io::stdin()
        .read_line(&mut vector_size)
        .expect("Failed to read the line!");
    let _vector_size: i32 = vector_size.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");

    let vector: Vec<i32> = input
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    match vector.iter().min() {
        Some(x) => {
            let index = vector.iter().position(|&r| r == *x).unwrap();
            println!("Menor valor: {}", x);
            println!("Posicao: {}", index);
        }
        None => println!("Error"),
    };
}
