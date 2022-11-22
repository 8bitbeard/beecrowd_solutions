use std::io;

fn main() {
    let mut counter: u32 = 1;
    loop {
        let mut coords = String::new();
        io::stdin()
            .read_line(&mut coords)
            .expect("Failed to read the coords");
        let coords: Vec<u32> = coords
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let (x1, y1, x2, y2): (u32, u32, u32, u32) =
            match (coords[0], coords[1], coords[2], coords[3]) {
                (0, 0, 0, 0) => break,
                _ => (coords[0], coords[1], coords[2], coords[3]),
            };

        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the amount");
        let amount: usize = amount.trim().parse::<usize>().unwrap();

        let mut total: u32 = 0;
        for _i in 0..amount {
            let mut data = String::new();
            io::stdin()
                .read_line(&mut data)
                .expect("Failed to read the data");
            let data: Vec<u32> = data
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let (x, y): (u32, u32) = (data[0], data[1]);

            match ((x1..=x2).contains(&x), (y2..=y1).contains(&y)) {
                (true, true) => total += 1,
                _ => (),
            }
        }

        println!("Teste {}", counter);
        println!("{}", total);

        counter += 1;
    }
}
