use std::io;
use std::convert::TryInto;

fn main() {
    let mut tests = String::new();
    io::stdin().read_line(&mut tests).expect("Failed to read line!");
    let tests = match tests.trim().parse::<u32>() {
        Ok(x) => x,
        Err(_) => 0,
    };

    for i in 1..=tests {
        let mut game = String::new();
        io::stdin().read_line(&mut game).expect("Failed to read line!");
        let game: [&str; 2] = game.trim().split(' ').collect::<Vec<&str>>().try_into().unwrap();

        let result = if game[0] == game[1] {
            "De novo"
        } else {
            match game {
                ["tesoura", "papel"] => "Bazinga",
                ["papel", "pedra"] => "Bazinga",
                ["pedra", "lagarto"] => "Bazinga",
                ["lagarto", "Spock"] => "Bazinga",
                ["Spock", "tesoura"] => "Bazinga",
                ["tesoura", "lagarto"] => "Bazinga",
                ["lagarto", "papel"] => "Bazinga",
                ["papel", "Spock"] => "Bazinga",
                ["Spock", "pedra"] => "Bazinga",
                ["pedra", "tesoura"] => "Bazinga",
                _ => "Raj trapaceou",
            }
        };

        println!("Caso #{}: {}!", i, result);
    }
}
