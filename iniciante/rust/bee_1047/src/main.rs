use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let numbers: Vec<u32> = input
                                .trim()
                                .split(' ')
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect();
    
    let start_in_minutes: u32 = numbers[0] * 60 + numbers[1];
    let end_in_minutes: u32 = numbers[2] * 60 + numbers[3];
    
    let total = if start_in_minutes == end_in_minutes {
        24 * 60
    } else if start_in_minutes > end_in_minutes {
        24 * 60 - start_in_minutes + end_in_minutes
    } else {
        end_in_minutes - start_in_minutes
    };
    
    let hours: u32 = total / 60;
    let minutes: u32 = total % 60;
    
    println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", hours, minutes);

}
