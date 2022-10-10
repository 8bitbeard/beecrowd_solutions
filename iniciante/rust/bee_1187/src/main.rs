use std::io;

const MATRIX_SIZE: usize = 12;

fn main() {
    let mut operation = String::new();

    let mut matrix: [[f64; MATRIX_SIZE]; MATRIX_SIZE] = [[0.0; MATRIX_SIZE]; MATRIX_SIZE];

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read the line!");
    let operation = operation.trim();

    let mut total: f64 = 0.0;
    let mut counter: f64 = 0.0;

    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line!");

            let input: f64 = input.trim().parse().unwrap();

            matrix[i][j] = input;

            if j > i && i < MATRIX_SIZE - (j + 1) {
                total += input;
                counter += 1.0;
            }
        }
    }

    match operation {
        "S" => println!("{:.1}", total),
        "M" => println!("{:.1}", total / counter),
        _ => println!("Operação inválida!"),
    }
}
