use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line!");
    let amount: u32 = amount.trim().parse().unwrap();

    let mut counter: u32 = 0;

    let mut output: (u32, u32, u32) = (0, 0, 0);

    while counter < amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let v: Vec<&str> = input.trim().split(' ').collect();

        let samples: u32 = v[0].parse().unwrap();

        match v[1] {
            "C" => output.0 += samples,
            "R" => output.1 += samples,
            "S" => output.2 += samples,
            _ => continue,
        }

        counter += 1;
    }

    let total: u32 = output.0 + output.1 + output.2;
    let c_percent: f64 = (output.0 as f64 / total as f64) * 100.0;
    let r_percent: f64 = (output.1 as f64 / total as f64) * 100.0;
    let s_percent: f64 = (output.2 as f64 / total as f64) * 100.0;

    println!("Total: {} cobaias", total);
    println!("Total de coelhos: {}", output.0);
    println!("Total de ratos: {}", output.1);
    println!("Total de sapos: {}", output.2);
    println!("Percentual de coelhos: {:.2} %", c_percent);
    println!("Percentual de ratos: {:.2} %", r_percent);
    println!("Percentual de sapos: {:.2} %", s_percent);

}
