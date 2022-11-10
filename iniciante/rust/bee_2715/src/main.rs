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
        let output: i32 = match number {
            1 => values.trim().parse::<i32>().unwrap(),
            _ => {
                let v: Vec<i32> = values
                .trim()
                .split(' ')
				.filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
                compute_diff(&v)
            }
        };

        println!("{}", output);
    }
}

fn compute_diff(v: &Vec<i32>) -> i32 {
    let mut sum_a = v[0];
    let mut sum_b = v[1..].iter().sum::<i32>();
    let mut diff = (sum_b - sum_a).abs();

    for i in 1..v.len() {
        sum_b -= v[i];
        sum_a += v[i];
        let d = (sum_b - sum_a).abs();
        if d >= diff {
            break
        }
        diff = d;
    }

    diff

}
