use std::io;

fn main() {
    let mut meal_amount = String::new();
    io::stdin()
        .read_line(&mut meal_amount)
        .expect("Failed to read line");
    let meal_amount: Vec<u32> = meal_amount
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut orders = String::new();
    io::stdin()
        .read_line(&mut orders)
        .expect("Failed to read line");
    let orders: Vec<u32> = orders
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let it = meal_amount.iter().zip(orders.iter());

    let amount = it.fold(0u32, |sum, val| sum + compare(val));

    println!("{}", amount);
}

fn compare(t: (&u32, &u32)) -> u32 {
    match t.1 <= t.0 {
        true => 0,
        false => *t.1 - *t.0,
    }
}
