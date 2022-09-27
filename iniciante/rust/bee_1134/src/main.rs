use std::io;

fn main() {

    let mut numbers: [u32; 3] = [0, 0, 0];
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
    
        let input: u32 = input.trim().parse().unwrap();
        
        match input {
            1 => numbers[0] += 1,
            2 => numbers[1] += 1,
            3 => numbers[2] += 1,
            4 => break,
            _ => continue,
        };
    }
    
    println!("MUITO OBRIGADO");
    println!("Alcool: {}", numbers[0]);
    println!("Gasolina: {}", numbers[1]);
    println!("Diesel: {}", numbers[2]);
}
