use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line!");
    let amount: u32 = amount.trim().parse().unwrap();
    let mut counter: u32 = 0;

    while counter < amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let v: Vec<&str> = input.trim().split(' ').collect();
        let v: Vec<f64> = v.iter().map(|&x| x.parse::<f64>().unwrap()).collect();

        match v[1] as i32 {
            0 => println!("divisao impossivel"),
            _ => println!("{:.1}", v[0] / v[1]),
        }

        counter += 1;
    }
}
