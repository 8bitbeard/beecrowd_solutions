use std::io;

fn main() {
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read the line!");
    let _size: u32 = size.trim().parse().unwrap();

    let mut digits = String::new();
    io::stdin()
        .read_line(&mut digits)
        .expect("Failed to read the line!");
    let digits: Vec<u32> = digits
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut result: Vec<u32> = vec![0, 0, 0, 0];

    for digit in digits {
        if digit % 2 == 0 {
            result[0] += 1
        }
        if digit % 3 == 0 {
            result[1] += 1
        }
        if digit % 4 == 0 {
            result[2] += 1
        }
        if digit % 5 == 0 {
            result[3] += 1
        }
    }

    println!("{} Multiplo(s) de 2", result[0]);
    println!("{} Multiplo(s) de 3", result[1]);
    println!("{} Multiplo(s) de 4", result[2]);
    println!("{} Multiplo(s) de 5", result[3]);
}
