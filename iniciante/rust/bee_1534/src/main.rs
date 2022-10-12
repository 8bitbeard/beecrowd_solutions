use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let size = match input.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        for i in 0..size {
            for j in 0..size {
                if j == size - i - 1 {
                    print!("2");
                } else if i == j {
                    print!("1");
                } else {
                    print!("3");
                }
            }
            println!("");
        }
    }
}
