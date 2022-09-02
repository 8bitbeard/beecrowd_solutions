use std::io;

fn main() {
    let mut product_one = String::new();
    let mut product_two = String::new();

    io::stdin().read_line(&mut product_one).expect("Failed to read line!");
    let product_one = product_one.trim();
    let data_one: Vec<&str> = product_one.split(' ').collect();

    let product_one_amount: f64 = data_one[1].parse().unwrap();
    let product_one_price: f64 = data_one[2].parse().unwrap();

    io::stdin().read_line(&mut product_two).expect("Failed to read line!");
    let product_two = product_two.trim();
    let data_two: Vec<&str> = product_two.split(' ').collect();

    let product_two_amount: f64 = data_two[1].parse().unwrap();
    let product_two_price: f64 = data_two[2].parse().unwrap();

    let total = product_one_amount * product_one_price + product_two_amount * product_two_price;

    println!("VALOR A PAGAR: R$ {:.2}", total);
}
