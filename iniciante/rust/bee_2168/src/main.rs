use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read tests");
    let amount: usize = amount.trim().parse().unwrap();

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for _i in 0..=amount {
        let mut values = String::new();
        io::stdin()
            .read_line(&mut values)
            .expect("Failed to read tests");
        let values: Vec<u8> = values
            .trim()
            .split(' ')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        matrix.push(values);
    }

    for i in 0..amount {
        for j in 0..amount {
            match check_quadrant(i, j, &matrix) {
                true => print!("S"),
                false => print!("U"),
            }
        }
        println!("");
    }
}

fn check_quadrant(a: usize, b: usize, matrix: &Vec<Vec<u8>>) -> bool {
    let loc: Vec<(usize, usize)> = vec![(a, b), (a, b + 1), (a + 1, b), (a + 1, b + 1)];

    let mut total = 0;
    for p in loc {
        if matrix[p.0][p.1] == 1 {
            total += 1
        }
    }

    total >= 2
}
