use std::io;

fn main() {

    let mut amount = String::new();
        
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    
    let amount: u32 = amount.trim().parse().unwrap();
    let mut counter: u32 = 0;
    
    let mut output: (u32, u32) = (0, 0);
    
    while counter < amount {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input: i32 = input.trim().parse().unwrap();
        
        match input {
            (10..=20) => output.0 += 1,
            _ => output.1 += 1,
        }
        
        counter += 1;
    };
    
    println!("{} in", output.0);
    println!("{} out", output.1);
    
}
