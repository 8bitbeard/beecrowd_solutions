use std::io;

fn main() {
    let mut counter: u32 = 1;

    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read the tea number!");
        let number: u32 = match number.trim().parse::<u32>() {
            Ok(x) => x,
            Err(_) => break
        };

        let mut numbers: Vec<u32> = vec![0];

        let word = match number {
            0 => "numero",
            _ => {
                for i in 1..=number {
                    numbers.append(&mut vec![i; i as usize]);
                };
                "numeros"
            }
        };

        println!("Caso {}: {} {}", counter, numbers.len(), word);
        println!("{}", numbers.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        println!("");

        counter += 1;
    }
}
