use std::io;


fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input: f64 = input.trim().parse().unwrap();
    
    let taxes: f64 = match input {
        (0.0..=2000.0) => 0.0,
        (2000.01..=3000.00) => (input - 2000.0) * 0.08,
        (3000.01..=4500.00) => (input - 3000.0) * 0.18 + (1000.0 * 0.08),
        _ => (input - 4500.0) * 0.28 + (1500.0 * 0.18) + (1000.0 * 0.08),
    };
    
    if taxes == 0.0 {
        println!("Isento");
    } else {
        println!("R$ {:.2}", taxes)
    }
}
