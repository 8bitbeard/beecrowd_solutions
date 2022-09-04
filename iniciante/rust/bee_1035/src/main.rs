use std::io;

fn main() {

    let mut values = String::new();

    io::stdin().read_line(&mut values).expect("Failed to read line!");
    let values = values.trim();

    let vector: Vec<&str> = values.split(' ').collect();

    let a: i32 = vector[0].parse().unwrap();
    let b: i32 = vector[1].parse().unwrap();
    let c: i32 = vector[2].parse().unwrap();
    let d: i32 = vector[3].parse().unwrap();

    let validation_one: bool = b > c;
    let validation_two: bool = d > a;
    let validation_three: bool = c + d > a + b;
    let validation_four: bool = c > 0 && d> 0;
    let validation_five: bool = a % 2 == 0;

    if validation_one && validation_two && validation_three && validation_four && validation_five {
        println!("Valores aceitos");
        return;
    }

    println!("Valores nao aceitos");
}
