use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let size = match input.trim().parse::<u32>() {
            Ok(x) if x != 0 => x,
            _ => break,
        };

        let width = (2_u32.pow((size - 1) * 2).to_string().chars().count() as u32) as usize;
        for i in 0..size {
            let joined = (i..(i + size))
                .map(|x| format!("{:>1$}", 2_u32.pow(x), width))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", joined);
        }
        println!("");
    }
}
