use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input: Vec<f64> = match input.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<f64>().unwrap()).collect(),
        };

        let (h, m): (f64, f64) = (input[0], input[1]);

        println!("{:02}:{:02}", (h / 30.0).ceil(), (m / 6.0).ceil())
    }
}
