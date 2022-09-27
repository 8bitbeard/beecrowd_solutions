use std::io;

fn main() {

    let mut numbers: Vec<u32> = Vec::new();
    
    while numbers.len() < 2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
    
        let input: u32 = input.trim().parse().unwrap();
        
        numbers.push(input);
    }
    
    
    numbers.sort();
    
    let mut total: u32 = 0;
    for i in numbers[0]..=numbers[1] {
        if i % 13 != 0 {
            total += i;
        }
    }
        
    println!("{}", total);
        
}
