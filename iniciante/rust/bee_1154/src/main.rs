use std::io;

fn main() {
    let mut ages: Vec<f64> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let age: f64 = input.trim().parse().unwrap();

        if age < 0.0 {
            break;
        }

        ages.push(age);
    }

    let median: f64 = ages.iter().sum::<f64>() / ages.len() as f64;

    println!("{:.2}", median);
}
