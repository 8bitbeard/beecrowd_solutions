use std::io;

fn main() {

    let mut amount = String::new();

    io::stdin().read_line(&mut amount).expect("Failed to read line!");
    let amount: u32 = amount.trim().parse().unwrap();
    let mut counter: u32 = 0;

    let mut output_vec: Vec<f64> = Vec::new();

    while counter < amount {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let vec: Vec<f64> = input.trim().split(' ').collect::<Vec<&str>>().iter().map(|&x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();

        let bases: [f64; 3] = [2.0, 3.0, 5.0];

        let mut average: f64 = 0.0;
        for (pos, e) in vec.iter().enumerate() {
            average += e * bases[pos];
        }
        average /= 10.0;

        output_vec.push(average);

        counter += 1;
    }

    for number in output_vec {
        println!("{:.1}", number);
    }

}
