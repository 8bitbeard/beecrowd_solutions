use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: Vec<usize> = match input.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<usize>().unwrap()).collect(),
        };

        let mut output = String::from("Pizza antes de FdI");

        let mut found: bool = false;
        for _i in 0..input[1] {
            let mut data = String::new();
            io::stdin()
                .read_line(&mut data)
                .expect("Failed to read the line");
            let data: Vec<&str> = data.trim().split(' ').collect();

            let votes: Vec<u32> = data[1..]
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            match (votes.iter().all(&|i| i == &1), found) {
                (true, false) => {
                    output = String::from(data[0]);
                    found = true;
                },
                _ => (),
            }
        }

        println!("{}", output);
    }
}
