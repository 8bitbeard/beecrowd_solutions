use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: u32 = amount.trim().parse().unwrap();

    let mut total_tries: Vec<f64> = vec![0.0, 0.0, 0.0];
    let mut success_tries: Vec<f64> = vec![0.0, 0.0, 0.0];

    for _i in 0..amount {
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read the name");

        let mut try_data = String::new();
        io::stdin()
            .read_line(&mut try_data)
            .expect("Failed to read the name");
        let try_data: Vec<u32> = try_data
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for (pos, v) in try_data.iter().enumerate() {
            total_tries[pos] += *v as f64;
        }

        let mut success_data = String::new();
        io::stdin()
            .read_line(&mut success_data)
            .expect("Failed to read the name");
        let success_data: Vec<u32> = success_data
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for (pos, v) in success_data.iter().enumerate() {
            success_tries[pos] += *v as f64;
        }
    }

    println!(
        "Pontos de Saque: {:.2} %.",
        (success_tries[0] / total_tries[0]) * 100.0
    );
    println!(
        "Pontos de Bloqueio: {:.2} %.",
        (success_tries[1] / total_tries[1]) * 100.0
    );
    println!(
        "Pontos de Ataque: {:.2} %.",
        (success_tries[2] / total_tries[2]) * 100.0
    );
}
