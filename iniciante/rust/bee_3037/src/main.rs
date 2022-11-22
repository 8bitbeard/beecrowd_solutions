use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    for _i in 0..amount {
        let mut values: (u32, u32) = (0, 0);
        for j in 0..6 {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the input");
            let v: Vec<u32> = input
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            let points = v[0] * v[1];

            match j {
                (0..=2) => values.0 += points,
                _ => values.1 += points,
            }
        }

        println!(
            "{}",
            match values.0 < values.1 {
                true => "MARIA",
                false => "JOAO",
            }
        );
    }
}
