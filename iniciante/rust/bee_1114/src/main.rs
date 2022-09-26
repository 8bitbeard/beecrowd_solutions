use std::io;

const PASSWORD: &str = "2002";

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let input = input.trim();

        if input == PASSWORD {
            println!("Acesso Permitido");
            break;
        }

        println!("Senha Invalida");
    }
}
