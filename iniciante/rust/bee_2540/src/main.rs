use std::io;

fn main() {
    loop {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read the line");

        let members: f64 = match data.trim() {
            "" => break,
            x => x.parse::<f64>().unwrap(),
        };

        let mut votes = String::new();
        io::stdin()
            .read_line(&mut votes)
            .expect("Failed to read input");
        let votes: Vec<&str> = votes
            .trim()
            .split(' ')
            .collect();

        let in_favor = votes.iter().filter(|&n| *n == "1").count() as f64;

        println!("{}", in_favor);
        println!("{}", members * 2.0/3.0);

        match in_favor >= (members * 2.0/3.0) {
            true => println!("impeachment"),
            false => println!("acusacao arquivada")
        }
    }
}
