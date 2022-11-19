use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = match input.trim() {
            "" => break,
            x => x,
        };

        println!(
            "{}",
            match input {
                "esquerda" => "ingles",
                "direita" => "frances",
                "nenhuma" => "portugues",
                _ => "caiu",
            }
        );
    }
}
