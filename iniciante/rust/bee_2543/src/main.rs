use std::io;

fn main() {
    loop {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");
        let data: Vec<&str> = match data.trim() {
            "" => break,
            x => x.split(' ').collect(),
        };

        let (n, i) = (data[0].parse::<u32>().unwrap(), data[1]);

        let mut total: u32 = 0;
        for _iter in 0..n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: Vec<&str> = input.trim().split(' ').collect();

            match (input[0], input[1]) {
                (x, "0") if x == i => total+= 1,
                _ => continue
            }
        }
        println!("{}", total);
    }

}
