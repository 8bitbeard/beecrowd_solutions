use std::io;

fn main() {

    let mut number = String::new();
        
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    
    let number: u32 = number.trim().parse().unwrap();
    
    for x in (2..=number).step_by(2) {
        println!("{}^2 = {}", x, x*x);
    }
}
