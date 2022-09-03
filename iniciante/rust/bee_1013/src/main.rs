use std::io;

fn main() {

    let mut input_data = String::new();

    io::stdin().read_line(&mut input_data).expect("Failed to read line!");
    let input_data = input_data.trim();
    let data_vector: Vec<&str> = input_data.split(' ').collect();

    let a: i32 = data_vector[0].parse().unwrap();
    let b: i32 = data_vector[1].parse().unwrap();
    let c: i32 = data_vector[2].parse().unwrap();

    let greater_ab: i32 = (a + b + (a - b).abs()) / 2;
    let greater_abc: i32 = (greater_ab + c + (greater_ab - c).abs()) / 2;

    println!("{} eh o maior", greater_abc);
}
