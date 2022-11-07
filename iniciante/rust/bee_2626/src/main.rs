use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: Vec<&str> = match input.trim() {
            "" => break,
            x => x.split(' ').collect(),
        };

        let (a, b, c) = (input[0], input[1], input[2]);

        match (a, b, c) {
            ("pedra", "tesoura", "tesoura")
            | ("papel", "pedra", "pedra")
            | ("tesoura", "papel", "papel") => {
                println!("Os atributos dos monstros vao ser inteligencia, sabedoria...")
            }
            ("tesoura", "pedra", "tesoura")
            | ("pedra", "papel", "pedra")
            | ("papel", "tesoura", "papel") => {
                println!("Iron Maiden's gonna get you, no matter how far!")
            }
            ("tesoura", "tesoura", "pedra")
            | ("pedra", "pedra", "papel")
            | ("papel", "papel", "tesoura") => {
                println!("Urano perdeu algo muito precioso...")
            }
            _ => println!("Putz vei, o Leo ta demorando muito pra jogar..."),
        }
    }
}
