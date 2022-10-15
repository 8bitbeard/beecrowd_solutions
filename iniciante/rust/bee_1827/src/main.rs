use std::io;

fn main() {
    loop {
        let mut size = String::new();
        io::stdin().read_line(&mut size).expect("Failed to read line!");
        let size = match size.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break,
        };

        let border: u32 = size / 3;

        for i in 0..size {
            for j in 0..size {
                if i == size / 2 && j == i {
                    print!("{}", 4);
                } else if (border..size - border).contains(&i) && (border..size - border).contains(&j) {
                    print!("{}", 1);
                } else if i == j {
                    print!("{}", 2);
                } else if j == size - 1 - i {
                    print!("{}", 3);
                } else {
                    print!("{}", 0);
                }
            }
            println!("");
        }
        println!("");
    }
}
