use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read amount");
        let amount: u32 = match amount.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        let mut current_position: i32 = 1;
        let mut total_moves: i32 = 0;
        for i in 0..amount {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            if i > 0 {
                let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();

                match input.iter().filter(|&n| *n == "1").count() > 0 {
                    true => {
                        let idx_to_go: i32 = input.iter().position(|&c| c == "0").unwrap() as i32;
                        total_moves += (idx_to_go - current_position).abs();

                        current_position = idx_to_go;
                    }
                    false => (),
                }
            }
        }

        println!("{}", total_moves);
    }
}
