use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: Vec<u32> = match input.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<u32>().unwrap()).collect(),
        };

        let (n, min, max): (u32, u32, u32) = (input[0], input[1], input[2]);

        let mut total: u32 = 0;
        for _i in 0..n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input = input.trim().parse::<u32>().unwrap();

            if input >= min && input <= max {
                total += 1
            }
        }

        println!("{}", total);
    }
}
