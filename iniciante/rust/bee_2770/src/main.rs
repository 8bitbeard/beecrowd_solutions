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

        let mut available_dim: Vec<u32> = vec![input[0], input[1]];
        available_dim.sort();

        for _i in 0..input[2] {
            let mut dimensions = String::new();
            io::stdin()
                .read_line(&mut dimensions)
                .expect("Failed to read the line");
            let mut dimensions: Vec<u32> = dimensions
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            dimensions.sort();

            if dimensions[0] <= available_dim[0] && dimensions[1] <= available_dim[1] {
                println!("Sim")
            } else {
                println!("Nao")
            }
        }
    }
}
