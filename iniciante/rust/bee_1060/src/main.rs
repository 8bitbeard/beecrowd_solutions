use std::io;


fn main() {

    let mut v: Vec<f64> = Vec::new();
    let mut counter: u8 = 0;

    while counter < 6 {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = input.trim().parse().unwrap();

        v.push(input);

        counter += 1;
    };

    v.retain(|&i| i > 0.0);

    println!("{} valores positivos", v.len());
}
