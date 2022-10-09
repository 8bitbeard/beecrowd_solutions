use std::io;

const MATRIX_SIZE: usize = 12;

fn main() {
    let mut matrix_line = String::new();
    let mut operation = String::new();

    let mut matrix: [[f64; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];

    io::stdin()
        .read_line(&mut matrix_line)
        .expect("Failed to read the line!");
    let matrix_line: usize = matrix_line.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read the line!");
    let operation = operation.trim();

    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line!");

            let input: f64 = input.trim().parse().unwrap();

            matrix[i][j] = input;
        }
    }

    let total: f64 = matrix[matrix_line].iter().sum();

    match operation {
        "S" => println!("{:.1}", total),
        "M" => println!("{:.1}", total / MATRIX_SIZE as f64),
        _ => println!("Operação inválida!"),
    }
}
