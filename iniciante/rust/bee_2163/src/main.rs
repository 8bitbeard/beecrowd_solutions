use std::io;

fn main() {
    let mut matrix_size = String::new();
    io::stdin()
        .read_line(&mut matrix_size)
        .expect("Failed to read tests");
    let matrix_size: Vec<u32> = matrix_size
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _i in 0..matrix_size[0] {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read tests");
        let line: Vec<i32> = line
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        matrix.push(line);
    }

    for i in 1..matrix_size[0] - 1 {
        for j in 1..matrix_size[1] - 1 {
            if matrix[i as usize][j as usize] == 42 {
                match check_around(i, j, &matrix) {
                    true => {
                        println!("{} {}", i + 1, j + 1);
                        return;
                    }
                    false => continue,
                }
            }
        }
    }

    println!("0 0");
}

fn check_around(a: u32, b: u32, matrix: &Vec<Vec<i32>>) -> bool {
    let points = vec![
        (a - 1, b - 1),
        (a - 1, b),
        (a - 1, b + 1),
        (a, b - 1),
        (a, b + 1),
        (a + 1, b - 1),
        (a + 1, b),
        (a + 1, b + 1),
    ];
    for p in points {
        if matrix[p.0 as usize][p.1 as usize] != 7 {
            return false;
        }
    }
    true
}
