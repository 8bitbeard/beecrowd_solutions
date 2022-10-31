use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the line!");
    let amount: usize = match amount.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("Failed to read the test amount");
            return;
        }
    };

    for _i in 1..=amount {
        let mut player_one = String::new();
        io::stdin()
            .read_line(&mut player_one)
            .expect("Failed to read the line!");
        let player_one = player_one.trim();

        let mut player_two = String::new();
        io::stdin()
            .read_line(&mut player_two)
            .expect("Failed to read the line!");
        let player_two = player_two.trim();

        match (player_one, player_two) {
            ("ataque", "ataque") => println!("Aniquilacao mutua"),
            ("ataque", "pedra") => println!("Jogador 1 venceu"),
            ("ataque", "papel") => println!("Jogador 1 venceu"),
            ("pedra", "ataque") => println!("Jogador 2 venceu"),
            ("pedra", "pedra") => println!("Sem ganhador"),
            ("pedra", "papel") => println!("Jogador 1 venceu"),
            ("papel", "ataque") => println!("Jogador 2 venceu"),
            ("papel", "pedra") => println!("Jogador 2 venceu"),
            ("papel", "papel") => println!("Ambos venceram"),
            (_, _) => break,
        }
    }
}
