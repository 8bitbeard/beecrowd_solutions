use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let input: u32 = input.trim().parse().unwrap();
    
    for i in (4..=(input*4)).step_by(4) {
        println!("{} {} {} PUM", i - 3, i - 2, i - 1);
    }
}
