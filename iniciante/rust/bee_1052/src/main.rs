use std::io;
use std::collections::HashMap;


fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input: u8 = input.trim().parse().unwrap();
    
    let mut months: HashMap<u8, &str> = HashMap::new();
    months.insert(1, "January");
    months.insert(2, "February");
    months.insert(3, "March");
    months.insert(4, "April");
    months.insert(5, "May");
    months.insert(6, "June");
    months.insert(7, "July");
    months.insert(8, "August");
    months.insert(9, "September");
    months.insert(10, "October");
    months.insert(11, "November");
    months.insert(12, "December");
    
    let output = match months.get(&input) {
        Some(x) => x,
        None => "",
    };
    
    println!("{}", output);
    
}
