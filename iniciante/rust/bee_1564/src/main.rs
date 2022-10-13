use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        match input.trim().parse::<u32>() {
            Ok(0) => println!("vai ter copa!"),
            Ok(_) => println!("vai ter duas!"),
            Err(_) => break
        };
    }
}
