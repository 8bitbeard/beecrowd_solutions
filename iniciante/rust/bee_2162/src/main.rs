use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read tests");
    let amount: usize = amount.trim().parse().unwrap();

    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read tests");
    let values: Vec<i32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut dir: bool = match compare(&values[0], &values[1]) {
        Some(x) => x,
        None => {
            println!("0");
            return;
        }
    };

    for i in 1..amount - 1 {
        match compare(&values[i], &values[i + 1]) {
            Some(x) if x == !dir => dir = !dir,
            _ => {
                println!("0");
                return;
            }
        }
    }

    println!("1");
}

fn compare(a: &i32, b: &i32) -> Option<bool> {
    if a == b {
        return None;
    } else if a < b {
        return Some(false);
    } else {
        return Some(true);
    }
}
