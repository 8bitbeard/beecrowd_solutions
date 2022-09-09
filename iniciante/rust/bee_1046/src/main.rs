use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let numbers: Vec<i32> = input
                                .trim()
                                .split(' ')
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect();
    
    let total: i32 = if numbers[0] == numbers[1] {
        24
    } else if numbers[0] > numbers[1] {
        24 - numbers[0] + numbers[1]
    } else {
        numbers[1] - numbers[0]
    };
    
    println!("O JOGO DUROU {} HORA(S)", total);

}
