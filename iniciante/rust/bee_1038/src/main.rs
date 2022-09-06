use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");

    let v: Vec<&str> = input.trim().split(' ').collect();

    let code: u32 = v[0].parse().unwrap();
    let amount: f64 = v[1].parse().unwrap();
    
    let total: f64 = match code {
        1 => 4.0,
        2 => 4.5,
        3 => 5.0,
        4 => 2.0,
        5 => 1.5,
        _ => 0.0
    } * amount;

    println!("Total: R$ {:.2}", total);

}
