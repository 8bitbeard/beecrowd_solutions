use std::collections::HashMap;
use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: u32 = amount.trim().parse().unwrap();

    let mut menu: HashMap<u32, f64> = HashMap::new();
    menu.insert(1001, 1.5);
    menu.insert(1002, 2.5);
    menu.insert(1003, 3.5);
    menu.insert(1004, 4.5);
    menu.insert(1005, 5.5);

    let mut total: f64 = 0.0;
    for _i in 1..=amount {
        let mut order_data = String::new();
        io::stdin()
            .read_line(&mut order_data)
            .expect("Failed to read line");

        let order_data: Vec<u32> = order_data
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        total += order_data[1] as f64 * menu.get(&order_data[0]).unwrap();
    }

    println!("{:.2}", total);
}
