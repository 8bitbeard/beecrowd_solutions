use std::io;

fn main() {
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read_line");
        let number: usize = match number.trim().parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        let mut values = String::new();
        io::stdin()
            .read_line(&mut values)
            .expect("Failed to read the line");
        let values: Vec<i64> = values
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let output = match number {
            1 => values[0],
            _ => compute_diff(&values),
        };

        println!("{}", output);
    }
}

fn compute_diff(v: &Vec<i64>) -> i64 {
    let mut a = v[0];
    let mut b = v[1..].iter().sum::<i64>();
    let mut diff = (a - b).abs();
    for i in 1..v.len() {
        a += v[i];
        b -= v[i];
        let d = (a - b).abs();
        if d > diff {
            break
        }
        diff = d;
    }
    diff
}
