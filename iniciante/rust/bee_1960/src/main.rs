use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse::<u32>().unwrap();
    
    let u: u32 = input % 10;
    let d: u32 = (input % 100) - u;
    let c: u32 = input - d - u;
    
    let split: Vec<u32> = vec![c, d, u];
    
    let output: Vec<String> = split.iter().map(|x| parse_value(x)).collect();
    
    println!("{}", output.join(""))
}

fn parse_value(number: &u32) -> String {
    match *number {
        (0..=3) => "I".repeat(*number as usize),
        4 => "IV".to_string(),
        (5..=8) => "V".to_string() + &"I".repeat((*number - 5) as usize),
        9 => "IX".to_string(),
        (10..=30) => "X".repeat((*number / 10) as usize),
        40 => "XL".to_string(),
        (50..=80) => "L".to_string() + &"X".repeat((*number / 10 - 5) as usize),
        90 => "XC".to_string(),
        (100..=300) => "C".repeat((*number / 100) as usize),
        400 => "CD".to_string(),
        (500..=800) => "D".to_string() + &"C".repeat((*number / 100 - 5) as usize),
        _ => "CM".to_string()
    }
}
