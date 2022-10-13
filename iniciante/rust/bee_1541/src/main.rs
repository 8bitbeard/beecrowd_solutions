use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let input = input.trim();

        let data: Vec<u32> = match input {
            "0" => break,
            _ => {
                input.split(' ').map(|x| x.parse::<u32>().unwrap()).collect()
            }
        };

        let x = data[0] * data[1] * 100 / data[2];

        println!("{:.0}", f64::sqrt(x as f64).floor());
    }
}
